mod loading;
mod playing;

use bevy::prelude::*;
use loading::StateLoadingPlugin;
use playing::StatePlayingPlugin;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StateLoadingPlugin)
        .add_plugin(StatePlayingPlugin);
    }
}