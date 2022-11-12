mod board;
mod config;
mod ui;
mod menu;
mod lens;

use bevy_tweening::TweeningPlugin;
use menu::*;
use board::*;
use config::*;
use ui::*;
use lens::*;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use bevy_inspector_egui::{WorldInspectorPlugin};
use iyes_loopless::prelude::*;
use sudoku_variants::{Sudoku, constraint::DefaultConstraint};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    #[allow(dead_code)]
    Start,
    #[allow(dead_code)]
    Playing,
    #[allow(dead_code)]
    Over,
}

#[derive(Deref, DerefMut)]
pub struct SukokuState(pub Sudoku<DefaultConstraint>);

impl Default for SukokuState {
    fn default() -> Self {
        Self(Sudoku::new_empty(3,3, DefaultConstraint).unwrap())
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sudoku".to_string(),
            ..default()
        })
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())        
        .add_plugin(TweeningPlugin)
        // grid
        //.add_plugin(bevy_infinite_grid::InfiniteGridPlugin)
        //.add_startup_system(spawn_grid)
        // Setup Resources
        .init_resource::<SukokuState>()
        
        // Local Plugins
        .add_plugin(UIPlugin)
        .add_plugin(ConfigPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LensPlugin)
        .add_loopless_state(GameState::Loading)
        // global setup
        .add_startup_system(setup_camera)
        .add_startup_system(setup_layout)        
        .add_startup_system(startup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(
    mut commands: Commands,
    theme: Res<Theme>,
) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            // Using custom clear color so it can be tweened
            clear_color: ClearColorConfig::Custom(theme.background),
        },
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn startup(
    mut new_game_events: EventWriter<NewGame>,
) {
    new_game_events.send(NewGame);
}

fn setup_layout(
    mut commands: Commands,
    //config: Res<SudokuConfig>,
    font_assets: Res<FontAssets>,
    theme: Res<Theme>,
) {    
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Layout"))
        .with_children(|parent| {
            create_board(parent, &theme, &font_assets);
            create_board_buttons(parent, &theme,  &font_assets);
            create_menu(parent, &theme, &font_assets);
        });
}


