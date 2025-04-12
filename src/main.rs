mod game_of_life;

use bevy::{prelude::*, window::PrimaryWindow};
use chrono::TimeDelta;
use game_of_life::{clock::SimpleClock, grid::{Cell, Grid}};

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 25;
const CELL_SIZE: f32 = 16.0;
const TIME_STEP: TimeDelta = TimeDelta::new(0, (0.05 * 1e9) as u32).unwrap();
const INIT_POSITIV_PROBA: f64 = 0.5;

const BACKGROUND_COLOR: Color = Color::srgb(15.0 / 255.0, 15.0 / 255.0, 15.0 / 255.0);
const CELL_COLOR: Color = Color::srgb(236.0 / 255.0, 240.0 / 255.0, 241.0 / 255.0);
const LINE_COLOR: Color = Color::srgb(72.0 / 255.0, 84.0 / 255.0, 96.0 / 255.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Grid::new(GRID_HEIGHT, GRID_WIDTH))
        .insert_resource(SimpleClock::new(TIME_STEP))
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
        .add_systems(Update, handle_keyboard_inputs)
        .add_systems(Update, handle_mouse_inputs)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut grid: ResMut<Grid>, mut commands: Commands) {
    commands.spawn(Camera2d::default());

    grid.reset_random(INIT_POSITIV_PROBA);

    draw_lines(commands, grid.into());
}

fn update_life(mut clock: ResMut<SimpleClock>, mut grid: ResMut<Grid>) {
    if !clock.should_update() {
        return;
    }
    grid.update();
}

fn handle_keyboard_inputs(
    mut grid: ResMut<Grid>,
    mut simple_clock: ResMut<SimpleClock>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        simple_clock.toggle_pause();
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        grid.reset_random(INIT_POSITIV_PROBA);
    }
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        grid.clear();
    }
}

fn handle_mouse_inputs(
    mut grid: ResMut<Grid>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let x = (position.x / CELL_SIZE) as usize;
            let y = (position.y / CELL_SIZE) as usize;
            grid.toggle_value(x, y);
        }
    }
}

fn draw_lines(mut commands: Commands, grid: Res<Grid>) {
    for y in 0..GRID_HEIGHT {
        commands.spawn((
            Sprite {
                color: LINE_COLOR,
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
        commands.spawn((
            Sprite {
                color: LINE_COLOR,
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

fn draw_grid(
    mut commands: Commands,
    grid: Res<Grid>,
    query: Query<Entity, With<Cell>>,
    asset_server: Res<AssetServer>,
) {
    // Supprimer les anciennes entités (vivantes)
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for (x, y, value) in &grid {
        if value {
            let pos = Vec3::new(
                x as f32 * CELL_SIZE - (GRID_WIDTH as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0,
                -(y as f32 * CELL_SIZE - (GRID_HEIGHT as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0),
                0.0,
            );

            commands.spawn((
                Sprite::from_image(asset_server.load("cell16x16.png")),
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
