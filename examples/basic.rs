use bevy::{input::keyboard::KeyboardInput, prelude::*};
use prepare_two_die::prelude::*;

/// Basic example - Press space to spawn dice 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DicePlugin)
        .add_system(input)
        .add_system(get_results)
        .run();
}

fn input(mut events: EventWriter<RollDice>, mut input_events: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;

    for ev in input_events.iter() {
        if let KeyboardInput {
            state: ElementState::Pressed,
            key_code: Some(KeyCode::Space),
            ..
        } = ev
        {
            events.send(RollDice::DiceString("1d6".to_string()));
            events.send(RollDice::Dice(vec![DieType::D6]));
        }
    }
}

fn get_results(mut events: EventReader<DiceResult>) {
    for result in events.iter() {
        println!(
            "Rolled {} dice with total {}, results: {:?}",
            result.number_of_dice(),
            result.total(),
            result.results
        );
    }
}
