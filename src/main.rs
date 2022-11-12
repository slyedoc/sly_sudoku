mod board;
mod ui;
mod menu;
mod lens;

use bevy_tweening::TweeningPlugin;
use menu::*;
use board::*;
use ui::*;
use lens::*;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use sudoku_variants::{Sudoku, constraint::DefaultConstraint};

#[derive(Deref, DerefMut, Resource)]
pub struct SukokuState(pub Sudoku<DefaultConstraint>);

impl Default for SukokuState {
    fn default() -> Self {
        Self(Sudoku::new_empty(3,3, DefaultConstraint).unwrap())
    }
}

fn main() {
    App::new()

        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                  width: 400.0,
                  ..default()
                },
                ..default()
              })
        )

        //.add_plugin(WorldInspectorPlugin::default())        
        .add_plugin(TweeningPlugin)
        // grid
        //.add_plugin(bevy_infinite_grid::InfiniteGridPlugin)
        //.add_startup_system(spawn_grid)
        // Setup Resources
        .init_resource::<SukokuState>()
        
        // Local Plugins
        .add_plugin(UIPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LensPlugin)
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
    commands.spawn(Camera2dBundle {
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
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Layout"))
        .with_children(|parent| {
            create_board(parent, &theme, &font_assets);
            create_board_buttons(parent, &theme,  &font_assets);
            create_menu(parent, &theme, &font_assets);
        });
}


