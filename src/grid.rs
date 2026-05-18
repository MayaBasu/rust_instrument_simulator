use plotpy::{Curve, Plot, Text};
use crate::coordinate_system::{CoordinateSystem, Coordinates};
use rand::RngExt;
use crate::point::Point;
use crate::psf::PSF;

pub enum Corners{
    Four(usize,usize,usize,usize),
    Two(usize,usize),
    One(usize),
}
pub enum Location{
    Outside,
    Inside,
}
pub enum Bin{
    Binned(usize,usize),
    Outside,
}


#[derive(Debug,Clone)]
pub struct Grid {
    pub coordinates: Coordinates,

    pub x_num: usize,
    pub y_num: usize,

    pub x_step_size: f64,
    pub y_step_size: f64,

    pub x_size: f64,
    pub y_size: f64,

    pub num_points: usize,

    pub center: (f64,f64),
    pub corner: (f64, f64),

    pub snap_precision: f64,

    pub label: String,

}

impl Grid{
    pub fn new_empty((x_num,y_num): (usize,usize),
                     (x_step_size,y_step_size): (f64,f64),
                    center: (f64,f64),
                    snap_precision: f64,
                     coordinates: Coordinates,
    ) -> Grid{
        let x_size = x_step_size*(x_num-1)as f64;
        let y_size = y_step_size*(y_num-1)as f64;
        let num_points = x_num*y_num;
        let (x_0,y_0) = center;
        let corner = (x_0 - x_size/2.0,y_0 - y_size/2.0);
        assert!(snap_precision<0.5);
        Grid{
            coordinates,
            x_num,
            x_step_size,
            x_size,
            y_num,
            y_step_size,
            y_size,
            num_points,
            center,
            corner,
            snap_precision,
            label: "".to_string(),
        }
    }




    pub fn xy_indices(&self, grid_number:usize) -> (usize,usize){
        assert!((grid_number <= self.num_points - 1)&&(grid_number >= 0));
        let x_index = grid_number % self.x_num;
        let y_index = (grid_number - x_index)/self.y_num;
        (x_index,y_index)
    }

    pub fn grid_number(&self, x_index:usize,y_index:usize) -> usize{
        assert!(x_index <= self.x_num-1);
        assert!(y_index <= self.y_num-1);
        y_index*self.x_num + x_index
    }

    pub fn relative_location(&self, grid_number:usize) -> Point {

        assert!((grid_number <= self.x_num*self.y_num-1)&&(grid_number >= 0));
        let (x_corner,y_corner) = self.corner;
        let (x_index,y_index) = self.xy_indices(grid_number);
        let (x,y) = (x_corner + x_index as f64 *self.x_step_size, y_corner + y_index as f64 *self.y_step_size);
        println!("relatice location is {:?}",(x,y));
        Point::new(x,y,self.coordinates.clone())
    }

    pub fn absolute_location(&self, grid_number:usize) -> Point{
        let location = self.relative_location(grid_number);
        location.to_absolute()
    }


    pub fn snap(&self,point:(f64,f64))-> usize{
        let (x_mod,y_mod,x_residual,y_residual) = self.fit_grid(point);
        if (x_residual.abs() >= self.snap_precision)  | (y_residual.abs() >= self.snap_precision){
            panic!("Couldn't snap point")
        };
        self.grid_number(x_mod,y_mod)
    }

    pub fn random(&self)-> Point{
        let mut rng = rand::rng();
        let x_scale: f64 = rng.random();
        let y_scale: f64 = rng.random();
        let (x,y) = (self.corner.0 + self.x_size*x_scale, self.corner.1 + self.y_size*y_scale);
        //println!("Randomly generating point {:?} within the grid",(x,y));
        Point::new(x,y,self.coordinates.clone())
    }

