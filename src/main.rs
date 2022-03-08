use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(spawn_die)
        .run();
}

fn spawn_die(mut commands: Commands) {
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.05, 0.05, 0.05).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
            ..RigidBodyBundle::default()
        })
        .insert(RigidBodyPositionSync);
}
