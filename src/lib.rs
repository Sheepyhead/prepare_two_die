use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dice::{DiceResult, RollDice};
use systems::{dice_counting, spawn_die};

mod dice;
mod helpers;
mod systems;

pub mod prelude {
    pub use crate::dice::*;
    pub use crate::DicePlugin;
}

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.,
        })
        .add_event::<RollDice>()
        .add_event::<DiceResult>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_system(spawn_die)
        .add_system(dice_counting)
        .run();
    }
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
        ));
}
