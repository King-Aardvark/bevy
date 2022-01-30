use bevy::{prelude::*, render::options::WgpuOptions};

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            backends: None,
            ..Default::default()
        })
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .run();
}
