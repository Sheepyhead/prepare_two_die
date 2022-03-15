# Bevy Match 3

[![crates.io](https://img.shields.io/crates/v/prepare_two_die)](https://crates.io/crates/prepare_two_die)
[![docs.rs](https://docs.rs/prepare_two_die/badge.svg)](https://docs.rs/prepare_two_die)
[![Crates.io](https://img.shields.io/crates/d/prepare_two_die.svg)](https://crates.io/crates/prepare_two_die)


<img src="example.gif" width="300" height="300" />

`prepare_two_die` is a physics-based dice rolling crate for the game engine [Bevy](https://bevyengine.org/).

## Bevy Version Support
| `bevy` | `bevy_match3` |
| ------ | ------------- |
| 0.6    | 0.0.1           |

## Features
- Event-based dice rolling commands
- Visual dice model examples included in assets, feel free to use them or make dice based on them

## Immediate todo
- [ ] Implement more dice
  - [ ] d2 aka the coin die
  - [ ] d4
  - [ ] d8
  - [ ] d10
  - [ ] d100 as a special case of d10s
  - [ ] d12
  - [ ] d20
- [ ] Implement dice result handle so you can map a command to its corresponding result
- [ ] Implement configurable positioning and dice roll bounds

## Examples
To get started with this crate all you need is to set up the plugin
```rust
use bevy::prelude::*;
use prepare_two_die::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DicePlugin)
        .run();
}
```
and send commands! 
```rust
fn send_command(mut events: EventWriter<RollDice>) {
    events.send(RollDice::DiceString("3d6".to_string()));
    events.send(RollDice::Dice(vec![DieType::D6; 3]));
}

```
And receive the result events!

For now there is one example of all features at [`basic.rs`](examples/basic.rs)

## License
Note that this project is licensed under the [`Anti-Capitalist Software License`](https://anticapitalist.software/). If this proves a major obstacle for adoption I may consider a more conventional license, I would just like to avoid this crate being flipped by the likes of King and similar.
