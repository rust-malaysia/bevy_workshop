use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
        flappy_bird::GamePlugin,
    ));

    app.run();
}
