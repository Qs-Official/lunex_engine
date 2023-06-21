#![allow(dead_code)]
#![allow(unused_variables)]


pub mod information {

    #[derive(Default, Clone)]
    pub struct Info {
        pub text: String,
    }
    impl Info {
        pub fn new() -> Info {
            Info {
                text: String::new(),
            }
        }
    }
}


// #THE CONTAINER ===================================
pub mod container {
    use crate::library::prelude::HashMap;

    #[derive(Default, Clone)]
    pub enum SolidSize {
        #[default]
        Fit,
        Fill,
    }
    #[derive(Default, Clone)]
    pub struct Window {
        pub point_absolute: [f32; 2],
        pub point_relative: [f32; 2],
        pub width_absolute: f32,
        pub width_relative: f32,
        pub height_absolute: f32,
        pub height_relative: f32,
        pub origin: [f32; 2],
        //Rotation [f32, f32, f32]
    }
    impl Window {
        pub fn new () -> Window {
            Window {
                point_absolute: [0.0,0.0],
                point_relative: [0.0,0.0],
                width_absolute: 0.0,
                width_relative: 0.0,
                height_absolute: 0.0,
                height_relative: 0.0,
                origin: [50.0, 50.0],
            }
        }
        pub fn wrap (self) -> PositionType {
            PositionType::Window(self)
        }
        fn calculate (&self, point: [f32; 2], width: f32, height: f32) -> ([f32; 2], f32, f32) {
            let xs = width / 100.0;
            let ys = height / 100.0;
            (
                [point[0] + self.point_absolute[0] + (self.point_relative[0] * xs), point[1] + self.point_absolute[1] + (self.point_relative[1] * ys)],
                self.width_absolute + (self.width_relative * xs),
                self.height_absolute + (self.height_relative * ys),
            )
        }
    }
    #[derive(Default, Clone)]
    pub struct Relative {
        pub point_absolute_1: [f32; 2],
        pub point_absolute_2: [f32; 2],
        pub point_relative_1: [f32; 2],
        pub point_relative_2: [f32; 2],
        pub origin: [f32; 2],
    }
    impl Relative {
        pub fn new () -> Relative {
            Relative {
                point_absolute_1: [0.0,0.0],
                point_absolute_2: [0.0,0.0],
                point_relative_1: [0.0,0.0],
                point_relative_2: [0.0,0.0],
                origin: [50.0, 50.0],
            }
        }
        pub fn wrap (self) -> PositionType {
            PositionType::Relative(self)
        }
        fn calculate (&self, point: [f32; 2], width: f32, height: f32) -> [[f32; 2];2] {
            let xs = width / 100.0;
            let ys = height / 100.0;
            [
                [point[0] + self.point_absolute_1[0] + (self.point_relative_1[0] * xs), point[1] + self.point_absolute_1[1] + (self.point_relative_1[1] * ys)],
                [point[0] + self.point_absolute_2[0] + (self.point_relative_2[0] * xs), point[1] + self.point_absolute_2[1] + (self.point_relative_2[1] * ys)],
            ]
        }
    }
    #[derive(Default, Clone)]
    pub struct Solid {
        pub width: u32,
        pub height: u32,
        pub horizontal_anchor: f32,
        pub vertical_anchor: f32,
        pub size: SolidSize,
        pub origin: [f32; 2],
    }
    impl Solid {
        pub fn new () -> Solid {
            Solid {
                width: 0,
                height: 0,
                horizontal_anchor: 0.0,
                vertical_anchor: 0.0,
                size: SolidSize::Fit,
                origin: [50.0, 50.0],
            }
        }
        pub fn wrap (self) -> PositionType {
            PositionType::Solid(self)
        }
        fn calculate (&self, point: [f32; 2], width: f32, height: f32) -> ([f32; 2], f32, f32) {
            let scale = match self.size {
                SolidSize::Fill => f32::max(width/self.width as f32, height/self.height as f32),
                SolidSize::Fit => f32::min(width/self.width as f32, height/self.height as f32),
            };

            let center = [point[0] + width/2.0, point[1] + height/2.0];
            let vanilla_width = self.width as f32*scale;
            let vanilla_height = self.height as f32*scale;
            let vanilla_point = [center[0] - vanilla_width/2.0, center[1] - vanilla_height/2.0];

            (
                [(vanilla_point[0] + (vanilla_point[0] - point[0])*self.horizontal_anchor),(vanilla_point[1] + (vanilla_point[1] - point[1])*self.vertical_anchor)],
                vanilla_width,
                vanilla_height,
            )
        }
    }
    
