use crate::drivers::*;
use bevy::prelude::*;

/// A chain of drivers, calculating displacements, and animating in succession.
#[derive(Default, Component)]
pub struct Rig {
    pub drivers: Vec<Box<dyn RigDriver>>,

    // TODO: Only really saving this for its rotation information
    // to allow movement to be relative to the camera, get it from transfrom instead?
    pub final_transform: Transform,
}

impl Rig {
    /// Returns the first driver of the matching type
    // TODO: any cleaner way? old way would panic, screw that
    pub fn get_driver_mut<T: RigDriver>(&mut self) -> Option<&mut T> {
        for driver in self.drivers.iter_mut() {
            match driver.as_any_mut().downcast_mut::<T>() {
                Some(a) => return Some(a),
                None => (),
            }
        }
        None
    }

    /// Runs all the drivers in sequence, animating the rig, and producing a final transform of the camera.
    /// Camera rigs are approximately framerate independent, so `update` can be called at any frequency.
    pub fn update(&mut self, delta_time_seconds: f32) -> Transform {
        let mut result = Transform::default();

        for driver in self.drivers.iter_mut() {
            driver.update(&mut result, delta_time_seconds);
        }

        self.final_transform = result;
        result
    }

    pub fn add(mut self, driver: impl RigDriver) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }
}