    pub fn fit_grid(&self, point:(f64,f64))->(usize,usize,f64,f64){
        //ensure that the point is within the grid
        match self.inside_or_outside(point) {
            Location::Outside => {
                println!("Tried to grid point {:?}",point);
                panic!("Tried to grid a point that was outside of the grid")}
            Location::Inside => {}
        }
        //find the nearest point and then return the residuals to it

        let (x,y) = point;
        let (corner_x,corner_y) = self.corner;
        let delta_x = x - corner_x;
        let delta_y = y - corner_y;
      //  println!("delta x and delta y are {:?}, {:?}",delta_x,delta_y);
        let x_scaled_residual = delta_x/self.x_step_size - (delta_x/self.x_step_size).floor();
        let y_scaled_residual = delta_y/self.y_step_size - (delta_y/self.y_step_size).floor();

       // println!("residuals are {:?}, {:?}",x_scaled_residual,y_scaled_residual);

        let (x_mod, x_residual) = if x_scaled_residual <= 0.5{
            let x_mod = (delta_x/self.x_step_size).floor() as usize;
            let x_residual = x_scaled_residual*self.x_step_size;

            (x_mod,x_residual)

        }else{
            let x_mod = (delta_x/self.x_step_size).floor() as usize + 1;
            let x_residual = (x_scaled_residual-1.0)*self.x_step_size;
            (x_mod,x_residual)
        };

        let (y_mod,y_residual) = if y_scaled_residual <=0.5{
            let y_mod = (delta_y/self.y_step_size).floor() as usize;
            let y_residual = y_scaled_residual*self.y_step_size;
            (y_mod,y_residual)
        }else{
            let y_mod = (delta_y/self.y_step_size).floor() as usize + 1;
            let y_residual = (y_scaled_residual-1.0)*self.y_step_size;
            (y_mod,y_residual)
        };

     //   println!("{:?}",(x_mod,y_mod,x_residual,y_residual));
        //residuals should be between -0.5 and 0.5 times the grid width

        (x_mod,y_mod,x_residual,y_residual)
    }
//TODO remove redundancey of these two functions
    pub fn fit_grid_unscaled(&self, point:(f64,f64))->(usize,usize,f64,f64){
        //ensure that the point is within the grid
        match self.inside_or_outside(point) {
            Location::Outside => {
                println!("Tried to grid point {:?}",point);
                panic!("Tried to grid a point that was outside of the grid")}
            Location::Inside => {}
        }
        //find the nearest point and then return the residuals to it

        let (x,y) = point;
        let (corner_x,corner_y) = self.corner;

        let delta_x = x - corner_x;
        let delta_y = y - corner_y;
        println!("DELTAS ARE {:?}",(delta_x,delta_y));
        //  println!("delta x and delta y are {:?}, {:?}",delta_x,delta_y);
        let x_scaled_residual = delta_x/self.x_step_size - (delta_x/self.x_step_size).floor();
        let y_scaled_residual = delta_y/self.y_step_size - (delta_y/self.y_step_size).floor();

        // println!("residuals are {:?}, {:?}",x_scaled_residual,y_scaled_residual);

        let (x_mod, x_residual) = if x_scaled_residual <= 0.5{
            let x_mod = (delta_x/self.x_step_size).floor() as usize;
            let x_residual = x_scaled_residual;

            (x_mod,x_residual)

        }else{
            let x_mod = (delta_x/self.x_step_size).floor() as usize + 1;
            let x_residual = (x_scaled_residual-1.0);
            (x_mod,x_residual)
        };

        let (y_mod,y_residual) = if y_scaled_residual <=0.5{
            let y_mod = (delta_y/self.y_step_size).floor() as usize;
            let y_residual = y_scaled_residual;
            (y_mod,y_residual)
        }else{
            let y_mod = (delta_y/self.y_step_size).floor() as usize + 1;
            let y_residual = (y_scaled_residual-1.0);
            (y_mod,y_residual)
        };

        //   println!("{:?}",(x_mod,y_mod,x_residual,y_residual));
        //residuals should be between -0.5 and 0.5 times the grid width

        (x_mod,y_mod,x_residual,y_residual)
    }

