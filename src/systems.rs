use crate::{
    dice::{DiceResult, DieType, RollDice},
    helpers::random_vector,
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub(crate) fn spawn_die(
    mut commands: Commands,
    mut events: EventReader<RollDice>,
    ass: Res<AssetServer>,
) {
    for command in events.iter() {
        match command.to_dice() {
            Ok(dice) => {
                for die in dice {
                    match die {
                        DieType::D6 => {
                            let mut position: RigidBodyPosition = [0., 2., 0.].into();
                            position.position.rotation =
                                Isometry::rotation(random_vector(PI).into()).rotation;
                            commands
                                .spawn_bundle(ColliderBundle {
                                    shape: ColliderShape::cuboid(0.05, 0.05, 0.05).into(),
                                    ..ColliderBundle::default()
                                })
                                .insert_bundle(RigidBodyBundle {
                                    position: position.into(),
                                    velocity: RigidBodyVelocity {
                                        // Flatten the linvel so the die doesn't go up or down
                                        linvel: (random_vector(10.)
                                            .xz()
                                            .extend(0.)
                                            .xzy()
                                            .normalize()
                                            * 10.)
                                            .into(),
                                        angvel: random_vector(100.).into(),
                                    }
                                    .into(),
                                    ..RigidBodyBundle::default()
                                })
                                .insert_bundle(PbrBundle {
                                    mesh: ass.load("d6.glb#Mesh0/Primitive0"),
                                    material: ass.load("d6.glb#Material0"),
                                    ..PbrBundle::default()
                                })
                                .insert_bundle((RigidBodyPositionSync::Discrete, die))
                        }
                    };
                }
            }
            Err(err) => println!("{err}"),
        }
    }
}

pub(crate) fn dice_counting(
    mut commands: Commands,
    mut events: EventWriter<DiceResult>,
    dice: Query<(
        Entity,
        &DieType,
        &RigidBodyVelocityComponent,
        &RigidBodyPositionComponent,
    )>,
) {
    for (entity, die, velocity, position) in dice.iter() {
        if velocity.is_zero() {
            events.send(DiceResult {
                results: vec![die.get_resting_value(position.position.rotation.euler_angles())],
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}
