use bevy::prelude::*;

/// This example illustrates how to load and play an audio file
fn main() {
    App::new()
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/Windless Slopes.ogg");
    audio.play(music);
}
