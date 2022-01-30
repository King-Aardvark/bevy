use bevy::{input::touch::*, prelude::*};

fn main() {
    App::new()
        .add_plugin(ExamplesPlugin {
            title: file!().to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_system(touch_event_system)
        .run();
}

fn touch_event_system(mut touch_events: EventReader<TouchInput>) {
    for event in touch_events.iter() {
        info!("{:?}", event);
    }
}
