#![allow(dead_code)]
#![allow(unused_variables)]

use crate::library::prelude::{HashMap, Outcome};



//===========================================================================
//#POSITION TYPE OF THE CONTAINER
//-------------------------------
#[derive(Default, Clone, Debug)]
pub struct Window {
    pub point_absolute: [f32; 2],
    pub point_relative: [f32; 2],
    pub width_absolute: f32,
    pub width_relative: f32,
    pub height_absolute: f32,
    pub height_relative: f32,
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


#[derive(Default, Clone, Debug)]
pub struct Relative {
    pub point_absolute_1: [f32; 2],
    pub point_absolute_2: [f32; 2],
    pub point_relative_1: [f32; 2],
    pub point_relative_2: [f32; 2],
}
impl Relative {
    pub fn new () -> Relative {
        Relative {
            point_absolute_1: [0.0,0.0],
            point_absolute_2: [0.0,0.0],
            point_relative_1: [0.0,0.0],
            point_relative_2: [0.0,0.0],
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


#[derive(Default, Clone, Debug)]
pub struct Solid {
    pub width: u32,
    pub height: u32,
    pub horizontal_anchor: f32,
    pub vertical_anchor: f32,
    pub size: SolidSize,
}
impl Solid {
    pub fn new () -> Solid {
        Solid {
            width: 0,
            height: 0,
            horizontal_anchor: 0.0,
            vertical_anchor: 0.0,
            size: SolidSize::Fit,
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

//-------------------------------
//===========================================================================
//-------------------------------


#[derive(Default, Clone, Debug)]
pub enum SolidSize {
    #[default]
    Fit,
    Fill,
}
#[derive(Clone, Debug)]
pub enum PositionType {
    Window (Window),
    Relative (Relative),
    Solid (Solid),
}
#[derive(Default, Clone)]
pub struct ContainerPosition {
    pub point_1: [f32; 2],
    pub point_2: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

//-------------------------------
//===========================================================================
//-------------------------------

pub struct Container {
    position: ContainerPosition,
    position_types: HashMap<String, PositionType>,
    current_position: String,
}
impl Container {
    pub fn new () -> Container {
        Container {
            position: ContainerPosition { ..Default::default() },
            position_types: HashMap::new(),
            current_position: "default".to_string(),
        }
    }
    pub fn calculate (&mut self, point: [f32; 2], width: f32, height: f32) {
        //if self.position_types.len() != 0 {
            match self.position_types.get(&self.current_position){
                None => {},
                Some (position_types) => match position_types {
                    PositionType::Window(container) => {
                        let values = container.calculate(point, width, height);
                        self.position.point_1 = values.0;
                        self.position.width = values.1;
                        self.position.height = values.2;
                        self.position.point_2 = [self.position.point_1[0] + self.position.width, self.position.point_1[1] + self.position.height];
                    },
                    PositionType::Relative(container) => {
                        let values = container.calculate(point, width, height);
                        self.position.point_1 = values[0];
                        self.position.width = values[1][0] - values[0][0];
                        self.position.height = values[1][1] - values[0][1];
                        self.position.point_2 = [self.position.point_1[0] + self.position.width, self.position.point_1[1] + self.position.height];
                    },
                    PositionType::Solid(container) => {
                        let values = container.calculate(point, width, height);
                        self.position.point_1 = values.0;
                        self.position.width = values.1;
                        self.position.height = values.2;
                        self.position.point_2 = [self.position.point_1[0] + self.position.width, self.position.point_1[1] + self.position.height];
                    },
                },
            }
            
        //}
    }
    pub fn position (&self) -> &ContainerPosition {
        &self.position
    }

    pub fn position_add (&mut self, key: &str, position: PositionType) -> Outcome {
        if !self.position_types.contains_key(key) {
            self.position_types.insert(String::from(key), position);
            Outcome::Pass
        } else {
            Outcome::Fail(format!("The key '{}' is already in use!", &key).to_string())
        }
    }

    pub fn position_borrow (&self, key: &str) -> Result<& PositionType, String> {
        match self.position_types.get(key) {
            Some (position) => Result::Ok(position),
            None => Result::Err(format!("Container does not have position '{}'!", &key).to_string()),
        }
    }
    pub fn position_borrow_mut (&mut self, key: &str) -> Result<&mut PositionType, String> {
        match self.position_types.get_mut(key) {
            Some (position) => Result::Ok(position),
            None => Result::Err(format!("Container does not have position '{}'!", &key).to_string()),
        }
    }

}
