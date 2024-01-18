use std::marker::PhantomData;
use bevy::prelude::*;
use lunex_engine::*;

/// ## Compute camera ui
/// This function triggers computation method on camera's [`UiTree`] component if there was a change.
pub fn compute_camera_ui<T:Default + Component>(mut query: Query<(&Camera, &mut UiTree<T>), Or<(Changed<Camera>, Changed<UiTree<T>>)>>) {
    for (cam, mut ui) in &mut query {

        // Extract camera size
        if let Some(size) = cam.physical_viewport_size() {
            let size: (u32, u32) = size.into();

            // Compute the UI with the extracted size
            ui.compute(Rect2D::new().with_size((size.0 as f32, size.1 as f32)).into());
        }
    }
}

pub fn draw_debug_gizmo<T:Default + Component>(mut query: Query<&UiTree<T>>, mut gizmos: Gizmos) {
    for tree in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                /*gizmos.rect(
                    Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
                    Quat::from_rotation_y(PI / 2.),
                    Vec2::splat(2.),
                    Color::GREEN,
                );*/

                gizmos.rect_2d(
                    container.rect.pos.truncate() + container.rect.size / 2.0,
                    0.0,
                    container.rect.size,
                    Color::LIME_GREEN,
                );
            }
        }
    }
}


//pub fn weird<T: Component>(mut query: Query<&mut T>) {}



/// ## UI plugin
/// THis
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <T:Default + Component>(pub PhantomData<T>);
impl <T:Default + Component> UiPlugin<T> {
    pub fn new() -> Self {
        UiPlugin::<T>(PhantomData)
    }
}
impl <T:Default + Component> Plugin for UiPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_debug_gizmo::<T>)
            .add_systems(Update, compute_camera_ui::<T>);
    }
}