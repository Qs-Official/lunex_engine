use std::borrow::Borrow;

use bevy::ecs::component::Component;
use bevy::math::Vec3Swizzles;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Layout;
use crate::MasterData;
use crate::NodeSizeEvaluate;
use crate::Rect2D;
use crate::Rect3D;
use crate::import::*;
use crate::StackDirection;

use super::{UiNode, UiTree, NodeData};




// #============================#
// #=== DIRECT UINODE TRAITS ===#

/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeCreationTrait<N:Default + Component> {
    /// ## Make node
    /// Makes new subnode in this node and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::create_node`] for hierarchy creation
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Create node
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::make_node`] for direct creation
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Obtain or create node
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node`] for hierarchy retrieval
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Obtain or create node mut
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node_mut`] for hierarchy retrieval
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;

    /// ## Borrow or create node
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node`] for direct retrieval
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Borrow or create node mut
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node_mut`] for direct retrieval
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;  
}
impl <M: Default + Component, N: Default + Component> UiNodeCreationTrait<N> for UiTree<M, N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.make_ui_node(name)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.create_ui_node(path)
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node(path)
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node_mut(path)
    }
}
impl <N: Default + Component> UiNodeCreationTrait<N> for UiNode<N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError> {
        let n = self.make_node(name)?;
        self.insert_data(n.clone(), NodeData::default())?;
        Ok(n)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError> {
        let mut node: UiNode<N> = Node::new();
        node.add_data(NodeData::default());
        self.insert_node(path, Node::new())
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node(n)
        }
        self.obtain_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node_mut(n)
        }
        self.obtain_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node(rempath),
        }
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node_mut(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node_mut(rempath),
        }
    }
}

/// ## UiNode data trait
/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for [`UiTree`] data management.
pub trait UiNodeDataTrait<N> {
    /// ## Add ui data
    /// Adds new data to this node and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::insert_ui_data`] for hierarchy insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn add_ui_data(&mut self, data: N) -> Option<N>;

    /// ## Insert ui data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::add_ui_data`] for direct insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError>;

    /// ## Take ui data
    /// Removes data from this node and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::remove_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn take_ui_data(&mut self) -> Option<N>;

    /// ## Remove ui data
    /// Removes data from this node or any other subnode and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::take_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError>;

    /// ## Obtain ui data
    /// Borrows data from this node.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data(&self) -> Option<&N>;

    /// ## Obtain ui data mut
    /// Borrows data from this node as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data_mut`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N>;

    /// ## Borrow ui data
    /// Borrows data from this node or any other subnode.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError>;

    /// ## Borrow ui data mut
    /// Borrows data from this node or any other subnode as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data_mut`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError>;
}
impl <M: Default + Component, N: Default + Component> UiNodeDataTrait<N> for UiTree<M, N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        self.node.add_ui_data(data)
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        self.node.insert_ui_data(path, data)
    }

    fn take_ui_data(&mut self) -> Option<N> {
        self.node.take_ui_data()
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        self.node.remove_ui_data(path)
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        self.node.obtain_ui_data()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        self.node.obtain_ui_data_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        self.node.borrow_ui_data(path)
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        self.node.borrow_ui_data_mut(path)
    }
}
impl <N: Default + Component> UiNodeDataTrait<N> for UiNode<N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, Some(data))
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }

    fn take_ui_data(&mut self) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, None)
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, None))
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        let Some(container) = self.obtain_data() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_ref()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        let Some(container) = self.borrow_data(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_ref())
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_mut())
    }
}


/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeTreeInitTrait {
    /// ## New
    /// Creates new UiTree.
    fn new(name: impl Borrow<str>) -> Self;
}
impl <M: Default + Component, N: Default + Component> UiNodeTreeInitTrait for UiTree<M, N> {
    fn new(name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<M, N> = NodeTreeInitTrait::new(name);
        tree.add_topdata(MasterData::default());
        tree.add_data(NodeData::default());
        tree
    }
}



/// ## Node tree compute trait
/// Trait with all node tree layout computation implementations.
pub trait UiNodeTreeComputeTrait {
    fn compute(&mut self, parent: Rect3D);
}
impl <M: Default + Component, N: Default + Component> UiNodeTreeComputeTrait for UiTree<M, N> {
    fn compute(&mut self, parent: Rect3D) {

        let mut abs_scale = 1.0;
        let mut font_size = 16.0;

        if let Some(master_data) = self.obtain_topdata() {
            abs_scale = master_data.abs_scale;
            font_size = master_data.font_size;
        }

        self.node.compute_all(parent, abs_scale, font_size);
    }
}

