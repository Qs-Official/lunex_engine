use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, MyWidget>::new())
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Update, render_update)
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
            layout::Window::FULL.with_pos( Abs::splat2(20.0) ).with_size( Prc::splat2(100.0) - Abs::splat2(40.0) ).pack(),
        ));

        // Spawn `Square` div
        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            layout::Solid::new().with_align_x(Align::CENTER).pack(),
            Transform::default(),
            RenderContainer {
                color: Color::RED,
                corner_radii: Vec4::splat(1.0)
            }
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;



#[derive(Component)]
struct RenderContainer {
    color: Color,
    corner_radii: Vec4
}
fn render_update (mut painter: ShapePainter, query: Query<(&Transform, &RenderContainer)>) {
    for (transform, color) in &query {

        //painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        let ww = transform.scale.x;
        let hh = transform.scale.y;

        painter.color = color.color;
        painter.thickness = 1.0;
        painter.corner_radii = color.corner_radii;
        painter.rect(Vec2::new(ww, hh));
    }
}