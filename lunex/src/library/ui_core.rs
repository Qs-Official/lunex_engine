#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

use crate::library::prelude::{HashMap, Outcome, Ui};
use crate::library::ui_container::Container;

use super::ui_container::ContainerPosition;


//=====================================
//#Lunex = Main UI struct
//------------------
#[derive(Component)]
pub struct Hiearchy {
    pub width: f32,
    pub height: f32,
    branch: Branch,
}
impl Hiearchy {
    pub fn new () -> Hiearchy {                   //CREATES NEW UI SYSTEM
        let mut branch = Branch::new();
        branch.container.position_add("default", Ui::Pos::Relative {
            point_relative_1: [0.0, 0.0],
            point_relative_2: [100.0, 100.0],
            ..Default::default()
        }.wrap());

        Hiearchy {
            width: 0.0,
            height: 0.0,
            branch,
        }
    }
    pub fn calculate (&mut self) {
        self.branch.container_calculate([0.0,0.0], self.width, self.height);
    }
    pub fn draw (&self) {
        //self.expose().draw(&self.sprite_vault);
    }
    pub fn map (&self) -> String {
        let mut string = String::from("#HIEARCHY");
        string = self.branch.map(string, 0);
        string
    }
    pub fn map_debug (&self) -> String {
        let mut string = String::from("#HIEARCHY");
        string = self.branch.map_debug(string, 0);
        string
    }
    
    pub (in crate) fn expose (&self) -> & Branch {
        & self.branch
    }
    pub (in crate) fn expose_mut (&mut self) -> &mut Branch {
        &mut self.branch
    }
    
}

pub fn hiearchy_update(mut query: Query<&mut Hiearchy>, mut windows: Query<&mut Window>) {
    let window = windows.get_single_mut().unwrap();
    for mut system in &mut query {
        system.width = window.resolution.width();
        system.height = window.resolution.height();

        system.calculate();
    }
}


pub (in crate) struct Branch {
    container: Container,

    pernament: Vec<Branch>,
    removable: HashMap<usize, Branch>,
    register: HashMap<String, String>,
}
impl Branch {
    //#LIBRARY INITIATE
    fn new () -> Branch {
        Branch {
            container: Container::new(),

            pernament: Vec::new(),
            removable: HashMap::new(),
            register: HashMap::new(),
        }
    }
    
    //#LIBRARY CONTROL
    pub (in crate) fn container_get (&self) -> &ContainerPosition {                                                                 //This will spit out copy instead of borrow of the container to avoid concurency
        self.container.position()
    }

    pub (in crate) fn container_position_add (&mut self, key: &str, position: Ui::PositionType) -> Outcome {                        //Use this to add more positions to the container
        self.container.position_add(key, position)
    }
    pub (in crate) fn container_position_borrow (&self, key: &str) -> Result<& Ui::PositionType, String> {                          //This will give you pointer to a position
        self.container.position_borrow(key)
    }
    pub (in crate) fn container_position_borrow_mut (&mut self, key: &str) -> Result<&mut Ui::PositionType, String> {               //This will give you mutable pointer to a position
        self.container.position_borrow_mut(key)
    }
    
    //#LIBRARY TICK
    pub (in crate) fn container_calculate (&mut self, point: [f32; 2], width: f32, height: f32) {                                   //This will calculate container and enter reccursion
        self.container.calculate(point, width, height);
        for i in 0..self.pernament.len() {
            let pos = self.container.position();
            self.pernament[i].container_calculate(pos.point_1, pos.width, pos.height);
        }
        for x in self.removable.iter_mut(){
            let pos = self.container.position();
            x.1.container_calculate(pos.point_1, pos.width, pos.height);
        }
    }
    
    //#LIBRARY WORKINGS
    pub (in crate) fn create_simple (&mut self, removable: bool, position: Ui::PositionType) -> String {                            //This creates unnamed Branch in one of the 2 registers and return string with ABSOLUTE local path
        if !removable {
            let ukey = self.pernament.len();
            let mut branch = Branch::new();
            branch.container.position_add("default", position);
            self.pernament.push(branch);
            String::from("#p") + &ukey.to_string()
        } else {
            let mut ukey = 0;
            loop {
                if !self.removable.contains_key(&ukey) {break;};
                ukey += 1;
            };
            let mut branch = Branch::new();
            branch.container.position_add("default", position);
            self.removable.insert(ukey, branch);
            String::from("#r") + &ukey.to_string()
        }
    }
    pub (in crate) fn create_simple_checked (&mut self, key: &str, position: Ui::PositionType) -> Result<String, String> {          //This decides if Branch should be removable or not and also checks for key collision and returns ABSOLUTE/RELATIVE local path
        if key.is_empty() {
            Result::Ok(self.create_simple(false, position))
        } else {
            match self.register.get(key){
                None => {
                    let path = self.create_simple(true, position);
                    self.register_path(String::from(key), path);
                    Result::Ok(String::from(key))
                },
                Some (..) => Result::Err(format!("The key '{}' is already in use!", &key).to_string()),
            }
        }
    }

