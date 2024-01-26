use std::marker::PhantomData;
use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb};
use lunex_engine::*;

use crate::{Dimension, MovableByCamera, UiLink, Element};


/// This function pulls data from marked [`Camera`] and inserts it into marked [`Dimension`].
/// ## 📦 Types
/// * Generic `(M)` - Marker component scoping logic and data into one iterable group
/// ## ⚠️ Warning
/// * Developer should ensure that source query returns only one camera.
///   Otherwise, it will lead to value overwriting. Just make sure only one camera
///   is marked with `(M)` component at the same time.
pub fn fetch_dimension_from_camera<M:Default + Component, N:Default + Component, T: Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Dimension, (With<T>, With<UiTree<M, N>>)>
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
pub fn fetch_transform_from_camera<T: Component>(
    source: Query<&Camera, (With<T>, Changed<Camera>)>,
    mut destination: Query<&mut Transform, (With<T>, With<MovableByCamera>)>
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
pub fn compute_ui<M:Default + Component, N:Default + Component, T: Component>(
    mut query: Query<(&Dimension, &mut UiTree<M, N>), (With<T>, Or<(Changed<Dimension>, Changed<UiTree<M, N>>)>)>
) {
    for (dimension, mut ui) in &mut query {
        // Compute the Ui
        //println!("Ui DIM: {}", dimension.size);
        ui.compute(Rect2D::new().with_size(dimension.size).into());
    }
}

/// This function renders the outlines of the [`UiTree`] in the world
/// ## 📦 Types
/// * Generic `(T)` - Schema struct defining what data can be stored on a single [`UiNode`]
pub fn draw_debug_gizmo<M:Default + Component, N:Default + Component, T: Component>(mut query: Query<(&UiTree<M, N>, &Transform), With<T>>, mut gizmos: Gizmos) {
    for (tree, transform) in &mut query {
        let list = tree.crawl();
        for node in list {
            if let Some(container) = node.obtain_data() {

                let mut color = Color::LIME_GREEN;

                if let Layout::Solid(_) = container.layout { color = Color::YELLOW }

                let mut pos = container.rectangle.pos.invert_y() + transform.translation;
                pos.x += container.rectangle.size.x / 2.0;
                pos.y += container.rectangle.size.y / -2.0;

                gizmos.rect(
                    pos,
                    Quat::from_rotation_y(0.0),
                    container.rectangle.size,
                    color,
                );
            }
        }
    }
}

/// This function prints all changed [`UiTree`]s.
pub fn print_debug_tree<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<&UiTree<M, N>, (With<T>, Changed<UiTree<M, N>>)>
) {
    for ui in &uis {
        info!("{}\n{}\n", "UiTree has been changed...", ui.tree("show-hidden"));
    }
}


pub fn create_layout<M:Default + Component, N:Default + Component, T: Component>(
    mut uis: Query<(&mut UiTree<M, N>, &Children), With<T>>,
    query: Query<(&UiLink, &Layout), (With<T>, Changed<Layout>)>,
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


pub fn sync_linked_transform<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<T>, Without<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rectangle.pos.invert_y();
                    }
                }
            }
        }
    }
}

pub fn sync_linked_dimension<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Dimension), With<T>>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut dimension)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        if dimension.as_ref().size != container.rectangle.size {
                            //info!("Updated dimension: {}", container.rectangle.size);
                            dimension.size = container.rectangle.size;
                        }
                    }
                }
            }
        }
    }
}


pub fn sync_linked_element_transform<M:Default + Component, N:Default + Component, T: Component>(
    uis: Query<(&UiTree<M, N>, &Children), (With<T>, Changed<UiTree<M, N>>)>,
    mut query: Query<(&UiLink, &mut Transform), (With<T>, With<Element>)>,
) {
    for (ui, children) in &uis {
        for child in children {
            // If child matches
            if let Ok((link, mut transform)) = query.get_mut(*child) {
                // If node exists
                if let Ok(node) = ui.borrow_node(link.path.clone()) {
                    //Should always be Some but just in case
                    if let Some(container) = node.obtain_data() {
                        transform.translation = container.rectangle.pos.invert_y();
                        transform.translation.x += container.rectangle.size.x / 2.0;
                        transform.translation.y += container.rectangle.size.y / -2.0;
                    }
                }
            }
        }
    }
}

