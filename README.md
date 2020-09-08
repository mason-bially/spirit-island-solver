
## Licensing

(Eric: I agree to the terms for creating Spirit Island game elements [set forth in the FAQ](https://querki.net/u/darker/spirit-island-faq/#!.9v5ka4u))

All game assets (often rendered in code form) are owned by Greater Than Games. Other code, as and where marked, is licensed under the MPL, copyright Mason Bially. This code base is intended for personal and educational use only. I do not intend to use this code commercially - nor do I intend for others to use it commercially - due to the copyrighted elements and the [position of the publisher](https://querki.net/u/darker/spirit-island-faq/#!.9v5ka4u).

## Usage

An example command to run is:

```
cargo run --release -- -s river --seed foobar --solver rng --solver-take 2 -j10
```

## AI Approaches

This library is designed to support three different ways of approaching writing an AI for or solver for spirit island.

* Known Information: Given only known information, choose the best solution. This will only know what a hypothetical player playing the board game would know. From here we can use heuristics, expert advice systems, stochastic searches, or total searches memoization and card counting. This allows us to implement an AI "assistant" that will solve the board given what it is told.
* Fate Choosing: At every step of the way the AI will make the decisions that would otherwise be left up to chance. This is the opposite of the above. Every card draw, every invader action, every piece of information will be chosen by the AI to maximize its chances of winning. From here we would mainly be limited to total searches using aggressive state space sharing, relying on heuristics to bound or task planners to accelerate the process.
* Classic Cheating: This AI is playing a random game, but it can see the future of its actions. A middle ground of both the above, and a useful starting place as it reduces choices compared to the Fate Choosing solution, while reducing search space or usage of stochastic methods from the Known Information solution. From here we only have to search for the actions the spirits are taking and may even be able to plan out the whole game ahead of time. This would be best used for a computer game AI so that it could make perfect choices and rank its potential choices from the options it has.

The implementation of these different approaches is shared. The classic cheating solution is also a simulation of the known information situation, and the fate choosing solution is non-stochastic variation of the known information solution.
