use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(PreStartup, pre_startup)
        .add_systems(Startup, startup)
        .add_systems(PostStartup, post_startup)
        .add_systems(PreUpdate, pre_update)
        .add_systems(Update, update)
        .add_systems(PostUpdate, post_update)
        .add_systems(FixedPreUpdate, fixed_pre_update)
        .add_systems(FixedUpdate, fixed_update)
        .add_systems(FixedPostUpdate, fixed_post_update);

    app.run();
}

fn pre_startup() {
    info!("pre_startup");
}

fn startup() {
    info!("startup");
}

fn post_startup() {
    info!("post_startup");
}

fn pre_update() {
    info!("pre_update");
}

fn update() {
    info!("update");
}

fn post_update() {
    info!("post_update");
}

fn fixed_pre_update() {
    info!("fixed_pre_update");
}

fn fixed_update() {
    info!("fixed_update");
}

fn fixed_post_update() {
    info!("fixed_post_update");
}