    #[derive(Clone)]
    pub enum PositionType {
        Window (Window),
        Relative (Relative),
        Solid (Solid),
    }

    pub (in super) struct Container {
        pub point_1: [f32; 2],                                      //READ-ONLY   -   For changing this you need to edit actual position
        pub point_2: [f32; 2],                                      //READ-ONLY   -   For changing this you need to edit actual position
        pub width: f32,                                             //READ-ONLY   -   For changing this you need to edit actual position
        pub height: f32,                                            //READ-ONLY   -   For changing this you need to edit actual position
        pub depth: f32,                                             //READ-ONLY   -   For changing this you need to edit actual position
        pub origin: [f32; 2],                                       //READ-ONLY   -   For changing this you need to edit actual position

        pub parent_parameters: ([f32;2], f32, f32),

        pub position: HashMap<String, PositionType>,
    }
    impl Container {
        pub fn new () -> Container {
            Container {
                point_1: [0.0,0.0],
                point_2: [0.0,0.0],
                width: 0.0,
                height: 0.0,
                depth: 0.0,
                origin: [50.0, 50.0],

                parent_parameters: ([0.0, 0.0], 0.0, 0.0),

                position: HashMap::new(),
            }
        }
        pub fn calculate (&mut self, point: [f32; 2], width: f32, height: f32) {
            if self.position.len() != 0 {

                let key = "default";
                match self.position.get(key){
                    None => {},
                    Some (position) => match position {
                        PositionType::Window(window) => {
                            let values = window.calculate(point, width, height);
                            self.point_1 = values.0;
                            self.width = values.1;
                            self.height = values.2;
                            self.point_2 = [self.point_1[0] + self.width, self.point_1[1] + self.height];
                            self.parent_parameters = (point, width, height);
                        },
                        PositionType::Relative(window) => {
                            let values = window.calculate(point, width, height);
                            self.point_1 = values[0];
                            self.width = values[1][0] - values[0][0];
                            self.height = values[1][1] - values[0][1];
                            self.point_2 = [self.point_1[0] + self.width, self.point_1[1] + self.height];
                            self.parent_parameters = (point, width, height);
                        },
                        PositionType::Solid(window) => {
                            let values = window.calculate(point, width, height);
                            self.point_1 = values.0;
                            self.width = values.1;
                            self.height = values.2;
                            self.point_2 = [self.point_1[0] + self.width, self.point_1[1] + self.height];
                            self.parent_parameters = (point, width, height);
                        },
                    },
                }
                
            }
        }
        pub fn clone (&self) -> ContainerOutput {
            ContainerOutput { point_1: self.point_1, point_2: self.point_2, width: self.width, height: self.height, depth: self.depth, origin: self.origin, parent_parameters: self.parent_parameters }
        }
    }
    
    pub struct ContainerOutput {
        pub point_1: [f32; 2],                                      //READ-ONLY   -   For changing this you need to edit actual position
        pub point_2: [f32; 2],                                      //READ-ONLY   -   For changing this you need to edit actual position
        pub width: f32,                                             //READ-ONLY   -   For changing this you need to edit actual position
        pub height: f32,                                            //READ-ONLY   -   For changing this you need to edit actual position
        pub depth: f32,                                             //READ-ONLY   -   For changing this you need to edit actual position
        pub origin: [f32; 2],                                       //READ-ONLY   -   For changing this you need to edit actual position
        pub parent_parameters: ([f32;2], f32, f32),
    }

    #[derive(Default)]
    pub (in super) struct MasterContainer {
        pub width: f32,
        pub height: f32,
    }
    impl MasterContainer {
        pub fn new () -> MasterContainer {
            MasterContainer {
                width: 0.0,
                height: 0.0,
            }
        }
        pub fn update (&mut self, width: f32, height: f32) {
            self.width = width;
            self.height = height;
        }
    }

}

