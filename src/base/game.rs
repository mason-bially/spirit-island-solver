// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    fmt,
    sync::{Arc},
    clone::Clone,
    collections::VecDeque,
};

use super::*;


pub struct GameDescription {
    pub content: Vec<Box<dyn ContentPack>>,
    pub adversary: Box<dyn AdversaryDescription>,
    pub spirits: Vec<Arc<Box<dyn SpiritDescription>>>,
    pub table: Arc<TableDescription>,

    pub fear: Vec<Arc<FearCardDescription>>,
    pub powers: Vec<Arc<PowerCardDescription>>,
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
            spirits: spirits.into_iter().map(Arc::new).collect(),
            table: Arc::from(table),

            fear: fear_cards.into_iter().map(Arc::from).collect(),
            powers: power_cards.into_iter().map(Arc::from).collect(),
        }
    }
}



#[derive(Clone)]
pub struct GameState {
    pub desc: Arc<GameDescription>,

    pub rng: Box<dyn DeterministicRng>,

    pub enable_logging: bool,

    pub step: GameStep,
    pub next_step: GameStep,
    pub game_over_reason: Option<String>,
    pub choice_count: u32,

    pub choices: VecDeque<DecisionChoice>,
    pub effect_stack: Vec<Box<dyn Effect>>,
    pub power_usages: Vec<PowerUsage>,

    pub table: TableState,

    pub invader: InvaderDeck,
    
    pub fear: FearDeck,
    pub fear_pool: u8,
    pub fear_generated: u8,
    pub fear_generated_total: u8,

    pub blight_remaining: u8,

    pub spirits: Vec<SpiritState>,

    pub minor_powers: PowerDeck,
    pub major_powers: PowerDeck,

}

impl GameState {
    pub fn new(desc: Arc<GameDescription>, rng: Box<dyn DeterministicRng>) -> GameState {
        GameState {
            desc: Arc::clone(&desc),
            rng,

            enable_logging: false,

            step: GameStep::Init,
            next_step: GameStep::Init,
            game_over_reason: None,
            choice_count: 0,

            choices: VecDeque::new(),
            effect_stack: Vec::new(),
            power_usages: Vec::new(),

            table: TableState::new(desc.table.clone()),

            invader: InvaderDeck::new(),

            fear: FearDeck::new(),
            fear_pool: 0,
            fear_generated: 0,
            fear_generated_total: 0,

            blight_remaining: 5,

            spirits: Vec::new(),

            minor_powers: PowerDeck::new(),
            major_powers: PowerDeck::new(),
        }
    }

    pub fn is_over(&self) -> bool {
        match self.step {
            GameStep::Victory | GameStep::Defeat => true,
            _ => false,
        }
    }

    pub fn log_effect(&self, args: fmt::Arguments) {
        if self.enable_logging {
            println!("   |{}- {}", "  ".repeat(self.effect_stack.len()), args);
        }
    }
    pub fn log_decision(&self, args: fmt::Arguments) {
        if self.enable_logging {
            println!("   |{}{}) {}", "  ".repeat(self.effect_stack.len()), self.choice_count, args);
        }
    }
    pub fn log_subeffect(&self, args: fmt::Arguments) {
        if self.enable_logging {
            println!("   |{}- {}", "  ".repeat(self.effect_stack.len() + 1), args);
        }
    }


    pub fn get_land(&self, land_index: u8) -> Result<&LandState, StepFailure> {
        self.table.lands
            .get(land_index as usize)
            .ok_or(StepFailure::InternalError("index out of range".to_string()))
    }
    pub fn get_land_mut(&mut self, land_index: u8) -> Result<&mut LandState, StepFailure> {
        self.table.lands
            .get_mut(land_index as usize)
            .ok_or(StepFailure::InternalError("index out of range".to_string()))
    }
    pub fn get_land_desc(&self, land_index: u8) -> Result<Arc<LandDescription>, StepFailure> {
        Ok(Arc::clone(
            self.desc.table.lands
                .get(land_index as usize)
                .ok_or(StepFailure::InternalError("index out of range".to_string()))?
            ))
    }
    pub fn get_adjacent_lands(&self, land_index: u8) -> Result<Vec<&LandState>, StepFailure> {
        let adjacent_indexes = self.get_land_desc(land_index)?.adjacent.clone();

        Ok(adjacent_indexes.into_iter().map(|i| self.get_land(i).ok().unwrap()).collect())
    }
    pub fn get_adjacent_lands_desc(&self, land_index: u8) -> Result<Vec<Arc<LandDescription>>, StepFailure> {
        Ok(self.desc.table
            .get_adjacent_lands(land_index))
    }

