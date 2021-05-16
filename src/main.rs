use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Vivo".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