// #THE WIDGET TREE ===================================
pub mod tree {
    use bevy::prelude::Component;

    use crate::library::prelude::{HashMap, Outcome, Ui};
    use super::container::{Container, ContainerOutput};
    use super::information::Info;
    pub (in super) struct Branch {
        container: Container,
        //stylisation: Option<HashMap<String, Style>>,
        information: Option<Info>,

        pernament: Vec<Branch>,
        removable: HashMap<usize, Branch>,
        register: HashMap<String, String>,
    }
    impl Branch {
        //#LIBRARY INITIATE
        fn new () -> Branch {
            Branch {
                container: Container::new(),
                //stylisation: Option::None,
                information: Option::None,

                pernament: Vec::new(),
                removable: HashMap::new(),
                register: HashMap::new(),
            }
        }
        
        //#LIBRARY CONTROL
        pub fn container_get (&self) -> ContainerOutput {                                                           //This will spit out copy instead of borrow of the container to avoid concurency
            self.container.clone()
        }

        pub fn container_position_add (&mut self, key: &str, position: Ui::Positions) -> Outcome {                  //Use this to add more positions to the container
            if !self.container.position.contains_key(key) {
                self.container.position.insert(String::from(key), position);
                Outcome::Pass
            } else {
                Outcome::Fail(String::from("The key '") + key + "' is already in use!")
            }
        }
        pub fn container_position_borrow (&self, key: &str) -> Result<& Ui::Positions, String> {                    //This will give you pointer to a position
            match self.container.position.get(key) {
                Some (position) => Result::Ok(position),
                None => Result::Err(String::from("Container does not have position '") + &key + "'!"),
            }
        }
        pub fn container_position_borrow_mut (&mut self, key: &str) -> Result<&mut Ui::Positions, String> {         //This will give you mutable pointer to a position
            match self.container.position.get_mut(key) {
                Some (position) => Result::Ok(position),
                None => Result::Err(String::from("Container does not have position '") + &key + "'!"),
            }
        }
        
        pub fn information_set (&mut self, info: Info) {
            self.information = Option::Some(info);
        }
        pub fn information_get (& self) -> Result<&Info, String> {
            match &self.information {
                Option::None => Result::Err(String::from("Widget does not contain any information!")),
                Option::Some (info) => Result::Ok(info),
            }
        }
        pub fn information_get_mut (&mut self) -> Result<&mut Info, String> {
            match &mut self.information {
                Option::None => Result::Err(String::from("Widget does not contain any information!")),
                Option::Some (info) => Result::Ok(info),
            }
        }


        //#LIBRARY TICK
        pub fn container_calculate (&mut self, point: [f32; 2], width: f32, height: f32) {                          //This will calculate container and enter reccursion
            self.container.calculate(point, width, height);
            for i in 0..self.pernament.len() {self.pernament[i].container_calculate(self.container.point_1, self.container.width, self.container.height);}
            for x in self.removable.iter_mut(){x.1.container_calculate(self.container.point_1, self.container.width, self.container.height);}
        }
        
        //#LIBRARY WORKINGS
        pub fn create_simple (&mut self, removable: bool, position: Ui::Positions) -> String {                      //This creates unnamed Branch in one of the 2 registers and return string with ABSOLUTE local path
            if !removable {
                let ukey = self.pernament.len();
                let mut branch = Branch::new();
                branch.container.position.insert(String::from("default"), position);
                self.pernament.push(branch);
                String::from("#p") + &ukey.to_string()
            } else {
                let mut ukey = 0;
                loop {
                    if !self.removable.contains_key(&ukey) {break;};
                    ukey += 1;
                };
                let mut branch = Branch::new();
                branch.container.position.insert(String::from("default"), position);
                self.removable.insert(ukey, branch);
                String::from("#r") + &ukey.to_string()
            }
        }
        pub fn create_simple_checked (&mut self, key: &str, position: Ui::Positions) -> Result<String, String> {    //This decides if Branch should be removable or not and also checks for key collision and returns ABSOLUTE/RELATIVE local path
            if key.is_empty() {
                Result::Ok(self.create_simple(false, position))
            } else {
                match self.register.get(key){
                    None => {
                        let path = self.create_simple(true, position);
                        self.register_path(String::from(key), path);
                        Result::Ok(String::from(key))
                    },
                    Some (..) => Result::Err(String::from("This key is already used!")),
                }
            }
        }

