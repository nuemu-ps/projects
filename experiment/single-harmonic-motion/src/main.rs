use physics::classical_mechanics::point_mass::{PointMass, Position, Force};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Body(PointMass);

const DELTA: f64 = 0.001;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_circle)
        .add_systems(Update, (update_body, update_circle).chain())
        .run();
}

fn add_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let body = PointMass::new(
        Some(1.0),
        Some(Position {
            x: 200.0,
            y: 0.0,
            z: 0.0
        }),
        None,
        None
    );

    commands.spawn((Body(body), MaterialMesh2dBundle {
        mesh: meshes.add(Circle { radius: 10.0 }).into(),
        transform: Transform::from_xyz(200., 0., 0.),
        material: materials.add(Color::RED),
        ..default()
    }));
}

fn update_body(mut query: Query<&mut Body>) {
    for mut body in &mut query {
        for _ in 0..10 {
            let x = body.0.position.x;

            body.0.add_force(Force {
                x: - 1024.0 * x,
                y: 0.0,
                z: 0.0
            });
            body.0.velocity = body.0.velocity_after(DELTA);
            body.0.position = body.0.position_after(DELTA);
        }
    }
}

fn update_circle(mut circle_position: Query<&mut Transform, With<Body>>, query: Query<&mut Body>) {
    for mut transform in &mut circle_position {
        transform.translation.x = query.single().0.position.x as f32;
    }
}