    pub (in crate) fn register_path (&mut self, key: String, path: String){                                                         //This registers ABSOLUTE PATH for a key
        self.register.insert(key, path);
    }

    pub (in crate) fn translate_simple (&self, key: &str) -> Result<String, String> {                                               //This can take ONLY RELATIVE and return ABSOLUTE
        match self.register.get(key) {
            Some (value) => Result::Ok(String::from(value)),
            None => Result::Err(format!("The key '{}' is not in the register!", &key).to_string()),
        }
    }
    pub (in crate) fn translate_simple_checked (&self, key: &str) -> Result<String, String> {                                       //This can take RELATIVE/ABSOLUTE and return ABSOLUTE
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => Result::Ok(key.to_owned()),
                _ => self.translate_simple(key),
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate) fn translate_chain (&self, keypath: &str) -> Result<String, String> {                                            //This can take chained RELATIVE path and return ABSOLUTE
        match keypath.split_once('/') {
            None => {
                self.translate_simple(keypath)
            },
            Some (tuple) => match self.translate_simple(tuple.0) {
                Ok (new_key) => match self.borrow_simple(&new_key) {
                    Ok (borrowed_widget) => match borrowed_widget.translate_chain(tuple.1) {
                        Ok (path_result) => Result::Ok(new_key.to_owned() + "/" + &path_result),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Result::Err(message),
                },
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate) fn translate_chain_checked (&self, keypath: &str) -> Result<String, String> {                                    //This can take chained RELATIVE/ABSOLUTE path and return ABSOLUTE
        match keypath.split_once('/') {
            None => {
                self.translate_simple_checked(keypath)
            },
            Some (tuple) => match self.translate_simple_checked(tuple.0) {
                Ok (new_key) => match self.borrow_simple_checked(&new_key) {
                    Ok (borrowed_widget) => match borrowed_widget.translate_chain_checked(tuple.1) {
                        Ok (path_result) => Result::Ok(new_key.to_owned() + "/" + &path_result),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Result::Err(message),
                },
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate) fn borrow_simple (&self, path: &str) -> Result<&Branch, String> {                                                //This can take ONLY ABSOLUTE and return reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return Result::Err(format!("Pernament branch with index '{}' does not exist!", &index).to_string());
                                };
                                Result::Ok(&self.pernament[index])
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get(&index) {
                                    Some (widget) => {
                                        Result::Ok(widget)
                                    },
                                    None => Result::Err(format!("Removable branch with path '{}' does not exist!", &index).to_string()),
                                }
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Result::Err(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Result::Err(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate) fn borrow_simple_checked (&self, key: &str) -> Result<&Branch, String> {                                         //This can take RELATIVE/ABSOLUTE and return reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.borrow_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.borrow_chain_checked(&new_key),
                    Err (message) => Result::Err(message),
                },
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate) fn borrow_chain (&self, path: &str) -> Result<&Branch, String> {                                                 //This can take chained ABSOLUTE path and return reference
        match path.split_once('/') {
            None => {
                self.borrow_simple(path)
            },
            Some (tuple) => match self.borrow_simple(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate) fn borrow_chain_checked (&self, keypath: &str) -> Result<&Branch, String> {                                      //This can take chained ABSOLUTE/RELATIVE path and return reference
        match keypath.split_once('/') {
            None => {
                self.borrow_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate) fn borrow_simple_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                    //This can take ONLY ABSOLUTE and return MUT reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return Result::Err(format!("Pernament branch with index '{}' does not exist!", &index).to_string());
                                };
                                Result::Ok(&mut self.pernament[index])
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get_mut(&index) {
                                    Some (widget) => {
                                        Result::Ok(widget)
                                    },
                                    None => Result::Err(format!("Removable branch with path '{}' does not exist!", &index).to_string()),
                                }
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Result::Err(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Result::Err(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate) fn borrow_simple_checked_mut (&mut self, key: &str) -> Result<&mut Branch, String> {                             //This can take RELATIVE/ABSOLUTE and return MUT reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.borrow_simple_mut(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.borrow_chain_checked_mut(&new_key),
                    Err (message) => Result::Err(message),
                },
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate) fn borrow_chain_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                     //This can take chained ABSOLUTE path and return MUT reference
        match path.split_once('/') {
            None => {
                self.borrow_simple_mut(path)
            },
            Some (tuple) => match self.borrow_simple_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_mut(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate) fn borrow_chain_checked_mut (&mut self, keypath: &str) -> Result<&mut Branch, String> {                          //This can take chained ABSOLUTE/RELATIVE path and return MUT reference
        match keypath.split_once('/') {
            None => {
                self.borrow_simple_checked_mut(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked_mut(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate) fn check_simple (&self, path: &str) -> bool {                                                                    //This can take ONLY ABSOLUTE and return reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return false;
                                };
                                true
                            },
                            Err (..) => false,
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get(&index) {
                                    Some (widget) => true,
                                    None => false,
                                }
                            },
                            Err (..) => false,
                        }
                    },
                    _ => false,
                }
            },
            None => false,
        }
    }
    pub (in crate) fn check_simple_checked (&self, key: &str) -> bool {                                                             //This can take RELATIVE/ABSOLUTE and return reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.check_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.check_chain_checked(&new_key),
                    Err (message) => false,
                },
            }
            None => false,
        }
    }
    pub (in crate) fn check_chain (&self, path: &str) -> bool {                                                                     //This can take chained ABSOLUTE path and return reference
        match path.split_once('/') {
            None => {
                self.check_simple(path)
            },
            Some (tuple) => match self.borrow_simple(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.check_chain(tuple.1),
                Err (..) => false,
            },
        }
    }
    pub (in crate) fn check_chain_checked (&self, keypath: &str) -> bool {                                                          //This can take chained ABSOLUTE/RELATIVE path and return reference
        match keypath.split_once('/') {
            None => {
                self.check_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.check_chain_checked(tuple.1),
                Err (..) => false,
            },
        }
    }

    pub (in crate) fn destroy_simple (&mut self, path: &str) -> Outcome {                                                           //This can take ONLY ABSOLUTE and return Option if the destruction succeded
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => Outcome::Fail(String::from("Widgets with no name are supposed to be permanent and cannot be destroyed directly!")),
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if !self.removable.contains_key(&index) {
                                    return Outcome::Fail(format!("Removable branch with key '{}' does not exist!", &index).to_string());
                                }
                                self.removable.remove(&index);
                                Outcome::Pass
                            },
                            Err (..) => Outcome::Fail(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Outcome::Fail(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Outcome::Fail(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate) fn destroy_simple_checked (&mut self, key: &str) -> Outcome {                                                    //This can take RELATIVE/ABSOLUTE and return Option if the destruction succeded
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.destroy_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.destroy_chain(&new_key),
                    Err (message) => Outcome::Fail(message),
                },
            }
            None => Outcome::Fail(String::from("There is no key!")),
        }
    }
    pub (in crate) fn destroy_chain (&mut self, path: &str) -> Outcome {                                                            //This can take chained ABSOLUTE path and return Option if the destruction succeded
        match path.split_once('/') {
            None => {
                self.destroy_simple(path)
            },
            Some (tuple) => match self.borrow_simple_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.destroy_chain(tuple.1),
                Err (message) => Outcome::Fail(message),
            },
        }
    }
    pub (in crate) fn destroy_chain_checked (&mut self, keypath: &str) -> Outcome {                                                 //This can take chained ABSOLUTE/RELATIVE path and return Option if the destruction succeded
        match keypath.split_once('/') {
            None => {
                self.destroy_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.destroy_simple_checked(tuple.1),
                Err (message) => Outcome::Fail(message),
            },
        }
    }

    pub (in crate) fn remove_simple_checked (&mut self, key: &str) -> Outcome {                                                     //This can take ONLY RELATIVE and return Option if the widget was destroyed and removed from register
        if self.register.contains_key(key) {
            match self.destroy_chain_checked(key) {
                Outcome::Pass => {
                    self.register.remove(key);
                    Outcome::Pass
                },
                Outcome::Fail (message) => Outcome::Fail(message),
            }
        } else {
            Outcome::Fail(format!("Widget registered as '{}' does not exist!", &key).to_string())
        }
    }

    pub (in crate) fn map (&self, mut string: String, level: u32) -> String {
        for x in self.register.iter(){
            if let Ok (widget) = self.borrow_chain_checked(x.1){
                string += "\n  ";
                for _x in 0..level {
                    string += "|    ";
                };
                string += "|-> ";
                string += x.0;
                string = widget.map(string, level + 1);
            }
        }
        string
    }
    pub (in crate) fn map_debug (&self, mut string: String, level: u32) -> String {
        let mut done_widgets: HashMap<String, bool> = HashMap::new();
        for x in self.register.iter(){
            match self.borrow_chain_checked(x.1){
                Ok (widget) => {
                    string += "\n  ";
                    for _x in 0..level {
                        string += "|    ";
                    }
                    string += "|-> ";
                    string += x.0;
                    string += " (";
                    string += x.1;
                    string += ")";
                    string = widget.map_debug(string, level + 1);
                    done_widgets.insert(x.1.to_string(), true);
                },
                Err(..) => {
                    string += "\n  ";
                    for _x in 0..level {
                        string += "|    ";
                    }
                    string += "|-> ";
                    string += x.0;
                    string += " #[! Dangling pointer !]";
                },
            }
        }
        for i in 0..self.pernament.len() {
            if done_widgets.contains_key( &("#p".to_string() + &i.to_string())) {
                continue;
            }
            string += "\n  ";
            for _x in 0..level {
                string += "|    ";
            }
            string += "|-> #p";
            string += &i.to_string();
            string = self.pernament[i].map_debug(string, level + 1);
        }
        for x in self.removable.iter(){
            if done_widgets.contains_key( &("#r".to_string() + &x.0.to_string())) {
                continue;
            }
            string += "\n  ";
            for _x in 0..level {
                string += "|    ";
            }
            string += "|-> #r";
            string += &x.0.to_string();
            string = x.1.map_debug(string, level + 1);
        }
        string
    }
    
}