pub fn reconstruct_element_mesh<T: Component>(
    mut msh: ResMut<Assets<Mesh>>,
    mut query: Query<(&Dimension, &mut Handle<Mesh>, &mut Aabb), (With<T>, With<Element>, Changed<Dimension>)>,
) {
    for (dimension, mut mesh, mut aabb) in &mut query {
        //info!("Recreating mesh: {:?}", aabb);
        let _ = msh.remove(mesh.id());

        *aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(dimension.size.x/2.0, dimension.size.y/2.0, 1.0),
        };
        *mesh = msh.add(shape::Quad { size: dimension.size, flip: false }.into());
    }
}


/// Plugin implementing all Ui logic for the specified generic types.
/// * generic `(T)` - Schema struct defining what data can be stored on [`UiNode`]
/// * generic `(M)` - Marker component scoping logic and data into one iterable group
/// 
/// ## 🛠️ Example
/// *1. Define the types used*
/// ```
///  #[derive(Component, Default)]
///  struct MyMasterData { theme: String } // What data will each tree hold
/// 
///  #[derive(Component, Default)]
///  struct MyNodeData { value: i32 } // What data will each node contain
/// 
///  #[derive(Component)]
///  struct MyUiWidget; // Empty marker, used for selecting between multiple types of Ui
/// ```
/// *2. Add the plugin to your app*
/// ```
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugins(UiPlugin::<MyMasterData, MyNodeData, MyUiWidget>::new())
///      .run();
/// ```
/// *3. Use the [`UiTree`] freely*
/// ```
///#  fn setup(mut commands: Commands) {
///   commands.spawn((
///      MyUiWidget,
///      UiTree::<MyMasterData, MyNodeData>::new("MyWidget")
///   ));
///#  }
/// ```
#[derive(Debug, Default, Clone)]
pub struct UiPlugin <M:Default + Component, N:Default + Component, T: Component>(PhantomData<M>, PhantomData<N>, PhantomData<T>);
impl <M:Default + Component, N:Default + Component, T: Component> UiPlugin<M, N, T> {
    pub fn new() -> Self {
        UiPlugin::<M, N, T>(PhantomData, PhantomData, PhantomData)
    }
}
impl <M:Default + Component, N:Default + Component, T: Component> Plugin for UiPlugin<M, N, T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, create_layout::<M, N, T>.before(compute_ui::<M, N, T>))
            .add_systems(Update, sync_linked_transform::<M, N, T>)
            .add_systems(Update, (sync_linked_dimension::<M, N, T>, reconstruct_element_mesh::<T>).chain())
            .add_systems(Update, sync_linked_element_transform::<M, N, T>)
            .add_systems(Update, (fetch_dimension_from_camera::<M, N, T>, fetch_transform_from_camera::<T>).before(compute_ui::<M, N, T>))
            .add_systems(Update, compute_ui::<M, N, T>);
    }
}

/// Plugin implementing all debug Ui logic for the specified generic types.
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
///  struct MyUiWidget; // Empty marker, used for selecting between multiple types of Ui
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
pub struct UiDebugPlugin <M:Default + Component, N:Default + Component, T: Component>(PhantomData<M>, PhantomData<N>, PhantomData<T>);
impl <M:Default + Component, N:Default + Component, T: Component> UiDebugPlugin<M, N, T> {
    pub fn new() -> Self {
        UiDebugPlugin::<M, N, T>(PhantomData, PhantomData, PhantomData)
    }
}
impl <M:Default + Component, N:Default + Component, T: Component> Plugin for UiDebugPlugin<M, N, T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_debug_gizmo::<M, N, T>)
            .add_systems(Update, print_debug_tree::<M, N, T>);
    }
}