use bevy::render::camera::Projection;
use bevy::window::CursorGrabMode;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, PrimaryWindow},
};
use bevy_fly_camera::*;
use bevy_framepace::*;
use bevy_mod_wanderlust::{
    Controller, ControllerBundle, ControllerInput, ControllerPhysicsBundle, Float, Freeze,
    GroundCaster, Jump, Movement, RapierPhysicsBundle, Strength, Upright, WanderlustPlugin,
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::{FRAC_2_PI, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlyCameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 0.016,
                substeps: 32,
            },
            ..default()
        })
        .add_systems(Startup, (ground, walls, lights, camera))
        .add_systems(Update, (raycast))
        .run()
}

pub fn camera(mut commands: Commands) {
    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 3.0),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 90.0 * (PI / 180.0),
                aspect_ratio: 1.0,
                near: 0.3,
                far: 1000.0,
            }),
            ..default()
        },))
        .insert(FlyCamera {
            sensitivity: 10.0,
            ..default()
        });
}

pub fn ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let material = mats.add(Color::WHITE.into());

    let size = 1000.0;
    let mesh = meshes.add(
        shape::Plane {
            size: size,
            ..default()
        }
        .into(),
    );

    commands.spawn((
        PbrBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
        Collider::halfspace(Vec3::Y).unwrap(),
        ColliderDebugColor(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }),
        //Collider::cuboid(size / 2.0, 0.1, size / 2.0),
        Name::from("Ground"),
    ));
}

fn lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform {
            rotation: Quat::from_rotation_z(35.0 * PI / 180.0)
                * Quat::from_rotation_y(35.0 * PI / 180.0),
            ..default()
        },
        ..default()
    });
}

pub fn walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let materials = [Color::GRAY, Color::WHITE, Color::BLACK];
    let materials = materials
        .iter()
        .map(|color| {
            mats.add(StandardMaterial {
                base_color: *color,
                perceptual_roughness: 0.5,
                reflectance: 0.05,
                ..default()
            })
        })
        .collect::<Vec<_>>();

    let wall = commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(-5.0, 0.0, -5.0),
                rotation: Quat::from_rotation_y(-PI / 4.0),
                ..default()
            },
            ..default()
        })
        .id();

    let parts = 20;
    let width = 0.25;
    for part in 0..=parts {
        let material = materials[part % materials.len()].clone();
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: material,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, part as f32 * width),
                        scale: Vec3::new(1.0, 40.0, width),
                        ..default()
                    },
                    ..default()
                },
                Name::from("Wall segment"),
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .set_parent(wall);
    }

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials[0].clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, ((parts + 1) as f32 * width) * 1.5),
                    scale: Vec3::new(1.0, 40.0, width * parts as f32),
                    ..default()
                },
                ..default()
            },
            Name::from("Full wall segment"),
            Collider::cuboid(0.5, 0.5, 0.5),
        ))
        .set_parent(wall);
}

pub fn raycast(ctx: Res<RapierContext>, mut gizmos: Gizmos) {
    let pos = Vec3::new(-5.385965, 2.2417314, -3.4730017);
    let direct_ray = Vec3::new(-0.5011464, -0.7491554, -0.43314943);
    let length = 1.5;

    let direct_ray = direct_ray.normalize_or_zero();

    gizmos.ray(pos, direct_ray * length, Color::BLUE);
    gizmos.sphere(pos, Quat::IDENTITY, 0.1, Color::BLUE);
    if let Some((ray_entity, inter)) = ctx.cast_ray_and_get_normal(
        pos,
        direct_ray,
        direct_ray.length() + 0.5,
        true,
        QueryFilter::default(),
    ) {
        info!("toi: {:?}", inter.toi);
        info!("normal: {:?}", inter.normal);
        gizmos.ray(inter.point, inter.normal, Color::RED);
    }
}
