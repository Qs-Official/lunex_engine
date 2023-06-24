use bevy::prelude::*;
use crate::library::prelude::*;

//# This function constructs the Hierarchy and layout of the main menu.
pub fn create_main_menu() -> Hierarchy {

    let mut system = Hierarchy::new();

    //# Create APP widget
    let _app = Widget::new(&mut system, "App", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create HANDLE in WIDGET
    let _handle = Widget::new_in(&mut system, &_app, "Handle", Layout::Window {
        relative: Vec2 { x: -5.0, y: -5.0 },
        width_relative: 110.0,
        height_relative: 110.0,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create BACKGROUND in HANDLE
    let _background = Widget::new_in(&mut system, &_handle, "Background", Layout::Solid {
        width: 2560,
        height: 1440,
        scaling: Scale::Fill,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create BOARD in WIDGET
    let _board = Widget::new_in(&mut system, &_app, "Board", Layout::Solid {
        width: 807,
        height: 1432,
        horizontal_anchor: -0.80,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create un-named widget in BOARD (useful when widget is not important and is used for layout position only (no image, not interactive), helps with clarity)
    let _logo_boundary = Widget::new_in(&mut system, &_board, "", Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: 70.0 },
        relative_2: Vec2 { x: 105.0, y: 85.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create LOGO in un-named widget and register LOGO under BOARD (it will be Board/Logo instead Board/un-named/Logo)
    let _logo = Widget::new_in(&mut system, &_board, "#p0/Logo", Layout::Solid {
        width: 681,
        height: 166,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create LOGOSHADOW in LOGO
    let _logo_boundary = Widget::new_in(&mut system, &_logo, "LogoShadow", Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: -10.0 },
        relative_2: Vec2 { x: 105.0, y: 110.0 },
        ..Default::default()
    }.wrap()).unwrap();


    //################################################################################
    //# == Button Layout ==
    //# Here we will create a ButtonList widget which will contain all the buttons.

    //# Create BUTTONLIST in BOARD
    let _button_list = Widget::new_in(&mut system, &_board, "ButtonList", Layout::Relative {
        relative_1: Vec2 { x: 17.0, y: 21.0 },
        relative_2: Vec2 { x: 82.0, y: 66.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create a list with names for iteration
    let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    
    //# Create buttons in BUTTONLIST
    let step = 2.0/button_list.len() as f32;        //Distribute the containers equally
    for i in 0..button_list.len() {


        //# Create a BUTTON widget that will be used for input detection
        let button = Widget::new_in(&mut system, &_button_list, button_list[i], Layout::Solid {
            width: 532,
            height: 75,
            scaling: Scale::Fit,
            vertical_anchor: 1.0 - step * i as f32,      //Where should the container be on Y axis (range: 1.0 to -1.0)
            ..Default::default()
        }.wrap()).unwrap();

        //# Create a button for DECORATION that will be animated
        let button_decor = Widget::new_in(&mut system, &button, "", Layout::Window {
            width_relative: 100.0,
            height_relative: 100.0,
            ..Default::default()
        }.wrap()).unwrap();

        //# Create a data stored in hierarchy for sharing
        let data = button_decor.fetch_mut(&mut system, "").unwrap().data_get_mut();
        *data = Option::Some(Box::new(MainMenuButtonDecoration {alpha: 0.0}));
    }

    //################################################################################
    //# == Hierarchy Debug ==
    //# This will print out both "normal" and "debug" maps (It's like "ls" command on Linux). The difference is that "debug" will also print out "nameless" widgets.
    //# "Nameless" widgets are hidden because they are NOT IMPORTANT to the main functionality of the system, but are there only for layout purposes.
    //# Displaying them would be considered overwhelming.

    println!("{}", system.map_debug());
    println!("{}", system.map());

    system
}


//################################################################################
//# == Button Logic ==
//# These two components and systems do all the button logic. Due to Bevy ECS, there is no "clean" way of modifying values of specific entities. (At least that I am not aware of)
//# So because each of the buttons are made of a mix of 2 entities that interact between each other, I save the changes of one entity to the Hierarchy as metadata and the other
//# entity fetches that data and synchronizes itself. This way there is a direct access to data, no looping over querries and finding corresponding entity, etc.
//# Might not be as much of an ECS solution as people want but it works and it is nice and simple. Sometimes mix of both worlds is the best solution.

//# The main entitity that will interact with cursor (Hitbox)
#[derive(Component)]
pub struct MainMenuButton ();
fn button_update(mut systems: Query<&mut Hierarchy>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &MainMenuButton)>) {
    
    //# Get Hierarchy and cursor
    let mut system = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();

    //# Loop through all widgets in the query (MainMenuButton)
    for (widget, _) in &mut query {

        //# Check if the cursor is within the current widget boundaries
        if widget.is_within(&system, "", cursor.position_screen()).unwrap(){
            
            //# Fetch the nameless widget layout from Hierarchy and update it (Smooth animation of the decoration widget)
            let window = widget.fetch_layout_mut(&mut system, "#p0").unwrap().expect_window_mut();
            window.relative.x = 5.0;

            //# Fetch the nameless widget data from Hierarchy and update it (Image alpha of the decoration widget)
            match widget.fetch_mut(&mut system, "#p0").unwrap().data_get_mut() {
                Option::None => (),
                Option::Some ( _box ) => {
                    _box.set_f32(0.4);
                }
            }

        } else {
            //# Fetch the nameless widget layout from Hierarchy and update it (Smooth animation of the decoration widget)
            let window = widget.fetch_layout_mut(&mut system, "#p0").unwrap().expect_window_mut();
            if window.relative.x > 0.0 {window.relative.x -= 1.0;} else {window.relative.x = 0.0;}
        }
    }
}

//# The secondary entity that will get updated by the main entity
#[derive(Component)]
pub struct MainMenuButtonDecoration { pub alpha: f32 }
impl Data for MainMenuButtonDecoration {
    fn get_f32 (&self) -> f32 {
        self.alpha
    }
    fn set_f32 (&mut self, value: f32) {
        self.alpha = value;
    }
}
fn button_update_decoration(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Sprite, &MainMenuButtonDecoration)>) {
    
    //# Get Hierarchy
    let mut system = systems.get_single_mut().unwrap();

    //# Loop through all widgets in the query (MainMenuButtonDecoration)
    for (widget, mut sprite,  _) in &mut query {

        //# Fetch the current widget data from Hierarchy and synchronize itself (Image alpha of the sprite)
        match widget.fetch_mut(&mut system, "").unwrap().data_get_mut() {
            Option::None => (),
            Option::Some ( _box ) => {
                let mut alpha = _box.get_f32();
                if alpha > 0.0 {alpha -= 0.01} else {alpha = 0.0}
                _box.set_f32(alpha);
                sprite.color.set_a(alpha);
            }
        }
    }
}

//# Wrap it into plugin for code clarity
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(button_update)
            .add_system(button_update_decoration);
    }
}