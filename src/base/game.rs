// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    collections::VecDeque,
};

use super::*;


pub struct GameDescription {
    pub content: Vec<Box<dyn ContentPack>>,
    pub adversary: Box<dyn AdversaryDescription>,
    pub spirits: Vec<Box<dyn SpiritDescription>>,
    pub table: Rc<TableDescription>,

    pub fear: Vec<Rc<FearCardDescription>>,
    pub power: Vec<Rc<PowerCardDescription>>,
}

impl GameDescription {
    pub fn new(
        content: Vec<Box<dyn ContentPack>>,
        adversary: Box<dyn AdversaryDescription>,
        spirits: Vec<Box<dyn SpiritDescription>>,
        table: Box<TableDescription>,
    ) -> GameDescription {
        let fear_cards = join_fear_cards(&content);
        let mut power_cards = join_power_cards(&content);

        for (index, spirit) in spirits.iter().enumerate() {
            power_cards.extend(spirit.get_power_cards(index as u8));
        }

        GameDescription {
            content,
            adversary,
            spirits,
            table: Rc::from(table),

            fear: fear_cards.into_iter().map(Rc::from).collect(),
            power: power_cards.into_iter().map(Rc::from).collect(),
        }
    }
}



#[derive(Clone)]
pub struct GameState {
    pub desc: Rc<GameDescription>,

    rng: Box<dyn DeterministicRng>,

    pub step: GameStep,
    pub next_step: GameStep,
    pub game_over_reason: Option<String>,

    pub choices: VecDeque<DecisionChoice>,
    pub effect_stack: Vec<Box<dyn Effect>>,

    pub table: TableState,

    pub invader: InvaderDeck,
    
    pub fear: FearDeck,
    pub fear_pool: u8,
    pub fear_generated: u8,

    pub blight_remaining: u8,

    /*
    fears: SimpleDeck<Box<dyn Fear>>,
    fears_pending: Vec<Box<dyn Fear>>,
    fear_counts: (u8, u8, u8),

    minor_powers: SimpleDeck<Box<dyn Power>>,
    major_powers: SimpleDeck<Box<dyn Power>>,
    */
}

impl GameState {
    pub fn new(desc: Rc<GameDescription>, rng: Box<dyn DeterministicRng>) -> GameState {
        GameState {
            desc: Rc::clone(&desc),
            rng,

            step: GameStep::Init,
            next_step: GameStep::Init,
            game_over_reason: None,

            choices: VecDeque::new(),
            effect_stack: Vec::new(),

            table: TableState::new(desc.table.clone()),

            invader: InvaderDeck::new(),

            fear: FearDeck::new(),
            fear_pool: 0,
            fear_generated: 0,

            blight_remaining: 5,

            /*

            minor_powers: SimpleDeck::new(),
            major_powers: SimpleDeck::new(),
            */
        }
    }

    pub fn is_over(&self) -> bool {
        match self.step {
            GameStep::Victory | GameStep::Defeat => true,
            _ => false,
        }
    }

    pub fn log(&self, s: String) {
        println!("   |{}- {}", "  ".repeat(self.effect_stack.len()), s);
    }
    pub fn log_subeffect(&self, s: String) {
        println!("   |{}- {}", "  ".repeat(self.effect_stack.len() + 1), s);
    }

    pub fn do_effect_box(&mut self, effect: Box<dyn Effect>) -> Result<(), StepFailure> {
        self.effect_stack.push(effect.box_clone());
        let res = effect.apply_effect(self)?;
        self.effect_stack.pop();

        Ok(res)
    }

    pub fn do_effect<T : Effect>(&mut self, effect: T) -> Result<(), StepFailure> {
        self.do_effect_box(effect.box_clone())
    }

    pub fn consume_choice(&mut self) -> Result<DecisionChoice, StepFailure> {
        match self.choices.pop_front() {
            Some(v) => Ok(v),
            None => Err(StepFailure::DecisionRequired)
        }
    }

    pub fn do_defeat(&mut self, defeat_reason: &str) -> Result<(), StepFailure> {
        self.game_over_reason = Some(defeat_reason.to_string());
        self.step = GameStep::Defeat;

        Err(StepFailure::GameOverDefeat)
    }
    
    pub fn do_victory(&mut self, victory_reason: &str) -> Result<(), StepFailure> {
        self.game_over_reason = Some(victory_reason.to_string());
        self.step = GameStep::Victory;

        Err(StepFailure::GameOverVictory)
    }