    pub fn bin_up_patch(&self, center_of_the_corner_pixel:Point,psf:&Vec<Vec<f32>>,scale:usize)-> ((usize,usize),Vec<Vec<f32>>){ //TODO make this grid dependent
        let Point{x,y,..} = center_of_the_corner_pixel.convert(&self.coordinates);
        let center_of_the_corner_pixel = (x,y);

        let (x_mod,y_mod,x_residual,y_residual) = self.fit_grid_unscaled(center_of_the_corner_pixel);
        //unscaled returns the raction of the pixel
        //TODO remove assumption that the psf is sven by even!!!
        //TODO remove the assumption that the scale is an intege
        //scale is the number of little pixels of the psf that fit into one big detector pixel
        //x direction
        let x_pixels = 64; //TODO
        let y_pixels = 64;
        let scale = scale as f64;
        let x_tail_end_in_pixels = x_residual*scale + scale/2.0 - 1.0;
        let y_tail_end_in_pixels = y_residual*scale + scale/2.0 - 1.0;
        let x_offset = if x_tail_end_in_pixels < 0.0{ 0 }else{
            x_tail_end_in_pixels.ceil() as usize
        };
        let y_offset = if y_tail_end_in_pixels < 0.0{ 0 }else{
            y_tail_end_in_pixels.ceil() as usize
        };

        let binned_x_size = if x_offset > 0{
            (x_pixels as f64/scale + 1.0) as usize
        }else{
            (x_pixels as f64/scale)  as usize
        };

        let binned_y_size = if y_offset > 0{
            (y_pixels as f64/scale + 1.0) as usize
        }else{
            (y_pixels as f64/scale)  as usize
        };
        println!("Binned x size, y size is {:?}",((x_pixels as f64/scale).ceil() as usize + 1,(y_pixels as f64/scale).ceil() as usize + 1));

        let mut binned_psf = vec![vec![0.0;(x_pixels as f64/scale).ceil() as usize + 1];(y_pixels as f64/scale).ceil() as usize + 1];
        for y in 0..y_pixels{
            for x in 0..x_pixels{
                let binned_x_index:usize = ((x + x_offset) as f64/scale).floor() as usize;
                let binned_y_index:usize = ((y + y_offset) as f64/scale).floor() as usize;
                binned_psf[binned_y_index][binned_x_index] += psf[y][x];
            }
        }
        println!(" {:?} MODULUSES ER {:?}",center_of_the_corner_pixel,(x_mod,y_mod));
        ((x_mod,y_mod),binned_psf)

    }





    pub fn inside_or_outside(&self, point:(f64,f64)) -> Location{
        let (x,y) = point;
        let epsilon = self.snap_precision;
        let (corner_x,corner_y) = self.corner;
        let (grid_x_min, grid_x_max) = (corner_x, corner_x + self.x_size);
        let (grid_y_min, grid_y_max) = (corner_y, corner_y + self.y_size);

        if (x < grid_x_min - epsilon) | (x > grid_x_max + epsilon) | (y < grid_y_min - epsilon) | (y > grid_y_max + epsilon){
            println!("Point was outside of the grid, min x is {grid_x_min}, max x is {grid_x_max}, min y is {grid_y_min}, max y is {grid_y_max}");
            return Location::Outside
        };
        Location::Inside
    }



    pub fn find_corners(&self,point:(f64,f64)) -> Corners{
        let epsilon = self.snap_precision;

        let (x_mod,y_mod,x_residual,y_residual)  = self.fit_grid(point);

        //first we check to see if it is most appropriate to snap this point to a grid point:
        if (x_residual.abs() <= epsilon) && (y_residual.abs() <= epsilon) {
           // println!("Snapped to grid point");
            return Corners::One(self.grid_number(x_mod,y_mod))
        };
        //Is the point between two vertical grid points?
        if (x_residual.abs() <= epsilon) {
            //println!("Snapped between vertical points");
            if y_residual < 0.0{
                return Corners::Two(self.grid_number(x_mod,y_mod),self.grid_number(x_mod,y_mod-1));
            }else{
                return Corners::Two(self.grid_number(x_mod,y_mod+1),self.grid_number(x_mod,y_mod));
            }
        };
        //Is the point between two horizontal grid points?
        if (y_residual.abs() <= epsilon) {
          //  println!("Snapped between horizontal points");
            if x_residual < 0.0{
                return Corners::Two(self.grid_number(x_mod,y_mod),self.grid_number(x_mod-1,y_mod));
            }else{
                return Corners::Two(self.grid_number(x_mod+1,y_mod),self.grid_number(x_mod,y_mod));
            }
        };
        //The point must be in the middle

        let (upper_x,lower_x) = if x_residual < 0.0{ (x_mod,x_mod-1) }else{ (x_mod+1, x_mod)};
        let (upper_y,lower_y) = if y_residual < 0.0{ (y_mod,y_mod-1) }else{ (y_mod+1, y_mod)};

       // println!("Finding four corners, enumerated clockwise starting top left");
        Corners::Four(self.grid_number(lower_x,upper_y),
                      self.grid_number(upper_x,upper_y),
                      self.grid_number(upper_x,lower_y),
                      self.grid_number(lower_x,lower_y))

    }

