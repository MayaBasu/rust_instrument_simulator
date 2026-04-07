
use plotpy::{Curve, Legend, Plot, StrError};



#[derive(Clone, Debug)]
pub struct CoordinateSystem{
    pub(crate) x_axis: (f64, f64),
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
    pub(crate) fn plot(&self) -> (Curve, Curve){
        let mut x_axis = Curve::new();
        x_axis.set_line_width(2.0);
        x_axis.set_line_color(self.color.as_str());
        x_axis.set_label(self.label.as_str());

        x_axis.points_begin();
        x_axis.points_add(self.center.0,self.center.1);
        x_axis.points_add(self.x_axis.0, self.x_axis.1);
        x_axis.points_end();


        let mut y_axis = Curve::new();
        y_axis.set_line_width(2.0);
        y_axis.set_line_color(self.color.as_str());
        y_axis.points_begin();
        y_axis.points_add(self.center.0,self.center.1);
        y_axis.points_add(self.y_axis.0, self.y_axis.1);
        y_axis.points_end();

        (x_axis,y_axis)

        
    }

    pub fn plot_coordinate_systems(coordinate_systems: Vec<&CoordinateSystem>,plot:&mut Plot){
        let curves:Vec<(Curve,Curve)> = coordinate_systems.iter().map(|coordinate_system: &&CoordinateSystem|
        coordinate_system.plot()).collect();

        let mut legend = Legend::new();
        legend.draw();

        for axes in curves{
            plot.add(&axes.0).grid_and_labels("x", "y");
            plot.add(&axes.1).grid_and_labels("x", "y");
        }
        plot.add(&legend);

    }
}






