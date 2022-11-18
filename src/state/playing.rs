use bevy::prelude::*;

use crate::{
    board::*,
    ui::{FontAssets, Theme},
    AppState, 
    menu::create_menu,
    cleanup, SudokuContainer,
};

pub struct StatePlayingPlugin;

impl Plugin for StatePlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set( SystemSet::on_enter(AppState::Playing)
            .with_system(setup_layout)
        )
        .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(cleanup));
    }
}

fn setup_layout(
    mut commands: Commands,
    //config: Res<SudokuConfig>,
    font_assets: Res<FontAssets>,
    theme: Res<Theme>,
    sudoku_container: Res<SudokuContainer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    // root node
    let grid = sudoku_container.0.grid();
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
            create_board(parent, &theme, &font_assets, grid, width, height);
            create_cell_menu(parent, &theme, &font_assets, width, height);
            create_menu(parent, &theme, &font_assets);
        });

        

}
