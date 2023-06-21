#![allow(dead_code)]
#![allow(unused_variables)]

use crate::library::prelude::{Ui, MString, Outcome};
use bevy::prelude::*;

use super::ui_container::ContainerPosition;

#[derive(Component)]
pub struct Widget {
    pub path: String,
}
impl Widget {

    pub fn position<'a> (&'a self, system: &'a  Ui::Hiearchy) -> Result<&ContainerPosition, String> {
        match system.expose().borrow_chain_checked(&self.path){
            Ok (reference) => Result::Ok(reference.container_get()),
            Err (message) => Err(String::from("WIDGET '") + &self.path + "' NOT FOUND! #Error: "+ &message),
        }
    }



    fn new_empty () -> Widget {
        Widget {
            path: String::new(),
        }
    }
    fn from_path (path: String) -> Widget {
        Widget { 
            path,
        }
    }

    pub fn new(system: &mut Ui::Hiearchy, key: &str, position: Ui::PositionType) -> Result<Widget, String> {
        match system.expose_mut().create_simple_checked(key, position) {
            Ok (new_key) => Result::Ok(Widget::from_path(new_key)),
            Err (message) => Err(String::from("UNABLE TO CREATE WIDGET! #Error: ") + &message),
        }
    }
    pub fn key(&self) -> String {
        String::from("#ROOT/") + &self.path + "/"
    }
    pub fn new_in(system: &mut Ui::Hiearchy, widget: &Widget, key: &str, position: Ui::PositionType) -> Result <Widget, String> {
        match key.split_once('/') {
            None => {
                match system.expose_mut().borrow_chain_checked_mut(&widget.path){
                    Ok (reference) => match reference.create_simple_checked(key, position) {
                        Ok (new_key) => Result::Ok(Widget::from_path(String::new() + &widget.path + "/"+ &new_key)),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Err(String::from("WIDGET '") + &widget.path + "' NOT FOUND! #Error: "+ &message),
                }
            },
            Some (tuple) => {
                let mut path = String::new();
                let tuple1 = MString::split_last(key,"/");       // xxx/xxx/xxx - yyy
                let is_rooted:bool = match tuple1.0.split_once('/'){             // xxx - xxx/xxx
                    Some (tuple2) => if tuple2.0 == "#ROOT" {path += tuple2.1;true} else {path += &tuple1.0;false},
                    None =>  if tuple1.0 == "#ROOT" {true} else {path += &tuple1.0;false},
                };

                if is_rooted {
                    if path.is_empty() {
                        Result::Err(String::from("THIS KEY IS ILLEGAL!"))
                    } else {
                        let source = match system.expose_mut().translate_chain_checked(&path){
                            Ok (source) => source,
                            Err (message) => return Result::Err(message),
                        };
                        let substring = match system.expose_mut().translate_chain_checked(&widget.path){
                            Ok (substring) => substring,
                            Err (message) => return Result::Err(message),
                        };

                        let _path = MString::subtract_void(&source, &substring);

                        let new_key: String = match system.expose_mut().borrow_chain_mut(&source) {
                            Ok (set_widget) => {
                                set_widget.create_simple(true, position)
                            },
                            Err (message) => return Result::Err(message),
                        };
                        match system.expose_mut().borrow_chain_mut(&substring) {
                            Ok (register_widget) => {
                                register_widget.register_path(String::from(&tuple1.1), _path + "/" + &new_key)
                            },
                            Err (message) => return Result::Err(message),
                        };

                        Result::Ok(Widget::from_path(String::new() + &widget.path + "/" + &tuple1.1))
                    }
                } else {
                    let mut new_local_path = String::from(&path) + "/";
                    match system.expose_mut().borrow_chain_checked_mut(&(String::from(&widget.path)+ "/" + &path)) {
                        Ok (set_widget) => {
                            new_local_path += &set_widget.create_simple(true, position);
                        },
                        Err (message) => return Err(String::from("CRASHED ON MAKING NEW BRANCH! #Error: ") + &message),
                    };
                    match system.expose_mut().borrow_chain_checked_mut(&widget.path) {
                        Ok (register_widget) => {
                            register_widget.register_path(String::from(&tuple1.1), new_local_path.clone())
                        },
                        Err (message) => return Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
                    };
                    Result::Ok(Widget::from_path(String::new() + &widget.path + "/" + &tuple1.1))
                }
            },
        }
    }
    
    pub fn map (&self, system: & Ui::Hiearchy) -> Result<String, String> {
        match system.expose().borrow_chain_checked(&self.path){
            Ok (reference) => {
                let list: Vec<&str> =  self.path.split('/').collect();
                let mut string = String::from(list[list.len()-1]);
                string = reference.map(string, 0);
                Ok(string)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn map_debug (&self, system: & Ui::Hiearchy) -> Result<String, String> {
        match system.expose().borrow_chain_checked(&self.path){
            Ok (reference) => {
                let list: Vec<&str> =  self.path.split('/').collect();
                let mut string = String::from(list[list.len()-1]);
                string = reference.map_debug(string, 0);
                Ok(string)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn destroy (&self, system: &mut Ui::Hiearchy, path : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.destroy_chain_checked(path)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn remove (&self, system: &mut Ui::Hiearchy, key : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.remove_simple_checked(key)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }

    pub fn exist (&self, system: &mut Ui::Hiearchy, key : &str) -> bool{
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        system.expose_mut().check_chain_checked(&path)
        
    }

    /*pub fn container_get<'a> (&'a self, system: &'a mut Ui::System, key : &str) -> Result<Ui::Container, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (reference) => {
                Ok(reference.container_get())
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }*/
    
    // CONVERT ABSOLUTE INTO RELATIVE
    /*pub fn container_absolute_to_relative<'a> (&'a self, system: &'a mut Ui::System, key : &str, point: [f32;2]) -> Result<[f32; 2], String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (reference) => {
                let container = reference.container_get();
                Ok([(point[0]-container.parent_parameters.0[0]) * 100.0 / container.parent_parameters.1, (point[1]-container.parent_parameters.0[1]) * 100.0 / container.parent_parameters.2])
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }*/
    
    


    pub fn position_add (&self, system: &mut Ui::Hiearchy, name: &str, position: Ui::PositionType, key: &str) -> Outcome {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked_mut(&path){
            Ok (branch) => {
                branch.container_position_add(name, position)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow<'a> (&'a self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<& Ui::PositionType, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (branch) => {
                branch.container_position_borrow(name)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_mut<'a> (&'a mut self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<&mut Ui::PositionType, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked_mut(&path){
            Ok (branch) => {
                branch.container_position_borrow_mut(name)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    
    pub fn position_borrow_solid<'a> (&'a self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<& Ui::Pos::Solid, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (branch) => {
                match branch.container_position_borrow(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Solid (window) => Ok(&window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A SOLID TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_window<'a> (&'a self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<& Ui::Pos::Window, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (branch) => {
                match branch.container_position_borrow(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Window (window) => Ok(&window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A WINDOW TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_relative<'a> (&'a self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<& Ui::Pos::Relative, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked(&path){
            Ok (branch) => {
                match branch.container_position_borrow(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Relative (window) => Ok(&window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A RELATIVE TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_solid_mut<'a> (&'a mut self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<&mut Ui::Pos::Solid, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked_mut(&path){
            Ok (branch) => {
                match branch.container_position_borrow_mut(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Solid (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A SOLID TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_window_mut<'a> (&'a mut self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<&mut Ui::Pos::Window, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked_mut(&path){
            Ok (branch) => {
                match branch.container_position_borrow_mut(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Window (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A WINDOW TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_relative_mut<'a> (&'a mut self, system: &'a mut Ui::Hiearchy, name: &str, key: &str) -> Result<&mut Ui::Pos::Relative, String> {
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        match system.expose_mut().borrow_chain_checked_mut(&path){
            Ok (branch) => {
                match branch.container_position_borrow_mut(name) {
                    Ok (position) => {
                        match position {
                            Ui::PositionType::Relative (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A RELATIVE TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }

    //NOT FINISHED PAST THIS

}