use bevy::prelude::*;
use bevy_oddio::{AudioPlugin, Audio, frames::Stereo};
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
    mut audio: ResMut<Audio<Stereo>>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        info!("spacebar pressed");
        let _random_footstep_index = rand::thread_rng().gen_range(1..8);
        audio.play(asset_server.load(
            "footstep/footstep-1.ogg"    //this plays first sound
            // format!("footstep/footstep-{}.ogg", _random_footstep_index).as_str() // this plays all sounds in folder (how i was originally doing it)
        ), 0.0);
    }
}
