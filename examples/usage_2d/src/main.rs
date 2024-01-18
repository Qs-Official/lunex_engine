use bevy::prelude::*;
use bevy_lunex::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {

    cmd.spawn(Camera2dBundle { transform: Transform { translation: Vec3::new(0.0, 0.0, 100.0), ..default() }, ..default() });

    cmd.spawn((
        MyWidget,
        UiTree::<NoData>::new("UI"),
        //UiLogic::build(), // Needs direct link at UiTree
        //Transform, rendering, etc
    ));

    // This entity needs to be spawn as child
    cmd.spawn((
        MyWidget,
        UiLink::path("window"),
        //UI::Window::FULL,
    ));

    // BSN macro here?
    /*
    
    bsn! {

        let ctx1 = make!(path: "window", bundle: impl bundle){

            //Rust code

            let ctx2 = make!("window", impl bundle){

                //Rust code

            }

        }//Push ctx1.push_kid(ctx2)

    }


    bsn! {
        
        div!("window", ( Middle, Bold, Rainbow )){

            //Rust code

            div!("window", ( Middle, Bold, Rainbow )){

                //Rust code

            }
        }
    }
     */

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;