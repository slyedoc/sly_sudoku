mod board;
mod config;
mod ui;
mod menu;

use menu::*;
use board::*;
use config::*;
use ui::*;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;


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

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sudoku".to_string(),
            ..default()
        })
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        // grid
        //.add_plugin(bevy_infinite_grid::InfiniteGridPlugin)
        //.add_startup_system(spawn_grid)
        // Setup Resources
        .insert_resource(ClearColor(Color::WHITE))
        // Local Plugins
        .add_plugin(UIPlugin)
        .add_plugin(ConfigPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MenuPlugin)
        .add_loopless_state(GameState::Loading)
        // global setup
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_layout)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}



fn spawn_layout(
    mut commands: Commands,
    //config: Res<SudokuConfig>,
    font_assets: Res<FontAssets>,
    button_assets: Res<ButtonAssets>,
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
            create_board(parent, &button_assets, &font_assets);
            create_board_buttons(parent, &button_assets,  &font_assets);
            create_menu(parent, &button_assets, &font_assets);
        });
}


