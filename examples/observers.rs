use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, spawn_target_observer)
        .add_systems(PostStartup, trigger_global_event)
        .add_observer(global_observer);

    app.run();
}

fn trigger_global_event(mut commands: Commands) {
    // Triggers a global event.
    commands.trigger(Damage(1.0));
}

fn spawn_target_observer(mut commands: Commands) {
    // Spawns an empty entity.
    let entity = commands.spawn_empty().id();

    // Spawns an observer with then empty entity as the target.
    commands.entity(entity).observe(target_observer);

    // Triggers a targeted event.
    commands.trigger_targets(Damage(2.0), entity);
}

// Technically observer systems are not categorized,
// the naming here is just for the convenience of understanding.

fn global_observer(trigger: Trigger<Damage>) {
    info!("Global: {}, {:?}", trigger.target(), trigger.event());
}

fn target_observer(trigger: Trigger<Damage>) {
    info!("Target: {}, {:?}", trigger.target(), trigger.event());
}

#[derive(Event, Debug)]
pub struct Damage(pub f32);
