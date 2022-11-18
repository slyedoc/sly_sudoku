use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};

use crate::{
    cleanup,
    events::*,
    ui::{FontAssets, Theme},
    AppState,
};

pub struct StateLoadingPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

impl Plugin for StateLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(setup_ui)
                .with_system(setup_new_game),
        )
        //.add_system_set(SystemSet::on_update(AppState::Loading).with_system(update_text))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(new_game_ready))
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(cleanup))
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.333))
                // TODO: limit to loading state
                .with_system(update_text),
        );
    }
}

fn is_done(state: Res<State<AppState>>) -> ShouldRun {
    if state.current() == &AppState::Loading {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Component)]
struct LoadingText;

fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>, theme: Res<Theme>) {
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(10.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.loading_text("Generating", &theme),
                    font_assets.loading_text("", &theme),
                ],
            },
            ..Default::default()
        },
        Name::new("ui Loading"),
        LoadingText,
    ));
}

fn setup_new_game(mut new_game_event: EventWriter<NewGame>) {
    new_game_event.send(NewGame);
}

fn new_game_ready(
    mut new_game_event: EventReader<NewGameReady>,
    mut app_state: ResMut<State<AppState>>,
) {
    for _ in new_game_event.iter() {
        app_state.set(AppState::Playing).unwrap();
    }
}

fn update_text(
    state: Res<State<AppState>>,
    mut query: Query<&mut Text, With<LoadingText>>,
    mut count: Local<usize>,
) {
    if state.current() == &AppState::Loading {
        for mut text in query.iter_mut() {
            // Update the value of the second section
            let str = match *count {
                0 => ".",
                1 => "..",
                _ => "...",
            };
            text.sections[1].value = str.to_string();
            *count = (*count + 1) % 3;
        }
    }
}
