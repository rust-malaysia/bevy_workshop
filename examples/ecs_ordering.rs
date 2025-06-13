use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, (ivn, nik, jef, nix, han));
    // app.add_systems(Startup, (ivn, nik, jef, nix, han).chain());

    app.run();
}

fn ivn() {
    info!("ivn");
}

fn nik() {
    info!("nik");
}

fn jef() {
    info!("jef");
}

fn nix() {
    info!("nix");
}

fn han() {
    info!("han");
}