    pub fn get_spirit(&self, spirit_index: u8) -> Result<&SpiritState, StepFailure> {
        self.spirits.get(spirit_index as usize)
            .ok_or(StepFailure::InternalError("index out of range".to_string()))
    }
    pub fn get_spirit_mut(&mut self, spirit_index: u8) -> Result<&mut SpiritState, StepFailure> {
        self.spirits.get_mut(spirit_index as usize)
            .ok_or(StepFailure::InternalError("index out of range".to_string()))
    }
    pub fn get_spirit_desc(&self, spirit_index: u8) -> Result<Arc<Box<dyn SpiritDescription>>, StepFailure> {
        Ok(Arc::clone(
            self.desc.spirits
                .get(spirit_index as usize)
                .ok_or(StepFailure::InternalError("index out of range".to_string()))?
        ))
    }

    pub fn get_power_usage(&self) -> Result<&PowerUsage, StepFailure> {
        self.power_usages.last()
            .ok_or(StepFailure::InternalError("no current power usage".to_string()))
    }


    pub fn consume_choice(&mut self) -> Result<DecisionChoice, StepFailure> {
        match self.choices.pop_front() {
            Some(v) => {
                self.choice_count += 1;
                Ok(v)
            },
            None => Err(StepFailure::DecisionRequired)
        }
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

    pub fn score_game(&self) -> i16 {
        // TODO: calculate this
        let difficulty: i16 = 0;

        let mut score = 0;

        if self.step == GameStep::Victory {
            score += 5 * difficulty;
            score += 10;
            score += 2 * self.invader.draw.len() as i16;
        } else {
            score += 2 * difficulty;

            let invader_cards_not_in_deck: i16
                = (self.invader.pending.iter().map(|x| x.len()).sum::<usize>() + self.invader.discard.len()) as i16;
            score += 1 * invader_cards_not_in_deck;
        }

        let players: i16 = self.spirits.len() as i16;
        let living_dahan: i16 = self.table.lands.iter().map(|l| l.dahan.len()).sum::<usize>() as i16;
        score += 1 * (living_dahan / players);
        let blight: i16 = self.table.lands.iter().map(|l| l.tokens[TokenKind::Blight]).sum::<u8>() as i16;
        score -= 1 * (blight / players);

        score
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
        if self.enable_logging {
            println!("---+-{:-^70}-----", format!("-  {}  -", step));
        }

        let desc = self.desc.clone();

        self.next_step = match step {
            GameStep::Init => {
                let invaders = generate_invader_deck();
                self.invader.set_state(invaders, Vec::new(), self.desc.adversary.invader_steps());
                self.invader.shuffle_draw(&mut self.rng.get_rng());

                self.fear.init(&desc.fear, &mut self.rng.get_rng(), desc.adversary.fear_cards());
                self.fear_pool = 4 * desc.spirits.len() as u8;

                desc.adversary.setup(self);

                for spirit_desc in desc.spirits.iter() {
                    self.spirits.push(SpiritState::new(spirit_desc));
                }

                self.minor_powers.init(
                    self.desc.powers.iter()
                        .filter(|pcd| pcd.kind == PowerCardKind::Minor)
                        .map(|pcd| Arc::clone(pcd))
                        .collect(),
                    &mut self.rng.get_rng());
                self.major_powers.init(
                    self.desc.powers.iter()
                        .filter(|pcd| pcd.kind == PowerCardKind::Major)
                        .map(|pcd| Arc::clone(pcd))
                        .collect(),
                    &mut self.rng.get_rng());

                GameStep::SetupSpirit
            }
            GameStep::SetupSpirit => {
                for (index, spirit_desc) in desc.spirits.iter().enumerate() {
                    let spirit_index = index as u8;
                    self.log_effect(format_args!("Setting up spirit {} ({})", index, spirit_desc.name()));
                    
                    let powers = self.desc.powers.iter()
                        .filter(|pcd| pcd.kind == PowerCardKind::Spirit(spirit_index))
                        .map(|pcd| Arc::clone(pcd))
                        .collect();

                    spirit_desc.do_setup(self, index)?;

                    let spirit_mut = self.get_spirit_mut(spirit_index)?;

                    spirit_mut.deck.init(powers);
                }

                GameStep::SetupExplore
            }
            GameStep::SetupExplore => {
                // The initial explore
                self.invader.draw_into_pending();

                let &card = self.invader.pending.back().unwrap().first().unwrap();
                self.log_effect(format_args!("Invader Action Card: {}", card));

                let lands = desc.table.lands.iter().filter(|l| card.can_target(l));
                for land in lands {
                    self.do_effect(ExploreEffect { land_index: land.index_on_table })?;
                }

                self.invader.advance();

                // TODO: Post setup-explore adversary setup?

                GameStep::Turn(0, TurnStep::Spirit(SpiritStep::Growth))
            }
            GameStep::Turn(turn, turn_step) => {
                match &turn_step {
                    TurnStep::Spirit(spirit_step) => {
                        match &spirit_step {
                            SpiritStep::Growth => {
                                for (index, spirit_desc) in desc.spirits.iter().enumerate() {
                                    spirit_desc.do_growth(self, index)?;
                                }

                                GameStep::Turn(turn, TurnStep::Spirit(SpiritStep::Income))
                            }
                            SpiritStep::Income => {
                                for (index, spirit_desc) in desc.spirits.iter().enumerate() {
                                    spirit_desc.do_income(self, index)?;
                                }

                                GameStep::Turn(turn, TurnStep::Spirit(SpiritStep::Play))
                            }
                            SpiritStep::Play => {
                                for index in 0..desc.spirits.len() {
                                    self.do_effect(CardPlaysDecision{spirit_index: index as u8})?;
                                }

                                for (index, spirit_desc) in desc.spirits.iter().enumerate() {
                                    self.log_effect(format_args!("State of {}:", spirit_desc.name()));
                                    let spirit = self.get_spirit(index as u8)?;
                                    for card in spirit.deck.hand.iter() {
                                        self.log_effect(format_args!(" $  |{}|", card.desc));
                                    }
                                    for card in spirit.deck.pending.iter() {
                                        self.log_effect(format_args!(">>> |{}|", card.desc));
                                    }
                                }

                                GameStep::Turn(turn, TurnStep::FastPower)
                            }
                        }
                    }
                    TurnStep::FastPower => {
                        self.do_effect(DoCardPlaysDecision{power_speed: PowerSpeed::Fast})?;

                        GameStep::Turn(turn, TurnStep::Invader(InvaderStep::BlightedIsland))
                    }
                    TurnStep::Invader(inv_step) => {
                        match &inv_step {
                            InvaderStep::BlightedIsland => {
                                // TODO: blight
                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::Event(_event_card, _event_part) => {
                                // TODO: events
                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::FearEffect(fear_card) => {
                                let card = self.fear.pending[*fear_card as usize].clone();

                                let terror_level = self.fear.terror_level();

                                self.log_effect(format_args!("Fear Card ({}): {}", terror_level, card.desc.name));

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
                                // But which lands are targeted is *not*
                                let lands = desc.table.lands.iter().filter(|l| card.can_target(l));

                                self.log_effect(format_args!("Invader Action Card: {}", card));
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
                        self.do_effect(DoCardPlaysDecision{power_speed: PowerSpeed::Slow})?;

                        GameStep::Turn(turn, TurnStep::TimePasses)
                    }
                    TurnStep::TimePasses => {
                        for land in self.table.lands.iter_mut() {
                            land.time_passes();
                        }

                        for spirit in self.spirits.iter_mut() {
                            spirit.time_passes();
                        }

                        GameStep::Turn(turn + 1, TurnStep::Spirit(SpiritStep::Growth))
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