        pub fn register_path (&mut self, key: String, path: String){                                                //This registers ABSOLUTE PATH for a key
            self.register.insert(key, path);
        }

        pub fn translate_simple (&self, key: &str) -> Result<String, String> {                                      //This can take ONLY RELATIVE and return ABSOLUTE
            match self.register.get(key) {
                Some (value) => Result::Ok(String::from(value)),
                None => Result::Err(String::from("'") + key + "' is not in the register!"),
            }
        }
        pub fn translate_simple_checked (&self, key: &str) -> Result<String, String> {                              //This can take RELATIVE/ABSOLUTE and return ABSOLUTE
            match key.chars().next() {
                Some (_char) => match _char {
                    '#' => Result::Ok(key.to_owned()),
                    _ => self.translate_simple(key),
                }
                None => Result::Err(String::from("There is no key!")),
            }
        }
        pub fn translate_chain (&self, path: &str) -> Result<String, String> {                                      //This can take chained RELATIVE path and return ABSOLUTE
            match path.split_once('/') {
                None => {
                    self.translate_simple(path)
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
        pub fn translate_chain_checked (&self, path: &str) -> Result<String, String> {                              //This can take chained RELATIVE/ABSOLUTE path and return ABSOLUTE
            match path.split_once('/') {
                None => {
                    self.translate_simple_checked(path)
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

        pub fn borrow_simple (&self, key: &str) -> Result<&Branch, String> {                                        //This can take ONLY ABSOLUTE and return reference
            match key.chars().nth(1) {
                Some (value) => {
                    match value {
                        'p' => {
                            match str::parse::<usize>(&key[2..]) {
                                Ok (index) => {
                                    if index >= self.pernament.len() {
                                        return Result::Err(String::from("Pernament Branch with index '") + &index.to_string() + "' does not exist!");
                                    };
                                    Result::Ok(&self.pernament[index])
                                },
                                Err (..) => Result::Err(String::from("Error while parsing '") + key + "' (Needs to be a number)!"),
                            }
                        },
                        'r' => {
                            match str::parse::<usize>(&key[2..]) {
                                Ok (index) => {
                                    match self.removable.get(&index) {
                                        Some (widget) => {
                                            Result::Ok(widget)
                                        },
                                        None => Result::Err(String::from("Removable Branch with key '") + &index.to_string() + "' does not exist!"),
                                    }
                                },
                                Err (..) => Result::Err(String::from("Error while parsing '") + key + "' (Needs to be a number)!"),
                            }
                        },
                        _ => Result::Err(String::from("The 2nd char '") + &value.to_string() + "' in '" + key + "' needs to be either 'r' or 'p' (Identifies the storage stack)"),
                    }
                },
                None => Result::Err(String::from("Path '") + key + "' is missing information (Example: #r14)"),
            }
        }
        pub fn borrow_simple_checked (&self, key: &str) -> Result<&Branch, String> {                                //This can take RELATIVE/ABSOLUTE and return reference
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
        pub fn borrow_chain (&self, path: &str) -> Result<&Branch, String> {                                        //This can take chained ABSOLUTE path and return reference
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
        pub fn borrow_chain_checked (&self, path: &str) -> Result<&Branch, String> {                                //This can take chained ABSOLUTE/RELATIVE path and return reference
            match path.split_once('/') {
                None => {
                    self.borrow_simple_checked(path)
                },
                Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                    Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked(tuple.1),
                    Err (message) => Result::Err(message),
                },
            }
        }

        pub fn borrow_simple_mut (&mut self, key: &str) -> Result<&mut Branch, String> {                            //This can take ONLY ABSOLUTE and return MUT reference
            match key.chars().nth(1) {
                Some (value) => {
                    match value {
                        'p' => {
                            match str::parse::<usize>(&key[2..]) {
                                Ok (index) => {
                                    if index >= self.pernament.len() {
                                        return Result::Err(String::from("Pernament Branch with index '") + &index.to_string() + "' does not exist!");
                                    };
                                    Result::Ok(&mut self.pernament[index])
                                },
                                Err (..) => Result::Err(String::from("Error while parsing '") + key + "' (Needs to be a number)!"),
                            }
                        },
                        'r' => {
                            match str::parse::<usize>(&key[2..]) {
                                Ok (index) => {
                                    match self.removable.get_mut(&index) {
                                        Some (widget) => {
                                            Result::Ok(widget)
                                        },
                                        None => Result::Err(String::from("Removable Branch with key '") + &index.to_string() + "' does not exist!"),
                                    }
                                },
                                Err (..) => Result::Err(String::from("Error while parsing '") + key + "' (Needs to be a number)!"),
                            }
                        },
                        _ => Result::Err(String::from("The 2nd char '") + &value.to_string() + "' in '" + key + "' needs to be either 'r' or 'p' (Identifies the storage stack)"),
                    }
                },
                None => Result::Err(String::from("Path '") + key + "' is missing information (Example: #r14)"),
            }
        }
        pub fn borrow_simple_checked_mut (&mut self, key: &str) -> Result<&mut Branch, String> {                    //This can take RELATIVE/ABSOLUTE and return MUT reference
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
        pub fn borrow_chain_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                            //This can take chained ABSOLUTE path and return MUT reference
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
        pub fn borrow_chain_checked_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                    //This can take chained ABSOLUTE/RELATIVE path and return MUT reference
            match path.split_once('/') {
                None => {
                    self.borrow_simple_checked_mut(path)
                },
                Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                    Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked_mut(tuple.1),
                    Err (message) => Result::Err(message),
                },
            }
        }

        pub fn check_simple (&self, key: &str) -> bool {                                                            //This can take ONLY ABSOLUTE and return reference
            match key.chars().nth(1) {
                Some (value) => {
                    match value {
                        'p' => {
                            match str::parse::<usize>(&key[2..]) {
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
                            match str::parse::<usize>(&key[2..]) {
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
        pub fn check_simple_checked (&self, key: &str) -> bool {                                                    //This can take RELATIVE/ABSOLUTE and return reference
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
        pub fn check_chain (&self, path: &str) -> bool {                                                            //This can take chained ABSOLUTE path and return reference
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
        pub fn check_chain_checked (&self, path: &str) -> bool {                                                    //This can take chained ABSOLUTE/RELATIVE path and return reference
            match path.split_once('/') {
                None => {
                    self.check_simple_checked(path)
                },
                Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                    Ok (borrowed_widget) => borrowed_widget.check_chain_checked(tuple.1),
                    Err (..) => false,
                },
            }
        }

        pub fn destroy_simple (&mut self, path: &str) -> Outcome {                                                  //This can take ONLY ABSOLUTE and return Option if the destruction succeded
            match path.chars().nth(1) {
                Some (value) => {
                    match value {
                        'p' => Outcome::Fail(String::from("Unnamed widgets cannot be destroyed!")),
                        'r' => {
                            match str::parse::<usize>(&path[2..]) {
                                Ok (index) => {
                                    if !self.removable.contains_key(&index) {
                                        return Outcome::Fail(String::from("Branch does not exist!"));
                                    }
                                    self.removable.remove(&index);
                                    Outcome::Pass
                                },
                                Err (..) => Outcome::Fail(String::from("Invalid absolute path '") + path + "'!"),
                            }
                        },
                        _ => Outcome::Fail(String::from("The 2nd char '") + &value.to_string() + "' in '" + path + "' needs to be either 'r' or 'p' (Identifies the storage stack)"),
                    }
                },
                None => Outcome::Fail(String::from("Path '") + path + "' is missing information (Example: #r14)"),
            }
        }
        pub fn destroy_simple_checked (&mut self, key: &str) -> Outcome {                                           //This can take RELATIVE/ABSOLUTE and return Option if the destruction succeded
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
        pub fn destroy_chain (&mut self, path: &str) -> Outcome {                                                   //This can take chained ABSOLUTE path and return Option if the destruction succeded
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
        pub fn destroy_chain_checked (&mut self, path: &str) -> Outcome {                                           //This can take chained ABSOLUTE/RELATIVE path and return Option if the destruction succeded
            match path.split_once('/') {
                None => {
                    self.destroy_simple_checked(path)
                },
                Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                    Ok (borrowed_widget) => borrowed_widget.destroy_simple_checked(tuple.1),
                    Err (message) => Outcome::Fail(message),
                },
            }
        }

        pub fn remove (&mut self, key: &str) -> Outcome {                                                           //This can take ONLY ABSOLUTE and return Option if the widget was destroyed and removed
            if self.register.contains_key(key) {
                match self.destroy_chain_checked(key) {
                    Outcome::Pass => {
                        self.register.remove(key);
                        Outcome::Pass
                    },
                    Outcome::Fail (message) => Outcome::Fail(message),
                }
            } else {
                Outcome::Fail(String::from("'") + key + "' is not in the register!")
            }
        }

        pub fn map (&self, mut string: String, level: u32) -> String {
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
        pub fn map_debug (&self, mut string: String, level: u32) -> String {
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
        
        //#LIBRARY RENDER (Macroquad)
        /*pub fn draw_named (&self, vault: &Vault) {
            use macroquad::prelude::*;

            draw_rectangle_lines(self.container.point_1[0] as f32, self.container.point_1[1] as f32, self.container.width as f32, self.container.height as f32, 2.0, GREEN);

            for x in self.register.iter(){
                if let Ok (widget) = self.borrow_chain_checked(x.1){
                    widget.draw(vault);
                }
            }
        }
        pub fn draw (&self, vault: &Vault) {
            //use macroquad::prelude::*;
            //draw_rectangle_lines(self.container.point_1[0], self.container.point_1[1], self.container.width, self.container.height, 1.0, GREEN);
            match self.stylisation.as_ref() {
                Option::None => {},
                Option::Some(map) => {
                    match map.get("default"){
                        Option::None => {},
                        Option::Some (style) => {
                            //Draw the IMAGE
                            match &style.sprite_asset {
                                Option::Some (sprite) => vault.draw_sprite(sprite, self.container.point_1[0], self.container.point_1[1], self.container.width, self.container.height).unwrap(),
                                Option::None => {},
                            }

                            match self.information_get() {
                                Result::Ok (info) => {
                                    style.draw_text(vault, &info.text, self.container.point_1[0], self.container.point_1[1], self.container.width, self.container.height);
                                },
                                Result::Err(..) => {},
                            }
                        }
                    }
                }
            }
            //draw_rectangle_lines(self.container.point_1[0], self.container.point_1[1], self.container.width, self.container.height, 1.0, WHITE);
            for i in 0..self.pernament.len() {
                self.pernament[i].draw(vault);
            }
            for x in self.removable.iter(){
                x.1.draw(vault);
            }
        }*/
    }


    #[derive(Component)]
    pub struct System {
        width: f32,
        height: f32,
        branch: Branch,
    }
    impl System {
        pub fn new () -> System {                   //CREATES NEW UI SYSTEM
            let mut branch = Branch::new();
            branch.container.position.insert(String::from("default"), Ui::Pos::Relative {
                point_relative_1: [0.0, 0.0],
                point_relative_2: [100.0, 100.0],
                ..Default::default()
            }.wrap());

            System {
                width: 0.0,
                height: 0.0,
                branch,
            }
        }
        pub fn pull (&mut self) {                   //
            //use macroquad::prelude::*;
            //self.width = screen_width();
            //self.height = screen_height();
        }
        pub fn calculate (&mut self) {
            self.branch.container_calculate([0.0,0.0], self.width, self.height);
        }
        pub fn draw (&self) {
            //self.expose().draw(&self.sprite_vault);
        }
        pub fn map (&self) -> String {
            let mut string = String::from("#ROOT");
            string = self.branch.map(string, 0);
            string
        }
        pub fn map_debug (&self) -> String {
            let mut string = String::from("#ROOT");
            string = self.branch.map_debug(string, 0);
            string
        }
        
        pub (in super) fn expose (&self) -> & Branch {
            & self.branch
        }
        pub (in super) fn expose_mut (&mut self) -> &mut Branch {
            &mut self.branch
        }
        
    }
}

use crate::library::prelude::{Ui, MString, Outcome};
pub struct Widget {
    pub path: String,
}
impl Widget {
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

    pub fn new(system: &mut Ui::System, key: &str, position: Ui::Positions) -> Result<Widget, String> {
        match system.expose_mut().create_simple_checked(key, position) {
            Ok (new_key) => Result::Ok(Widget::from_path(new_key)),
            Err (message) => Err(String::from("UNABLE TO CREATE WIDGET! #Error: ") + &message),
        }
    }
    pub fn key(&self) -> String {
        String::from("#ROOT/") + &self.path + "/"
    }
    pub fn new_in(system: &mut Ui::System, widget: &Widget, key: &str, position: Ui::Positions) -> Result <Widget, String> {
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
    
    pub fn map (&self, system: & Ui::System) -> Result<String, String> {
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
    pub fn map_debug (&self, system: & Ui::System) -> Result<String, String> {
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
    pub fn destroy (&self, system: &mut Ui::System, path : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.destroy_chain_checked(path)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn remove (&self, system: &mut Ui::System, key : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.remove(key)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }

    pub fn exist (&self, system: &mut Ui::System, key : &str) -> bool{
        let mut path = String::from(&self.path);
        if !key.is_empty() {
            path += "/";
            path += &key
        }
        system.expose_mut().check_chain_checked(&path)
        
    }

    pub fn container_get<'a> (&'a self, system: &'a mut Ui::System, key : &str) -> Result<Ui::Container, String> {
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
    }
    
    // CONVERT ABSOLUTE INTO RELATIVE
    pub fn container_absolute_to_relative<'a> (&'a self, system: &'a mut Ui::System, key : &str, point: [f32;2]) -> Result<[f32; 2], String> {
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
    }
    
    


    pub fn information_set (&self, system: &mut Ui::System, path: &str, info: Ui::WidgetInfo) -> Outcome {          //OPTI
        let mut _path = String::from(&self.path);
        if !path.is_empty() {
            _path += "/";
            _path += &path
        }
        match system.expose_mut().borrow_chain_checked_mut(&_path){
            Ok (branch) => {
                branch.information_set(info);
                Outcome::Pass
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn information_get<'a> (&'a self, system: &'a Ui::System, path: &str) -> Result<&Ui::WidgetInfo, String> {          //OPTI
        let mut _path = String::from(&self.path);
        if !path.is_empty() {
            _path += "/";
            _path += &path
        }
        match system.expose().borrow_chain_checked(&_path){
            Ok (branch) => {
                branch.information_get()
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn information_get_mut<'a> (&'a self, system: &'a mut Ui::System, path: &str) -> Result<&mut Ui::WidgetInfo, String> {          //OPTI
        let mut _path = String::from(&self.path);
        if !path.is_empty() {
            _path += "/";
            _path += &path
        }
        match system.expose_mut().borrow_chain_checked_mut(&_path){
            Ok (branch) => {
                branch.information_get_mut()
            },
            Err (message) => Result::Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }




    pub fn position_add (&self, system: &mut Ui::System, name: &str, position: Ui::Positions, key: &str) -> Outcome {
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
    pub fn position_borrow<'a> (&'a self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<& Ui::Positions, String> {
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
    pub fn position_borrow_mut<'a> (&'a mut self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<&mut Ui::Positions, String> {
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
    
    pub fn position_borrow_solid<'a> (&'a self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<& Ui::Pos::Solid, String> {
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
                            Ui::Positions::Solid (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A SOLID TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_window<'a> (&'a self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<& Ui::Pos::Window, String> {
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
                            Ui::Positions::Window (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A WINDOW TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_relative<'a> (&'a self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<& Ui::Pos::Relative, String> {
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
                            Ui::Positions::Relative (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A RELATIVE TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_solid_mut<'a> (&'a mut self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<&mut Ui::Pos::Solid, String> {
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
                            Ui::Positions::Solid (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A SOLID TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_window_mut<'a> (&'a mut self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<&mut Ui::Pos::Window, String> {
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
                            Ui::Positions::Window (window) => Ok(window),
                            _ => Err(String::from("POSITION '") + name + "' IS NOT A WINDOW TYPE!"),
                        }
                    },
                    Err(message) => Err(message),
                }
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    pub fn position_borrow_relative_mut<'a> (&'a mut self, system: &'a mut Ui::System, name: &str, key: &str) -> Result<&mut Ui::Pos::Relative, String> {
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
                            Ui::Positions::Relative (window) => Ok(window),
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