    pub fn plot(&self,plot:&mut Plot,add_point:PlotPoint){
        CoordinateSystem::plot_coordinate_systems(vec![&self.coordinates],plot);

        let mut grid_points = Curve::new();
        grid_points.set_line_style("none")
            .set_label(format!("Grid points: {:?}",self.label).as_str())
            .set_marker_color("blue")
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style(".");

        let mut corner = Curve::new();
        corner
            .set_label("Corner")
            .set_line_style("none")
            .set_marker_color("#eeea83")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");

        let mut frame = Curve::new();
        frame.set_line_width(1.0)
            .set_label("Frame")
            .set_line_style("solid")
            .set_line_width(1.0)
            .set_marker_color("purple")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");

        let mut extra_point = Curve::new();
        extra_point.set_marker_color("#eeea83")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_line_style("none")
            .set_marker_style("*");



        let mut grid_numbers = Text::new();
        grid_numbers.set_color("purple")
            .set_fontsize(5.0);


        grid_points.points_begin();
        for point in 0..self.num_points{
            let point_location = self.absolute_location(point);
            grid_points.points_add(point_location.x, point_location.y);
            let label = format!("{}",point);
            grid_numbers.draw(point_location.x, point_location.y, label.as_str());
        }
        grid_points.points_end();



        corner.points_begin();
        let corner_location = self.corner;
        let corner_label = format!("Corner: ({:.3},{:.3})",corner_location.0,corner_location.1);
        corner.points_add(corner_location.0,corner_location.1).set_label(corner_label.as_str());
        corner.points_end();

        let mut example_point = Vec::new();

        match add_point {
            PlotPoint::No => {}
            PlotPoint::Given(x, y) => { example_point.push((x,y))}
            PlotPoint::Random => {
                let random = self.random();
                example_point.push((random.x,random.y)); }
        };

        for point in example_point{
            let (_,_, x_res, y_res) = self.fit_grid(point);
            extra_point.points_begin();
            extra_point.points_add(point.0,point.1).set_label(format!("x, y residuals: {:.3}, {:.3}", x_res, y_res).as_str());
            extra_point.points_end();

            let corners  = self.find_corners(point);
            let mut frame_points = Vec::new();
            match corners{
                Corners::Four(a, b, c, d) => {
                    frame_points.push(a);
                    frame_points.push(b);
                    frame_points.push(c);
                    frame_points.push(d);
                    frame_points.push(a);
                }
                Corners::Two(a,b) => {
                    frame_points.push(a);
                    frame_points.push(b);}
                Corners::One(a) => { frame_points.push(a); }
            }
            frame.points_begin();
            for point in frame_points{
                println!("point {point}");
                let point = self.absolute_location(point);
                frame.points_add(point.x,point.y);
            }

            frame.points_end();
        }

        plot.add(&frame);
        plot.add(&grid_numbers);
        plot.add(&grid_points);
        plot.add(&extra_point);
        plot.add(&corner)
            .set_figure_size_inches(10.0,10.0)
            .set_num_ticks_y(self.y_num+1)
            .set_num_ticks_x(self.x_num+1)
            .grid_labels_legend("x", "y");


    }

}

pub enum PlotPoint{
    No,
    Given(f64,f64),
    Random,
}



