use bevy::{app::AppExit, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use sudoku_variants::{constraint::*, generator::*, solver::*, *};

use std::fmt::{Display, Formatter};

use crate::{
    board::{Cell, CellPosition},
    ui::{FontAssets, Theme, ThemeToggle, ThickLine},
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Reset>()
            .add_event::<NewGame>()
            .add_event::<Solve>()
            .add_system(menu_button_system)
            .add_system(reset_events)
            .add_system(new_game_events)
            .add_system(solve_events);
    }
}

// Events
pub struct NewGame;
pub struct Reset;
pub struct Solve;

#[derive(EnumIter, Debug, Component)]
enum MenuButton {
    New,
    Reset,
    Solve,
    ThemeToggle,
    Quit,
}

impl Display for MenuButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuButton::New => write!(f, "New Game"),
            MenuButton::Reset => write!(f, "Reset"),
            MenuButton::Solve => write!(f, "Solve"),
            MenuButton::ThemeToggle => write!(f, "Theme"),
            MenuButton::Quit => write!(f, "Quit"),
        }
    }
}

pub fn create_menu(parent: &mut ChildBuilder, theme: &Theme, font_assets: &FontAssets) {
    // Menu buttons
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            color: theme.line_thick.into(),
            ..default()
        })
        .insert(ThickLine)
        .insert(Name::new("Menu Buttons"))
        .with_children(|parent| {
            for button in MenuButton::iter() {
                let name = format!("{button}");
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: theme.btn_normal.into(),
                        ..default()
                    })
                    .insert(Name::new(name.clone()))
                    .insert(button)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: name,
                                    style: TextStyle {
                                        font: font_assets.ui_font.clone(),
                                        font_size: 30.0,
                                        color: Color::BLACK,
                                    },
                                }],
                                alignment: TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            },
                            ..default()
                        });
                    });
            }
        });
}

fn menu_button_system(
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut new_game_event: EventWriter<NewGame>,
    mut reset_event: EventWriter<Reset>,
    mut solve_event: EventWriter<Solve>,
    mut theme_toggle_event: EventWriter<ThemeToggle>,
    mut exit_event: EventWriter<AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match menu_button {
                    MenuButton::New => {
                        new_game_event.send(NewGame);
                    }
                    MenuButton::Reset => {
                        reset_event.send(Reset);
                    }
                    MenuButton::Solve => {
                        solve_event.send(Solve);
                    }
                    MenuButton::ThemeToggle => {
                        theme_toggle_event.send(ThemeToggle);
                    }
                    MenuButton::Quit => {
                        exit_event.send(AppExit);
                    }
                }
            }
            _ => {}
        }
    }
}

fn new_game_events(
    mut new_game_event: EventReader<NewGame>,
    mut query: Query<(&mut Cell, &CellPosition)>,
) {
    for _ in new_game_event.iter() {
        let mut generator = Generator::new_default();
        let mut sudoku = generator.generate(3, 3, DefaultConstraint).unwrap();
        let mut reducer = Reducer::new_default();
        reducer.reduce(&mut sudoku);
        let grid = sudoku.grid();

        for (mut cell, cell_position) in query.iter_mut() {
            if let Ok(grid_cell) = grid.get_cell(cell_position.x, cell_position.y) {
                cell.init_value(grid_cell);
            }
        }
    }
}

fn reset_events(mut reset_event: EventReader<Reset>, mut query: Query<&mut Cell>) {
    for _ in reset_event.iter() {
        for mut cell in query.iter_mut() {
            cell.reset();
        }
    }
}

fn solve_events(
    mut solve_events: EventReader<Solve>,
    mut query: Query<(&mut Cell, &CellPosition)>,
) {
    for _ in solve_events.iter() {
        let mut sudoku = Sudoku::new_empty(3, 3, DefaultConstraint).unwrap();
        let grid = sudoku.grid_mut();
        for (cell, cell_pos) in query.iter_mut() {
            if let Some(value) = cell.value() {
                grid.set_cell(cell_pos.x, cell_pos.y, value.into()).unwrap();
            }
        }
        if !sudoku.is_valid() {
            error!("Sudoku is not valid");
            return;
        }

        let solver = BacktrackingSolver;
        match solver.solve(&sudoku) {
            Solution::Impossible => {
                error!("Sudoku is Ambiguous");
            }
            Solution::Unique(solution) => {
                for (mut cell, cell_pos) in query.iter_mut() {
                    if let Ok(grid_cell) = solution.get_cell(cell_pos.x, cell_pos.y) {
                        cell.set_value(grid_cell.unwrap().into());
                    }
                }
            }
            Solution::Ambiguous => {
                error!("Sudoku is Ambiguous");
            }
        }
    }
}
