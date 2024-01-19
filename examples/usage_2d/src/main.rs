use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn((
        MyWidget,
        Camera2dBundle::new_with_far(100.0)
    ));

    commands.spawn((
        // Create new DOM called `MyWidget`
        UiTreeBundle::<NoData, MyWidget>::from( UiTree::<NoData>::new("MyWidget") ),
        
        // Marks the `Transform` to recive data from camera size
        MovableByCamera,

    )).with_children(|parent| {

        // Spawn `Root` div
        parent.spawn((
            MyWidget,
            UiLink::path("Root"),
            //                               (20px, 20px)                        (100% - 40px, 100% - 40px)
            layout::Window::FULL.with_pos( Abs::splat2(20.0) ).with_size( Prc::splat2(100.0) + Abs::splat2(-40.0) ).pack(),
        ));

        // Spawn `Square` div
        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            layout::Solid::new().with_align_x(Align::CENTER).pack(),
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;