    pub fn step_to_next_event(&mut self) -> Result<InvaderStep, StepFailure> {
        Ok(self.step_to_next_fear()?)
    }
    pub fn step_to_next_fear(&mut self) -> Result<InvaderStep, StepFailure> {
        let next_card = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::FearEffect(card))) => *card + 1,
            _ => 0,
        };

        if (next_card as usize) < self.fear.pending.len() {
            Ok(InvaderStep::FearEffect(next_card))
        }
        else {
            Ok(self.step_to_next_invader()?)
        }
    }
    pub fn step_to_next_invader(&mut self) -> Result<InvaderStep, StepFailure> {
        let original_next = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::InvaderAction(action, part))) => (*action, *part + 1),
            _ => (0, 0),
        };
        let (mut next_action, mut next_part) = original_next;

        // BaC pg. 14, we go bottom to top
        while next_action < self.invader.step_count() {
            if (next_part as usize) < self.invader.pending.get(next_action as usize).unwrap().len() {
                return Ok(InvaderStep::InvaderAction(next_action, next_part));
            }
            else {
                next_action += 1;
                next_part = 0;
            }
        }

        // last action is a draw! the pending will be empty at first
        // so we backtrack here
        if next_action == self.invader.step_count()
            && 0 == self.invader.pending.back().unwrap().len() {
                
            // TODO: (the draw function must account for additional draws! this only happens once!)
            if self.invader.draw.len() == 0 {
                self.do_defeat("Invader deck empty!")?;
            } else {
                self.invader.draw_into_pending();
            }

            return Ok(InvaderStep::InvaderAction(next_action - 1, 0));
        }

        Ok(InvaderStep::InvaderAdvance)
    }

    pub fn step(&mut self) -> Result<(), StepFailure> {
        let step = self.step;
        println!("---+-{:-^70}-----", format!("-  {}  -", step));

        let desc = self.desc.clone();

        self.next_step = match step {
            GameStep::Init => {
                let invaders = generate_invader_deck();
                self.invader.set_state(invaders, Vec::new(), self.desc.adversary.invader_steps());
                self.invader.shuffle_draw(&mut self.rng.get_rng());

                self.fear.init(&desc.fear, &mut self.rng.get_rng(), desc.adversary.fear_cards());
                self.fear_pool = 4 * desc.spirits.len() as u8;

                desc.adversary.setup(self);

                GameStep::SetupSpirit
            }
            GameStep::SetupSpirit => {
                for (index, spirit) in desc.spirits.iter().enumerate() {
                    self.log(format!("Setting up spirit {} ({})", index, spirit.name()));
                    spirit.do_setup(self, index)?;
                }

                GameStep::SetupExplore
            }
            GameStep::SetupExplore => {
                // The initial explore
                self.invader.draw_into_pending();

                let &card = self.invader.pending.back().unwrap().first().unwrap();
                self.log(format!("Invader Action Card: {}", card));

                let lands = desc.table.lands.iter().filter(|l| card.can_target(l));
                for land in lands {
                    self.do_effect(ExploreEffect { land_index: land.index_on_table })?;
                }

                self.invader.advance();

                // TODO: Post setup-explore adversary setup?

                GameStep::Turn(0, TurnStep::Spirit)
            }
            GameStep::Turn(turn, turn_step) => {
                match &turn_step {
                    TurnStep::Spirit => {

                        GameStep::Turn(turn, TurnStep::FastPower)
                    }
                    TurnStep::FastPower => {

                        GameStep::Turn(turn, TurnStep::Invader(InvaderStep::BlightedIsland))
                    }
                    TurnStep::Invader(inv_step) => {
                        match &inv_step {
                            InvaderStep::BlightedIsland => {

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::Event(event_card, event_part) => {

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::FearEffect(fear_card) => {
                                let card = self.fear.pending[*fear_card as usize].clone();

                                let terror_level = self.fear.terror_level();

                                self.log(format!("Fear Card ({}): {}", terror_level, card.desc.name));

                                self.do_effect_box(
                                    match terror_level {
                                        TerrorLevel::I => card.desc.effect_1.clone(),
                                        TerrorLevel::II => card.desc.effect_2.clone(),
                                        TerrorLevel::III => card.desc.effect_3.clone(),
                                    })?;

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_fear()?))
                            }
                            InvaderStep::InvaderAction(inv_action, inv_card) => {
                                let inv_kind = self.invader.get_step_kind(*inv_action);
                                let &card = self.invader.pending
                                    .get(*inv_action as usize).unwrap()
                                    .get(*inv_card as usize).unwrap();

                                // TODO: Technically the order here is a decision...
                                let lands = desc.table.lands.iter().filter(|l| card.can_target(l));

                                self.log(format!("Invader Action Card: {}", card));
                                match &inv_kind {
                                    InvaderActionKind::Explore => {
                                        for land in lands {
                                            self.do_effect(ExploreEffect { land_index: land.index_on_table })?;
                                        }
                                    }
                                    InvaderActionKind::Build => {
                                        for land in lands {
                                            self.do_effect(BuildEffect { land_index: land.index_on_table })?;
                                        }
                                    }
                                    InvaderActionKind::Ravage => {
                                        for land in lands {
                                            self.do_effect(RavageEffect { land_index: land.index_on_table })?;
                                        }
                                    }
                                }

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_invader()?))
                            }
                            InvaderStep::InvaderAdvance => {
                                self.invader.advance();

                                GameStep::Turn(turn, TurnStep::SlowPower)
                            }
                        }
                    }
                    TurnStep::SlowPower => {

                        GameStep::Turn(turn, TurnStep::TimePasses)
                    }
                    TurnStep::TimePasses => {
                        for land in self.table.lands.iter_mut() {
                            land.time_passes();
                        }

                        GameStep::Turn(turn + 1, TurnStep::Spirit)
                    }
                }
            }
            GameStep::Victory => {

                panic!("Cannot step victory state!");
            }
            GameStep::Defeat => {

                panic!("Cannot step defeat state!");
            }
        };

        if self.choices.len() > 0 {
            return Err(StepFailure::InternalError("There are unconsumed choices!".to_string()));
        }

        Ok(())
    }

    pub fn advance(&mut self) -> Result<(), StepFailure> {
        self.step = self.next_step;

        Ok(())
    }
}