/// ## Node compute trait
/// Trait with all node layout computation implementations. Includes private methods.
trait UiNodeComputeTrait {
    fn compute_all(&mut self, parent: Rect3D, abs_scale: f32, font_size: f32);
    //fn position_all(&mut self, parent: Rect3D, abs_scale: f32, font_size: f32);
    //fn compute_content(&mut self, position: Vec2, size: Vec2, padding: Vec4, abs_scale: f32, font_size: f32) -> Vec2;
    //fn compute_stack(&mut self, position: Vec2, size: Vec2, padding: Vec4, abs_scale: f32, font_size: f32, horizontal: bool) -> Vec2;
    fn compute_content(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32) -> Vec2;
    fn compute_stack(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32, horizontal: bool) -> Vec2;
    fn align_stack(&mut self, ancestor_position: Vec2);
}
impl <N:Default + Component> UiNodeComputeTrait for UiNode<N> { 
    fn compute_all(&mut self, parent: Rect3D, abs_scale: f32, mut font_size: f32) {

        // Get depth before mutating self
        let depth = self.get_depth();
        
        let mut skip = true;
        let mut is_parametric = false;

        // Check here if computation is required for partial recalculation

        // Compute my layout and return computed rectangle for recursion
        let my_rectangle = if let Some(node_data) = &mut self.data {

            // Overwrite passed style with font size
            if let Some(fnt) = node_data.font_size { font_size = fnt }

            // Compute node layout
            match &node_data.layout {
                Layout::Div(_) => {
                    is_parametric = true;
                },
                Layout::Window(l) => {
                    node_data.rectangle = l.compute(parent.into(), abs_scale, font_size).into();
                    skip = false;
                },
                Layout::Solid(l)  => {
                    node_data.rectangle = l.compute(parent.into(), abs_scale, font_size).into();
                    skip = false;
                },
            }

            // Adding depth
            node_data.rectangle.pos.z = depth;
            node_data.rectangle

        } else { return; };

        if skip == false {
            if is_parametric {
                //compute divs with inherited scale
                self.compute_content(parent.size, Vec4::ZERO, abs_scale, font_size);
            } else {
                //compute divs with my rectangle scale
                self.compute_content(my_rectangle.size, Vec4::ZERO, abs_scale, font_size);
            }
        }

        // Enter recursion
        for (_, subnode) in &mut self.nodes {
            subnode.compute_all(my_rectangle, abs_scale, font_size);
        }
    }
    fn compute_content(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32) -> Vec2 {

        let stack_options = self.data.as_ref().unwrap().stack;

        match stack_options.direction {
            StackDirection::Horizontal => self.compute_stack(ancestor_size, ancestor_padding, abs_scale, font_size, true),
            StackDirection::Vertical => self.compute_stack(ancestor_size, ancestor_padding, abs_scale, font_size, false),
        }
    }
    /// This will compute the stack and position nodes ONLY locally as if every matrix starts at 0,0.
    /// Secondary pass after alignment of parent nodes is required.
    fn compute_stack(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32, horizontal: bool) -> Vec2 {

        let mut matrix: Vec<Vec<&mut Node<NodeData<N>>>> = Vec::new();
        let mut content_size = Vec2::ZERO;

        // Sort mutable pointers into matrix
        let mut i = 0;
        matrix.push(Vec::new());
        for (_, subnode) in &mut self.nodes {
            if let Some(subnode_data) = &subnode.data {
                if let Layout::Div(layout) = &subnode_data.layout {
                    let br = layout.force_break;
                    matrix[i].push(subnode);
                    if br {
                        i += 1;
                        matrix.push(Vec::new());
                    }
                }
            }
        }


        // INSIDE MATRIX =================================================================

        let _gap = self.data.as_ref().unwrap().stack.gap.evaluate(abs_scale, ancestor_size, font_size);
        let align = self.data.as_ref().unwrap().stack.node_alignment.0;


        let mut line_cursor = if horizontal { ancestor_padding.y } else { ancestor_padding.x };

        //--------------------------//
        let mut _i = 0;             //
        let _i_max = matrix.len();  //
        for line in &mut matrix {   //
            // INSIDE LINE =================================================================

            // Register that is shared between the two passes
            let mut comline = ComputedLine {
                divs: Vec::new(),
                line_length: 0.0,
            };

            // First pass to compute sizes--//
            let mut _ii = 0;                //
            let _ii_max = line.len();       //
            for subnode in &mut *line {     //
                // INSIDE SUBNODE =================================================================

                // Fetch data
                let subnode_data = subnode.data.as_ref().unwrap();
                let layout = if let Layout::Div(layout) = subnode_data.layout { layout } else { unreachable!() };

                // Get padding & margin => compute range of motion
                let padding = layout.compute_padding(ancestor_size, abs_scale, font_size);
                let margin = layout.compute_margin(ancestor_size, abs_scale, font_size);
                let border = layout.compute_border(ancestor_size, abs_scale, font_size);

                // Enter recursion to get the right content size
                let potential_content = subnode.compute_content(ancestor_size, padding, abs_scale, font_size);

                // Fetch data again, because they were modified
                let subnode_data = subnode.data.as_mut().unwrap();
                let mut subnode_content = subnode_data.content_size;

                // Overwrite subnode content if div contains no subdivs
                if potential_content != Vec2::ZERO { subnode_content = potential_content }

                // Compute size and (line_length)
                let size = layout.compute_size(subnode_content, padding, border);
                let line_length = if horizontal { margin.y + size.y + margin.w } else { margin.x + size.x + margin.z };

                // Push into register
                comline.line_length = f32::max(comline.line_length, line_length);
                comline.divs.push(ComputedDiv { size, margin });


                // END OF INSIDE SUBNODE =================================================================
                _ii += 1;
            }

            let mut cursor = if horizontal { ancestor_padding.x } else { ancestor_padding.y };

            // Second pass to align them----//
            let mut _ii = 0;                //
            let _ii_max = line.len();       //
            for subnode in &mut *line {     //
                // INSIDE SUBNODE =================================================================

                // Fetch data
                let subnode_data = subnode.data.as_ref().unwrap();
                let layout = if let Layout::Div(layout) = subnode_data.layout { layout } else { unreachable!() };
                
                let margin = comline.divs[_ii].margin;
                let size = comline.divs[_ii].size;

                let possible_size = if horizontal {comline.line_length - margin.y - margin.w } else {comline.line_length - margin.x - margin.z };

                let mut my_align = align;
                let my_offset;
                
                if horizontal {
                    if let Some(align) = layout.align_y { my_align = align.0 }

                    cursor += margin.x;
                    let off = margin.y + possible_size/2.0 - size.y/2.0;
                    my_offset = Vec2::new(cursor, off + (off - margin.x) * my_align);
                    cursor += size.x;
                    cursor += margin.z;

                } else {
                    if let Some(align) = layout.align_x { my_align = align.0 }

                    cursor += margin.y;
                    let off = margin.x + possible_size/2.0 - size.x/2.0;
                    my_offset = Vec2::new(off + (off - margin.x) * my_align, cursor);
                    cursor += size.y;
                    cursor += margin.w;
                };
                

                // Fetch data again, because they were modified
                let subnode_data = subnode.data.as_mut().unwrap();

                subnode_data.rectangle = Rect2D {
                    pos: my_offset + if horizontal { Vec2::new(0.0, line_cursor) } else { Vec2::new(line_cursor, 0.0) },
                    size,
                }.into();

                let subnode_data = subnode.data.as_ref().unwrap();
                subnode.align_stack(subnode_data.rectangle.pos.xy());


                // END OF INSIDE SUBNODE =================================================================
                _ii += 1;
            }

            // Set content size
            if horizontal {
                content_size.x = f32::max(content_size.x, cursor - ancestor_padding.x)
            } else {
                content_size.y = f32::max(content_size.y, cursor - ancestor_padding.y)
            }

            line_cursor += comline.line_length;

            // END OF INSIDE LINE =================================================================
            _i += 1;
        }

        // Set content size
        if horizontal {
            content_size.y = line_cursor - ancestor_padding.y
        } else {
            content_size.x = line_cursor - ancestor_padding.x
        }
        
        // END OF INSIDE MATRIX =========================================================
        content_size
    }
    fn align_stack(&mut self, ancestor_position: Vec2) {
        let mut matrix: Vec<Vec<&mut Node<NodeData<N>>>> = Vec::new();

        // Sort mutable pointers into matrix
        let mut i = 0;
        matrix.push(Vec::new());
        for (_, subnode) in &mut self.nodes {
            if let Some(subnode_data) = &subnode.data {
                if let Layout::Div(layout) = &subnode_data.layout {
                    let br = layout.force_break;
                    matrix[i].push(subnode);
                    if br {
                        i += 1;
                        matrix.push(Vec::new());
                    }
                }
            }
        }



        let mut _i = 0;
        let _i_max = matrix.len();
        for line in &mut matrix {

            let mut _ii = 0;
            let _ii_max = line.len();
            for subnode in &mut *line {
                // =================================================================

                let subnode_data = subnode.data.as_mut().unwrap();
                subnode_data.rectangle.pos.x += ancestor_position.x;
                subnode_data.rectangle.pos.y += ancestor_position.y;
                subnode.align_stack(ancestor_position);

                // =================================================================
                _ii += 1;
            }
            _i += 1;
        }
    }
}

struct ComputedDiv {
    /// Size of the div
    size: Vec2,
    /// Merged context and own margin
    margin: Vec4,
}

struct ComputedLine {
    divs: Vec<ComputedDiv>,
    /// Margin1 + SIZE + Margin2
    line_length: f32,
    // Future Margin1
    //line_padding: f32,
}





// #========================================#
// #=== FUNCTIONALITY WITH UINODE TRAITS ===#


/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized;
}
impl BuildAsNode for layout::Window {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}
impl BuildAsNode for layout::Solid {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}


/// ## Sync to node
/// Trait that [Component] types which represent values in [UiTree] need to
/// implement to load and store data in [UiTree].
pub trait SyncToNode {
    fn load<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
    fn save<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
}





pub trait Extract <T> {
    fn get_extract (&self) -> T;
    fn set_extract (&mut self, val: T) -> T;
}