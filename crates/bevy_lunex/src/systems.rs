use std::marker::PhantomData;
use bevy::prelude::*;
use lunex_engine::*;

use crate::{Dimension, MovableByCamera, UiLink, Element};


/// This function pulls data from marked [`Camera`] and inserts it into marked [`Dimension`].
/// ## 📦 Types
/// * Generic `(M)` - Marker component scoping logic and data into one iterable group
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(M)` component at the same time.
pub fn fetch_dimension_from_camera<T:Default + Component, M: Component>(
    source: Query<&Camera, (With<M>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<M>, With<UiTree<T>>)>
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        for mut dimension in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                dimension.size = Vec2::from((size.x as f32, size.y as f32));
            }
        }
    }
}

/// This function pulls data from marked [`Camera`] and inserts it into marked [`Transform`] that has [`MovableByCamera`].
/// ## 📦 Types
/// * Generic `(M)` - Marker component scoping logic and data into one iterable group
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(M)` component at the same time.
pub fn fetch_transform_from_camera<M: Component>(
    source: Query<&Camera, (With<M>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<M>, With<MovableByCamera>)>
) {
    // Undesired behaviour if source.len() > 1
    for cam in &source {
        for mut transform in &mut destination {
            // Extract camera size
            if let Some(size) = cam.physical_viewport_size() {
                transform.translation = Vec3::from((size.x as f32 /-2.0, size.y as f32 /2.0, 0.0));
            }
        }
    }
}

/// This function triggers computation method on marked [`UiTree`] with data from appended [`Dimension`] component.
/// ## 📦 Types
/// * Generic `(T)` - Schema struct defining what data can be stored on a single [`UiNode`]
/// * Generic `(M)` - Marker component scoping logic and data into one iterable group
pub fn compute_ui<T:Default + Component, M: Component>(
    mut query: Query<(&Dimension, &mut UiTree<T>), (With<M>, Or<(Changed<Dimension>, Changed<UiTree<T>>)>)>
) {
    for (dimension, mut ui) in &mut query {
        // Compute the UI
        //println!("UI DIM: {}", dimension.size);
        ui.compute(Rect2D::new().with_size(dimension.size).into());
    }
}

/// This function renders the outlines of the [`UiTree`] in the world
/// ## 📦 Types
/// * Generic `(T)` - Schema struct defining what data can be stored on a single [`UiNode`]
pub fn draw_debug_gizmo<T:Default + Component>(mut query: Query<(&UiTree<T>, &Transform)>, mut gizmos: Gizmos) {
    for (tree, transform) in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                let mut color = Color::LIME_GREEN;

                if let Layout::Solid(_) = container.layout { color = Color::YELLOW }

                let mut pos = container.rect.pos.invert_y() + transform.translation;
                pos.x += container.rect.size.x / 2.0;
                pos.y += container.rect.size.y / -2.0;

                gizmos.rect(
                    pos,
                    Quat::from_rotation_y(0.0),
                    container.rect.size,
                    color,
                );
            }
        }
    }
}



pub fn create_layout<T:Default + Component, M: Component>(
    mut uis: Query<(&mut UiTree<T>, &Children), With<M>>,
    query: Query<(&UiLink, &Layout), (With<M>, Changed<Layout>)>,
) {
    for (mut ui, children) in &mut uis {
        for child in children {
            // If child matches
            if let Ok((link, layout)) = query.get(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_or_create_ui_node_mut(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data_mut() {
                        //println!("Changed layout");
                        container.layout = *layout;
                    }
                }
            }
        }
    }
}


pub fn sync_linked_transform<T:Default + Component, M: Component>(
    uis: Query<(&UiTree<T>, &Children), (With<M>, Changed<UiTree<T>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<M>, Without<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rect.pos.invert_y();
                    }
                }
            }
        }
    }
}

pub fn sync_linked_dimension<T:Default + Component, M: Component>(
    uis: Query<(&UiTree<T>, &Children), (With<M>, Changed<UiTree<T>>)>,
    mut query: Query<(&UiLink, &mut Dimension), With<M>>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut dimension)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        if dimension.as_ref().size != container.rect.size {
                            //info!("Updated dimension: {}", container.rect.size);
                            dimension.size = container.rect.size;
                        }
                    }
                }
            }
        }
    }
}


pub fn sync_linked_element_transform<T:Default + Component, M: Component>(
    uis: Query<(&UiTree<T>, &Children), (With<M>, Changed<UiTree<T>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<M>, With<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rect.pos.invert_y();
                        transform.translation.x += container.rect.size.x / 2.0;
                        transform.translation.y += container.rect.size.y / -2.0;
                    }
                }
            }
        }
    }
}

pub fn reconstruct_element_mesh<M: Component>(
    mut msh: ResMut<Assets<Mesh>>,
    mut query: Query<(&Dimension, &mut Handle<Mesh>), (With<M>, With<Element>, Changed<Dimension>)>,
) {
    for (dimension, mut mesh) in &mut query {
        //info!("Recreating mesh: {}", dimension.size);
        *mesh = msh.add(shape::Quad { size: dimension.size, flip: false }.into());
    }
}


/// Plugin implementing all UI logic for the specified generic types.
/// * generic `(T)` - Schema struct defining what data can be stored on [`UiNode`]
/// * generic `(M)` - Marker component scoping logic and data into one iterable group
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct NodeData { value: i32 } // What data will each node contain
/// 
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for multiple types of UI
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiPlugin::<NodeData, MyUiWidget>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      MyUiWidget,
///      UiTree::<NodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <T:Default + Component, M: Component>(PhantomData<T>, PhantomData<M>);
impl <T:Default + Component, M: Component> UiPlugin<T, M> {
    pub fn new() -> Self {
        UiPlugin::<T, M>(PhantomData, PhantomData)
    }
}
impl <T:Default + Component, M: Component> Plugin for UiPlugin<T, M> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, create_layout::<T, M>)
            .add_systems(Update, sync_linked_transform::<T, M>)
            .add_systems(Update, (sync_linked_dimension::<T, M>, reconstruct_element_mesh::<M>).chain())
            .add_systems(Update, sync_linked_element_transform::<T, M>)
            .add_systems(Update, (fetch_dimension_from_camera::<T, M>, fetch_transform_from_camera::<M>).before(compute_ui::<T, M>))
            .add_systems(Update, compute_ui::<T, M>);
    }
}

/// Plugin implementing all debug UI logic for the specified generic types.
/// * generic `(T)` - Schema struct defining what data can be stored on [`UiNode`]
/// * generic `(M)` - Marker component scoping logic and data into one iterable group
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct NodeData { value: i32 } // What data will each node contain
/// 
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for multiple types of UI
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiDebugPlugin::<NodeData, MyUiWidget>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      MyUiWidget,
///      UiTree::<NodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiDebugPlugin <T:Default + Component, M: Component>(PhantomData<T>, PhantomData<M>);
impl <T:Default + Component, M: Component> UiDebugPlugin<T, M> {
    pub fn new() -> Self {
        UiDebugPlugin::<T, M>(PhantomData, PhantomData)
    }
}
impl <T:Default + Component, M: Component> Plugin for UiDebugPlugin<T, M> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_debug_gizmo::<T>);
    }
}