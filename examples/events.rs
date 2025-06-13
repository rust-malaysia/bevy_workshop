use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_event::<Blast>().add_systems(
        Startup,
        (setup, blast, (read_blast, spawn_blast)).chain(),
    );

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn blast(mut evw_blast: EventWriter<Blast>) {
    evw_blast.write(Blast(Vec2::new(0.0, 0.0)));
    evw_blast.write(Blast(Vec2::new(-100.0, -50.0)));
    evw_blast.write(Blast(Vec2::new(100.0, 50.0)));
}

fn read_blast(mut evr_blast: EventReader<Blast>) {
    for blast in evr_blast.read() {
        info!("blast at {}", blast.0);
    }
}

fn spawn_blast(
    mut commands: Commands,
    mut evr_blast: EventReader<Blast>,
) {
    const SIZE: f32 = 20.0;
    for blast in evr_blast.read() {
        commands.spawn((
            Sprite {
                rect: Some(Rect::default().inflate(SIZE)),
                color: CYAN_500.into(),
                ..default()
            },
            Transform::from_xyz(blast.x, blast.y, 0.0),
        ));
    }
}

#[derive(Event, Debug, Deref)]
pub struct Blast(pub Vec2);
