use plotpy::{Curve, Legend, Plot};
use serde::{Deserialize, Serialize};
use crate::point::Point;

#[derive(Clone, Debug)]
pub enum Coordinates {
    ABSOLUTE,
    RELATIVE(CoordinateSystem),
}

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct CoordinateSystem{
    pub x_axis: (f64, f64),
    pub y_axis: (f64,f64),
    pub center: (f64,f64),
    pub color: String,
    pub label: String,
}

impl CoordinateSystem{
    pub fn new(x_axis:(f64,f64),y_axis:(f64,f64),center: (f64,f64),label:String,color:String) -> CoordinateSystem{


        CoordinateSystem{
            x_axis,
            y_axis,
            center,
            color,
            label,
        }
    }
    pub fn point_from_absolute(&self, point:Point) -> Point{
        match point.coordinates{
            Coordinates::ABSOLUTE => {
                let proj_x = (point.x*self.x_axis.0 + point.y*self.x_axis.1)/(self.x_axis.0.powi(2) + self.x_axis.1.powi(2));
                let proj_y = (point.x*self.y_axis.0 + point.y*self.y_axis.1)/(self.y_axis.0.powi(2) + self.y_axis.1.powi(2));
                println!("{:?} PROJECTIONS ARE {:?}",(self.x_axis.0,self.x_axis.1),(proj_y,proj_x));
                Point::new(proj_x,proj_y,Coordinates::RELATIVE(self.clone()))
            }
            Coordinates::RELATIVE(_) => {panic!("tried to from_absolute a point in a not absolute coordinate system :( ")}
        }

    }



    pub(crate) fn plot(&self) -> (Curve, Curve){
        let mut x_axis = Curve::new();
        let scale = 1.0/self.x_axis.0;
        x_axis.set_line_width(2.0);
        x_axis.set_line_color(self.color.as_str());
        x_axis.set_label(format!("x axis for {:?}",self.label).as_str());

        x_axis.points_begin();
        x_axis.points_add(self.center.0,self.center.1);
        x_axis.points_add(self.center.0 + (self.x_axis.0)*scale, self.center.1 + (self.x_axis.1)*scale);
        x_axis.points_end();


        let mut y_axis = Curve::new();
        y_axis.set_line_width(1.0);
        y_axis.set_line_color(self.color.as_str());
        y_axis.set_label(format!("y axis for {:?}",self.label).as_str());

        y_axis.points_begin();
        y_axis.points_add(self.center.0,self.center.1);
        y_axis.points_add(self.center.0  + (self.y_axis.0)*scale,self.center.1  +  (self.y_axis.1)*scale);
        y_axis.points_end();

        (x_axis,y_axis)
    }

    pub fn plot_coordinate_systems(coordinates: Vec<&Coordinates>,plot:&mut Plot){
        let curves:Vec<(Curve,Curve)> = coordinates.iter().map(|coordinates: &&Coordinates|
            match coordinates{
                Coordinates::ABSOLUTE => {panic!("TODO")}
                Coordinates::RELATIVE(coordinate_system) => {coordinate_system.plot()}
            }
        ).collect();

        let mut legend = Legend::new();
        legend.draw();

        for axes in curves{
            plot.add(&axes.0).grid_and_labels("x", "y");
            plot.add(&axes.1).grid_and_labels("x", "y");
        }
        plot.add(&legend);

    }
}






