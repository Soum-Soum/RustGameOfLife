mod game_of_life;

use bevy::prelude::*;
use game_of_life::{Cell, Grid};

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: f32 = 10.0;
const TIME_STEP: f32 = 0.2;
const INIT_POSITIV_PROBA: f64 = 0.2;

#[derive(Resource)]
struct TimeSinceLastUpdate(f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Grid::new(GRID_HEIGHT, GRID_WIDTH))
        .insert_resource(TimeSinceLastUpdate(0.0))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        GRID_WIDTH as f32 * CELL_SIZE,
                        GRID_HEIGHT as f32 * CELL_SIZE,
                    )
                        .into(),
                    title: "Game of Life - Bevy".to_string(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, update_life)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut grid: ResMut<Grid>, mut commands: Commands) {
    commands.spawn(Camera2d::default());

    grid.reset_random(INIT_POSITIV_PROBA);

    draw_lines(commands, grid.into());
}

fn update_life(time: Res<Time>, mut timer: ResMut<TimeSinceLastUpdate>, mut grid: ResMut<Grid>) {
    timer.0 += time.delta_secs();
    if timer.0 < TIME_STEP {
        return;
    }
    timer.0 = 0.0;

    grid.update();
}

fn draw_lines(mut commands: Commands, grid: Res<Grid>) {
    for y in 0..GRID_HEIGHT {
        println!("{} ", y as f32 * CELL_SIZE);
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new((GRID_WIDTH as f32) * CELL_SIZE, 1.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                0.0,
                (y as f32 * CELL_SIZE) - (GRID_HEIGHT as f32 * CELL_SIZE) / 2.0,
                0.0,
            )),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));
    }

    for x in 0..GRID_WIDTH {
        println!("{} ", x as f32 * CELL_SIZE);
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1.0, (GRID_HEIGHT as f32) * CELL_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                (x as f32 * CELL_SIZE) - (GRID_WIDTH as f32 * CELL_SIZE) / 2.0,
                0.0,
                0.0,
            )),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));
    }
}

fn draw_grid(mut commands: Commands, grid: Res<Grid>, query: Query<Entity, With<Cell>>) {
    // Supprimer les anciennes entités (vivantes)
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for (x, y, value) in &grid {
        if value {
            let pos = Vec3::new(
                x as f32 * CELL_SIZE - (GRID_WIDTH as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
                y as f32 * CELL_SIZE - (GRID_HEIGHT as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
                0.0,
            );

            commands.spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..default()
                },
                Transform::from_translation(pos),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Cell { x, y },
            ));
        }
    }
}
