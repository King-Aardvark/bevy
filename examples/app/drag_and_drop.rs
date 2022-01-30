use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_system(file_drag_and_drop_system)
        .run();
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.iter() {
        info!("{:?}", event);
    }
}
