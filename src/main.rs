
use bevy::prelude::*;


#[derive(Component, Debug)]
struct Position(Vec3);

#[derive(Component, Debug)]
struct Rotation(Quat);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup_utils,
            setup_player,
        ))
        .add_systems(Update, ( 
            move_model,
            update_model_pos,
        ))
        .run();
}

/// set up a simple 3D scene
fn setup_utils(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Rotation(Quat::from_rotation_y(0.0)),
        Position(Vec3::new(0.0, 0.5, 0.0)),
        PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }));
}

fn move_model(time: Res<Time>,keys: Res<Input<KeyCode>>, mut query: Query<(&mut Position, &mut Rotation)>) {
    for (mut position,mut rotation) in query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            position.0.z -= 1.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            position.0.z += 1.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            rotation.0 = Quat::from_rotation_y(0.5 * time.delta_seconds());
        }
        if keys.pressed(KeyCode::D) {
            rotation.0 = Quat::from_rotation_y(-0.5 * time.delta_seconds());
        }
        if keys.just_released(KeyCode::A) || keys.just_released(KeyCode::D) {
            rotation.0 = Quat::from_rotation_y(0.0);
        }
    }
}
fn update_model_pos(mut query: Query<(&Position, &Rotation, &mut Transform)>) {
    for (position, rotation, mut transform) in query.iter_mut() {
        transform.translation = position.0;
        transform.rotate_y(rotation.0.y);
    }
}