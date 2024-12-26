use bevy::{app::App, DefaultPlugins};
use map::MapPlugin;

mod map;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(MapPlugin { draw: true });
    app.run();
}
