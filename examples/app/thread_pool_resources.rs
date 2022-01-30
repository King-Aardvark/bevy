use bevy::prelude::*;

/// This example illustrates how to customize the thread pool used internally (e.g. to only use a
/// certain number of threads).
fn main() {
    App::new()
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(4))
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .run();
}
