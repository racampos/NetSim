# Bevy and ECS

Bevy is a relatively new game engine written in Rust. Its main features are:
- Data-driven: Uses the ECS paradigm 
- Open source: Apache or MIT
- Modular: Uses plugins as a means of adding more functionality 
- Fast: Mainly because it’s written in Rust and also thanks to ECS.
- Cross-platform: Supports Windows, Mac and Linux. 

## The ECS paradigm

ECS stands for Entity Component System and it’s an alternative to the Object Oriented Programming (OOP) paradigm.

In ECS you don’t encapsulate data and behavior together, as in OOP. Instead, you break up data into components and have systems act upon them. Entities are unique ids assigned to groups of components.

Baby uses Rust structs to implement Components, Rust functions to implement Systems and plain integers to implement Entities.

Next we will go over each of Bevy's main building blocks:

## Systems
Systems is where you define all your game logic. They are typically implemented using Rust functions. `Queries` are used in tandem with Systems to specify exactly what data we want our system to work on. 

## Queries
A query is a declarative way of specifying what components we want our systems to act upon. They are used to fetch data from the game World according a given specification.

In this example, we access all entities that have `Position` and `Velocity` components. We get read-only access to `Position` and mutable access to `Velocity`:

```rust
fn system(mut query: Query<(&Position, &mut Velocity)>) {
    for (position, mut velocity) in &mut query.iter() {
        // do something
    }
}
```

## Apps
An App is Bevy's main building block. It defines the structure of your game. The App is typically created inside the `main` function.

For example, a very basic but functional App would be defined like this:

```rust
fn main() {
    App::build()
        .add_default_plugins()
        .run();
}
```

## Plugins
Plugins are what make Bevy a highly-scalable and moduler game engine. A plugin is just an alternative way of adding things to the App that doesn't require doing it from the `main` function as discussed above.

This is how you add plugins to an App. In this case, these are the default plugins that already come with Bevy out of the box:

```rust
fn main() {
    App::build()
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(UiPlugin::default())
        .run();
}
```

You can define your own plugins. There are two ways of doing so:
1.  Writing a Rust function that takes `&mut App`:

```rust
fn my_plugin(app: &mut App) {
    app.init_resource::<MyCustomResource>();
    app.add_systems(Update, (
        do_some_things,
        do_other_things,
    ));
}
```

2. Creating a struct and implementing the `Plugin` trait:

```rust
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyOtherResource>();
        app.add_event::<MyEvent>();
        app.add_systems(Startup, plugin_init);
        app.add_systems(Update, my_system);
    }
}
```

## Resources
Resources, as opposed to Components, exist independently of Entities. They're used to store global data, such as configuration and settings.

A Resource type can be defined with either a struct or an enum and deriving the `Resource` trait:

```rust
#[derive(Resource)]
struct GoalsReached {
    main_goal: bool,
    bonus: u32,
}
```

