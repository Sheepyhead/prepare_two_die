use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_die)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 20., 0.1).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });

    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(1000., 0.1, 1000.).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
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
            position: [15., 0., 0.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
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
            position: [-15., 0., 0.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
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
            position: [0., 0., 8.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
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
            position: [0., 0., -8.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
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

fn spawn_die(mut commands: Commands) {
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [0., 3., 0.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            ColliderDebugRender { color: Color::RED },
            Transform::default(),
            GlobalTransform::default(),
        ));
}
