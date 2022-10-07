use bevy::prelude::*;
use bevy_oddio::{AudioPlugin, Audio, frames::{Stereo, Mono}};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1200.0,
            height: 800.0,
            title: "Test".to_string(),
            ..default()
        })
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_system(player_walking_sound_system)
        .run();
}

pub fn player_walking_sound_system(
    mut audio: ResMut<Audio<Mono>>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        info!("spacebar pressed");
        let random_footstep_index = rand::thread_rng().gen_range(1..8);
        audio.play(asset_server.load(format!("footstep/footstep-{}.ogg", random_footstep_index).as_str()), 0.0);
    }
}
