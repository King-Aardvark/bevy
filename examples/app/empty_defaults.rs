use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .run();
}
