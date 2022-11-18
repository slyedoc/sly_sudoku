mod board;
mod lens;
mod menu;
mod state;
mod ui;
mod events;

#[cfg(not(target_os = "android"))]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use board::*;
use events::EventPlugin;
use lens::*;
use menu::*;
use state::*;
use ui::*;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use sudoku_variants::{constraint::DefaultConstraint, Sudoku};

#[derive(Deref, DerefMut, Resource)]
pub struct SudokuContainer(pub Sudoku<DefaultConstraint>);

impl Default for SudokuContainer {
    fn default() -> Self {
        Self(Sudoku::new_empty(3, 3, DefaultConstraint).unwrap())
    }
}

#[derive(Component)]
pub struct Keep;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    Playing,
}

#[bevy_main]
pub fn main() {
    let mut app = App::new();

    // #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    // app.insert_resource(WinitSettings::desktop_app());
    // #[cfg(target_os = "android")]
    // app.insert_resource(WgpuSettings {
    //     priority: WgpuSettingsPriority::Compatibility,
    //     ..default()
    // });
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Sudoku".to_string(),
            ..default()
        },
        ..default()
    }));
    #[cfg(not(target_os = "android"))]
    app.add_plugin(WorldInspectorPlugin::default());
    app.add_plugin(TweeningPlugin)
        // grid
        //.add_plugin(bevy_infinite_grid::InfiniteGridPlugin)
        //.add_startup_system(spawn_grid)
        // Setup Resources
        .add_state(AppState::Loading)
        .init_resource::<SudokuContainer>()
        // Local Plugins
        .add_plugin(UIPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LensPlugin)
        .add_plugin(StatePlugin)
        .add_plugin(EventPlugin)

        // global setup
        .add_startup_system(setup_camera)
        
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                // Using custom clear color so it can be tweened
                clear_color: ClearColorConfig::Custom(theme.background),
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Keep,
    ));
}

fn cleanup(mut commands: Commands, q: Query<Entity, Without<Keep>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}


