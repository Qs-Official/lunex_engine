#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;
use crate::library::prelude::{HashMap, Outcome};



//===========================================================================
//#POSITION TYPE OF THE CONTAINER
//-------------------------------
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Window {
    pub absolute: Vec2,
    pub relative: Vec2,
    pub width_absolute: f32,
    pub width_relative: f32,
    pub height_absolute: f32,
    pub height_relative: f32,
}
impl Window {
    pub fn new () -> Window {
        Window {
            absolute: Vec2 { x: 0.0, y: 0.0 },
            relative: Vec2 { x: 0.0, y: 0.0 },
            width_absolute: 0.0,
            width_relative: 0.0,
            height_absolute: 0.0,
            height_relative: 0.0,
        }
    }
    pub fn wrap (self) -> PositionType {
        PositionType::Window(self)
    }
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let xs = width / 100.0;
        let ys = height / 100.0;
        (
            Vec2 {x: point.x + self.absolute.x + (self.relative.x * xs), y: point.y + self.absolute.y + (self.relative.y * ys)},
            self.width_absolute + (self.width_relative * xs),
            self.height_absolute + (self.height_relative * ys),
        )
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Relative {
    pub absolute_1: Vec2,
    pub absolute_2: Vec2,
    pub relative_1: Vec2,
    pub relative_2: Vec2,
}
impl Relative {
    pub fn new () -> Relative {
        Relative {
            absolute_1: Vec2 { x: 0.0, y: 0.0 },
            absolute_2: Vec2 { x: 0.0, y: 0.0 },
            relative_1: Vec2 { x: 0.0, y: 0.0 },
            relative_2: Vec2 { x: 0.0, y: 0.0 },
        }
    }
    pub fn wrap (self) -> PositionType {
        PositionType::Relative(self)
    }
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> [Vec2; 2] {
        let xs = width / 100.0;
        let ys = height / 100.0;
        [
            Vec2 {x: point.x + self.absolute_1.x + (self.relative_1.x * xs), y: point.y + self.absolute_1.y + (self.relative_1.y * ys)},
            Vec2 {x: point.x + self.absolute_2.x + (self.relative_2.x * xs), y: point.y + self.absolute_2.y + (self.relative_2.y * ys)},
        ]
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
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
    fn calculate (&self, point: Vec2, width: f32, height: f32) -> (Vec2, f32, f32) {
        let scale = match self.size {
            SolidSize::Fill => f32::max(width/self.width as f32, height/self.height as f32),
            SolidSize::Fit => f32::min(width/self.width as f32, height/self.height as f32),
        };

        let center = [point.x + width/2.0, point.y + height/2.0];
        let vanilla_width = self.width as f32*scale;
        let vanilla_height = self.height as f32*scale;
        let vanilla_point = [center[0] - vanilla_width/2.0, center[1] - vanilla_height/2.0];

        (
            Vec2 {x: (vanilla_point[0] + (vanilla_point[0] - point[0])*self.horizontal_anchor), y: (vanilla_point[1] + (vanilla_point[1] - point[1])*self.vertical_anchor)},
            vanilla_width,
            vanilla_height,
        )
    }
}

//-------------------------------
//===========================================================================
//-------------------------------


#[derive(Clone, Debug, PartialEq, Default)]
pub enum SolidSize {
    #[default]
    Fit,
    Fill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PositionType {
    Window (Window),
    Relative (Relative),
    Solid (Solid),
}
impl Default for PositionType {
    fn default() -> Self {
        PositionType::Relative(Relative {..Default::default()})
    }
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ContainerPosition {
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

//-------------------------------
//===========================================================================
//-------------------------------


#[derive(Clone, Debug, PartialEq, Default)]
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
    pub fn calculate (&mut self, point: Vec2, width: f32, height: f32) {
        match self.position_types.get(&self.current_position){
            None => {},
            Some (position_types) => match position_types {
                PositionType::Window(container) => {
                    let values = container.calculate(point, width, height);
                    self.position.point_1 = values.0;
                    self.position.width = values.1;
                    self.position.height = values.2;
                    self.position.point_2 = Vec2 {x: self.position.point_1.x + self.position.width, y: self.position.point_1.y + self.position.height};
                },
                PositionType::Relative(container) => {
                    let values = container.calculate(point, width, height);
                    self.position.point_1 = values[0];
                    self.position.width = values[1][0] - values[0][0];
                    self.position.height = values[1][1] - values[0][1];
                    self.position.point_2 = Vec2 {x: self.position.point_1.x + self.position.width, y: self.position.point_1.y + self.position.height};
                },
                PositionType::Solid(container) => {
                    let values = container.calculate(point, width, height);
                    self.position.point_1 = values.0;
                    self.position.width = values.1;
                    self.position.height = values.2;
                    self.position.point_2 = Vec2 {x: self.position.point_1.x + self.position.width, y: self.position.point_1.y + self.position.height};
                },
            },
        }
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
