use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::{na::Vector3, prelude::*};
use rand::Rng;

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
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(0., 10., 0.1).looking_at(Vec3::new(0., 0., 0.), Vec3::Y);
    commands.spawn_bundle(camera);

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
            position: [1., 0., 0.].into(),
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
            position: [-1., 0., 0.].into(),
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
            position: [0., 0., 1.].into(),
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
            position: [0., 0., -1.].into(),
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
            shape: ColliderShape::cuboid(0.05, 0.05, 0.05).into(),
            ..ColliderBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: [0., 2., 0.].into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..RigidBodyCcd::default()
            }
            .into(),
            velocity: RigidBodyVelocity {
                // Flatten the linvel so the die doesn't go up or down
                linvel: (random_vector(100.).xz().extend(0.).xzy().normalize() * 100.).into(),
                angvel: random_vector(100.).into(),
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

fn random_vector(length: f32) -> Vec3 {
    Vec3::new(
        rand::thread_rng().gen(),
        rand::thread_rng().gen(),
        rand::thread_rng().gen(),
    )
    .normalize()
        * length
}
