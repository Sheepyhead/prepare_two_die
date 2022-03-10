use bevy::{input::keyboard::KeyboardInput, math::Vec3Swizzles, prelude::*};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::{
    na::{ComplexField, Vector3},
    prelude::*,
};
use rand::Rng;
use std::{f32::consts::PI, fmt};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.,
        })
        .add_plugins(DefaultPlugins)
        .add_event::<RollDice>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_system(spawn_die)
        .add_system(input)
        .add_system(dice_counting)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(0., 10., 0.1).looking_at(Vec3::new(0., 0., 0.), Vec3::Y);
    commands.spawn_bundle(camera);

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1000., 0.1, 1000.).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Transform::default(),
            GlobalTransform::default(),
            ColliderDebugRender {
                color: Color::WHITE,
            },
        ));
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.1, 1000., 1000.).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [1., 0., 0.].into(),
            body_type: RigidBodyType::Static.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Transform::default(),
            GlobalTransform::default(),
            ColliderDebugRender {
                color: Color::PURPLE,
            },
        ));
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.1, 1000., 1000.).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [-1., 0., 0.].into(),
            body_type: RigidBodyType::Static.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Transform::default(),
            GlobalTransform::default(),
            ColliderDebugRender {
                color: Color::PURPLE,
            },
        ));
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1000., 1000., 0.1).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [0., 0., 1.].into(),
            body_type: RigidBodyType::Static.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Transform::default(),
            GlobalTransform::default(),
            ColliderDebugRender {
                color: Color::PURPLE,
            },
        ));
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1000., 1000., 0.1).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [0., 0., -1.].into(),
            body_type: RigidBodyType::Static.into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Transform::default(),
            GlobalTransform::default(),
            ColliderDebugRender {
                color: Color::PURPLE,
            },
        ));
}

fn spawn_die(mut commands: Commands, mut events: EventReader<RollDice>, ass: Res<AssetServer>) {
    for RollDice(dice) in events.iter() {
        for die in dice {
            match die {
                DieType::D6 => {
                    let mut position: RigidBodyPosition = [0., 2., 0.].into();
                    position.position.rotation = Isometry::rotation(random_vector(PI).into()).rotation;
                    commands
                        .spawn_bundle(ColliderBundle {
                            shape: ColliderShape::cuboid(0.05, 0.05, 0.05).into(),
                            ..ColliderBundle::default()
                        })
                        .insert_bundle(RigidBodyBundle {
                            position: position.into(),
                            velocity: RigidBodyVelocity {
                                // Flatten the linvel so the die doesn't go up or down
                                linvel: (random_vector(10.).xz().extend(0.).xzy().normalize()
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
                        .insert_bundle((RigidBodyPositionSync::Discrete, *die))
                }
            };
        }
    }
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
            events.send(RollDice(vec![DieType::D6]));
        }
    }
}

fn random_vector(length: f32) -> Vec3 {
    use rand::thread_rng;

    let phi = thread_rng().gen_range(0.0..=PI * 2.0);
    let cos_theta = thread_rng().gen_range(-1.0..=1.0);

    let theta = cos_theta.acos();

    Vec3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    )
    .normalize()
        * length
}

pub struct RollDice(Vec<DieType>);

#[derive(Clone, Copy, Component)]
pub enum DieType {
    D6,
}

impl fmt::Display for DieType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            DieType::D6 => "d6",
        })
    }
}

impl DieType {
    fn get_resting_value(&self, (x, _, z): (f32, f32, f32)) -> u32 {
        match self {
            DieType::D6 => {
                // Represents how many different sides you can turn this die across one axis
                enum SideTurns {
                    None,
                    One,
                    Two,
                    Three,
                }
                impl fmt::Display for SideTurns {
                    fn fmt(
                        &self,
                        f: &mut std::fmt::Formatter<'_>,
                    ) -> std::result::Result<(), std::fmt::Error> {
                        f.write_str(match self {
                            SideTurns::None => "None",
                            SideTurns::One => "One",
                            SideTurns::Two => "Two",
                            SideTurns::Three => "Three",
                        })
                    }
                }
                let x_turns = match () {
                    () if x < ((-3.1 + -1.6) / 2.0) => SideTurns::Two,
                    () if x < ((-1.6 + 0.0) / 2.0) => SideTurns::Three,
                    () if x < ((0.0 + 1.6) / 2.0) => SideTurns::None,
                    () if x < ((1.6 + 3.1) / 2.0) => SideTurns::One,
                    () => SideTurns::Two,
                };
                let z_turns = match () {
                    () if z < ((-3.1 + -1.6) / 2.0) => SideTurns::Two,
                    () if z < ((-1.6 + 0.0) / 2.0) => SideTurns::Three,
                    () if z < ((0.0 + 1.6) / 2.0) => SideTurns::None,
                    () if z < ((1.6 + 3.1) / 2.0) => SideTurns::One,
                    () => SideTurns::Two,
                };

                match (x_turns, z_turns) {
                    (SideTurns::None, SideTurns::One)
                    | (SideTurns::One, SideTurns::One)
                    | (SideTurns::Two, SideTurns::One)
                    | (SideTurns::Three, SideTurns::One) => 1,
                    (SideTurns::None, SideTurns::None) | (SideTurns::Two, SideTurns::Two) => 2,
                    (SideTurns::One, SideTurns::None) | (SideTurns::Three, SideTurns::Two) => 3,
                    (SideTurns::One, SideTurns::Two) | (SideTurns::Three, SideTurns::None) => 4,
                    (SideTurns::None, SideTurns::Two) | (SideTurns::Two, SideTurns::None) => 5,
                    (SideTurns::None, SideTurns::Three)
                    | (SideTurns::One, SideTurns::Three)
                    | (SideTurns::Two, SideTurns::Three)
                    | (SideTurns::Three, SideTurns::Three) => 6,
                }
            }
        }
    }
}

fn dice_counting(
    mut commands: Commands,
    dice: Query<(
        Entity,
        &DieType,
        &RigidBodyVelocityComponent,
        &RigidBodyPositionComponent,
    )>,
) {
    for (entity, die, velocity, position) in dice.iter() {
        if velocity.is_zero() {
            println!(
                "{die} landed on value {}",
                die.get_resting_value(position.position.rotation.euler_angles())
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}
