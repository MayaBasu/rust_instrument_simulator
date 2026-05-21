use serde::Serialize;
use crate::coordinate_system::{Coordinates};

#[derive(Clone, Debug,Serialize)]
pub struct Point{
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) coordinates: Coordinates
}
impl Point{
    pub fn new(x:f64,y:f64,coordinates: Coordinates) -> Point{
        Point{x,y,coordinates}
    }
    pub fn to_absolute(&self) -> Point{
        match &self.coordinates{
            Coordinates::ABSOLUTE => {println!("already in absolute");
                self.clone()}
            Coordinates::RELATIVE(coordinate_system) => {
                let absolute_x =  self.x * coordinate_system.x_axis.0 + self.y *coordinate_system.y_axis.0;
                let absolute_y = self.x * coordinate_system.x_axis.1 + self.y * coordinate_system.y_axis.1;
                Point::new(absolute_x,absolute_y,Coordinates::ABSOLUTE)
            }
        }
    }
    pub fn convert(&self, coordinate_system: &Coordinates) -> Point{
        let absolute = self.to_absolute();
        match coordinate_system{
            Coordinates::ABSOLUTE => { absolute }
            Coordinates::RELATIVE(coordinate_system  ) => {
                coordinate_system.point_from_absolute(absolute)
            }
        }
    }
    pub fn values(&self)-> (f64,f64){
        (self.x,self.y)
    }
}