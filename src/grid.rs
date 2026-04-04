use std::io::Write;
use std::fs::File;
use std::fs;
use crate::data_frame::*;


pub enum Corners{
    Four(usize,usize,usize,usize),
    Two(usize,usize),
    One(usize),
    None
}
pub enum Location{
    Outside,
    Inside,
}

#[derive(Debug,Clone)]
pub enum Data{
    //Data attached to a grid can either be a dataframe at each point, or a single array of values
    Frames(Vec<(usize, DataFrame)>),
    Scalars(Vec<Vec<f64>>),
}


#[derive(Debug)]
pub struct Grid {
    pub x_num: usize,
    pub x_step_size: f64,
    pub x_size: f64,
    pub y_num: usize,
    pub y_step_size: f64,
    pub y_size: f64,
    pub num_points: usize,
    pub center: (f64,f64),
    pub corner: (f64, f64),
    pub data: Data,
    pub snap_precision: f64,
    valid: bool,
}

impl Grid{
    pub fn new_empty(x_num: usize,
                    x_step_size: f64,
                    y_num: usize,
                    y_step_size: f64,
                    center: (f64,f64),
                    data: Data,
                    snap_precision: f64,
    ) -> Grid{
        let x_size = x_step_size*(x_num-1)as f64;
        let y_size = y_step_size*(y_num-1)as f64;
        let num_points = x_num*y_num;
        let (x_0,y_0) = center;
        let corner = (x_0 - x_size/2.0,y_0 - y_size/2.0);
        assert!(snap_precision<0.5);
        Grid{
            x_num,
            x_step_size,
            x_size,
            y_num,
            y_step_size,
            y_size,
            num_points,
            center,
            corner,
            data,
            snap_precision,
            valid: false,
        }
    }


    pub fn load_data_frames(&mut self, directory_path:&str, center_fits_keys:(&str, &str), pixels:(usize, usize), size:(f64, f64)){
        
        let data = match self.data{
            Data::Frames(_) => {
                println!("Loading data frames into grid. This overwrites any data previously loaded");
                let mut data = vec![];
                let paths = fs::read_dir(directory_path).unwrap();
                let mut counter = 0;
                for path in paths {
                    counter += 1;
                    let path = path.unwrap().path();
                    let (x_pixels,y_pixels) = pixels;
                    let data_file = DataFile{
                        description: counter.to_string(),
                        path,
                        x_pixels,
                        y_pixels,
                    };
                    let frame = DataFrame::load_file(data_file,
                                                     (Load::FromKey(center_fits_keys.0.to_string()), Load::FromKey(center_fits_keys.1.to_string())),
                                                     (Load::FromValue(size.0),Load::FromValue(size.0)));
                    let frame_index = self.snap(frame.center);
                    data.push((frame_index,frame))
                }
                data.sort_by_key(|x|x.0);
                println!("Loaded {counter} files into a Grid Struct from {directory_path}");
                assert_eq!(counter,self.num_points,"Loaded the wrong number of PSF files");
                Data::Frames(data)
            }
            Data::Scalars(_) => {panic!("Can not load frames into a Grid if the data type is not set to frames (it is set to scalar data)")}
        };
        self.data = data;
        
    }



    pub fn xy_indices(&self, grid_number:usize) -> (usize,usize){
        assert!((grid_number <= self.num_points - 1)&&(grid_number >= 0));
        let x_index = grid_number % self.x_num;
        let y_index = (grid_number - x_index)/self.y_num;
        (x_index,y_index)
    }

    pub fn grid_number(&self, x_index:usize,y_index:usize) -> usize{
        assert!((x_index <= self.x_num-1) && (x_index >= 0));
        assert!((y_index <= self.y_num-1) && (y_index >= 0));
        y_index*self.x_num + x_index
    }

    pub fn location(&self, grid_number:usize) -> (f64,f64){
        assert!((grid_number <= self.x_num*self.y_num-1)&&(grid_number >= 0));
        let (x_corner,y_corner) = self.corner;
        let (x_index,y_index) = self.xy_indices(grid_number);
        (x_corner + x_index as f64 *self.x_step_size,
         y_corner + y_index as f64 *self.y_step_size)
    }
    pub fn snap(&self,point:(f64,f64))-> usize{
        let (x_mod,y_mod,x_residual,y_residual) = self.fit_grid(point);
        if (x_residual.abs() >= self.snap_precision)  | (y_residual.abs() >= self.snap_precision){
            panic!("Couldn't snap point")
        };
        self.grid_number(x_mod,y_mod)
    }

    pub fn fit_grid(&self, point:(f64,f64))->(usize,usize,f64,f64){
        //ensure that the point is within the grid
        match self.inside_or_outside(point) {
            Location::Outside => {panic!("Tried to grid a point that was outside of the grid")}
            Location::Inside => {}
        }
        //find the nearest point and then return the residuals to it

        let (x,y) = point;
        let (corner_x,corner_y) = self.corner;
        let delta_x = x - corner_x;
        let delta_y = y - corner_y;
        let x_scaled_residual = delta_x/self.x_step_size - (delta_x/self.x_step_size).floor();
        let y_scaled_residual = delta_y/self.y_step_size - (delta_y/self.y_step_size).floor();

        let (x_mod, x_residual) = if x_scaled_residual <= 0.5{
            let x_mod = (delta_x/self.x_step_size).floor() as usize;
            let x_residual = x_scaled_residual*self.x_step_size;
            (x_mod,x_residual)

        }else{
            let x_mod = (delta_x/self.x_step_size).floor() as usize + 1;
            let x_residual = -1.0*x_scaled_residual*self.x_step_size;
            (x_mod,x_residual)
        };

        let (y_mod,y_residual) = if y_scaled_residual <=0.5{
            let y_mod = (delta_y/self.y_step_size).floor() as usize;
            let y_residual = y_scaled_residual*self.y_step_size;
            (y_mod,y_residual)
        }else{
            let y_mod = (delta_y/self.y_step_size).floor() as usize + 1;
            let y_residual = -1.0*y_scaled_residual*self.y_step_size;
            (y_mod,y_residual)
        };

        (x_mod,y_mod,x_residual,y_residual)
    }



    pub fn inside_or_outside(&self, point:(f64,f64)) -> Location{
        let (x,y) = point;
        let epsilon = self.snap_precision;
        let (corner_x,corner_y) = self.corner;
        let (grid_x_min, grid_x_max) = (corner_x, corner_x + self.x_size);
        let (grid_y_min, grid_y_max) = (corner_y, corner_y + self.y_size);

        if (x < grid_x_min - epsilon) | (x > grid_x_max + epsilon) | (y < grid_y_min - epsilon) | (y > grid_y_max + epsilon){
            println!("Point was outside of the grid");
            return Location::Outside
        };
        Location::Inside
    }



    pub fn find_corners(&self,point:(f64,f64)) -> Corners{
        let epsilon = self.snap_precision;

        let (x_mod,y_mod,x_residual,y_residual)  = self.fit_grid(point);

        //first we check to see if it is most appropriate to snap this point to a grid point:
        if (x_residual.abs() <= epsilon) && (y_residual.abs() <= epsilon) {
            return Corners::One(self.grid_number(x_mod,y_mod))
        };
        //Is the point between two vertical grid points?
        if (x_residual.abs() <= epsilon) {
            if y_residual < 0.0{
                return Corners::Two(self.grid_number(x_mod,y_mod),self.grid_number(x_mod,y_mod-1));
            }else{
                return Corners::Two(self.grid_number(x_mod,y_mod+1),self.grid_number(x_mod,y_mod));
            }
        };
        //Is the point between two horizontal grid points?
        if (y_residual.abs() <= epsilon) {
            if x_residual < 0.0{
                return Corners::Two(self.grid_number(x_mod,y_mod),self.grid_number(x_mod-1,y_mod));
            }else{
                return Corners::Two(self.grid_number(x_mod+1,y_mod),self.grid_number(x_mod,y_mod));
            }
        };
        //The point must be in the middle

        let (upper_x,lower_x) = if x_residual < 0.0{ (x_mod,x_mod-1) }else{ (x_mod+1, x_mod)};
        let (upper_y,lower_y) = if y_residual < 0.0{ (y_mod,y_mod-1) }else{ (y_mod+1, y_mod)};


        Corners::Four(self.grid_number(upper_y,lower_x),
                      self.grid_number(upper_y,upper_x),
                      self.grid_number(lower_y,upper_x),
                      self.grid_number(lower_y,lower_x))

    }
    pub fn get_frame(&self,grid_number:usize)-> DataFrame{
        //TODO: remove this clone?
        match &self.data{
            Data::Frames(data)=> {
                let (index,data_frame) = data[grid_number].clone();
                assert_eq!(index,grid_number);
                data_frame
            }
            Data::Scalars(data)=>{panic!("Get frame attempted on scalar grid")}
        }
        
        

    }


    pub fn pretty_print(&self){
        for y in 0..self.y_num{
            for x in 0..self.x_num{
                let index = self.grid_number(x,y);
                let (location_x,location_y) = self.location(index);
                let (location_x,location_y) = ((location_x*100.0).round()/100.0,(location_y*100.0).round()/100.0);
                print!("{index}:{:?}  ",(location_x,location_y))
            }
            println!("\n")
        }
    }
    pub fn validate(&mut self) -> bool {
        
        match &self.data{
            Data::Frames(data) => {//check to make sure there are the same number of data frames as there are grid points
                if data.len() != self.num_points{
                    println!("Validation failed: expected {:?} data frames, have {:?}",self.num_points,data.len());
                    self.valid = false;
                    return false
                };
                //check to make sure that every grid point has a data frame
                let mut missing = Vec::new();
                let mut counter = 0;
                for grid_number in 0..self.num_points{
                    counter += 1;
                    if !data.iter().any(|(index,_)| *index==grid_number) {
                        missing.push(grid_number)
                    }
                };
                if missing.len() > 0{
                    println!("Validation failed! Missing data frames for the following grid positions: {:?} ",missing);
                    self.valid = false;
                    false
                }else{
                    self.valid = true;
                    true
                }}
            Data::Scalars(_) => {panic!("please implement me")} //TODO
        }
        
    }

    pub fn four_point_interpolation(&self, point0:usize,point1:usize,point2:usize,point3:usize,interp:(f64,f64))-> Vec<Vec<f32>>{
        if self.valid == false{
            panic!("Must validate Grid before attempting to interpolate")
        };
        let (xi,yi) = interp;
        let (x0,y0) = self.location(point0);
        let (x1,y1) = self.location(point1);
        let (x2,y2) = self.location(point2);
        let (x3,y3) = self.location(point3);

        //check that the four points are distinct
        assert!((x0-x1).abs()> 2.0*self.snap_precision);
        assert!((y1-y2).abs()> 2.0*self.snap_precision);
        //check that the points are actually arrayed in a square
        assert!((x0-x3).abs() < self.snap_precision);
        assert!((x1-x2).abs() < self.snap_precision);
        assert!((y0-y1).abs() < self.snap_precision);
        assert!((y2-y3).abs() < self.snap_precision);
        //check that the interp location is within the square //TODO need to be specific about the margins at play
        assert!(xi > x0 + self.snap_precision/2.0);
        assert!(xi < x2 - self.snap_precision/2.0);
        assert!(yi > y2 + self.snap_precision/2.0);
        assert!(yi < y1 - self.snap_precision/2.0);

        let q0 = self.get_frame(point0).data; //TODO add in verification that the locations of the frames match with the grid
        let q1 = self.get_frame(point1).data;
        let q2 = self.get_frame(point2).data;
        let q3 = self.get_frame(point3).data;

        let x_min = x0;
        let x_max = x1;
        let y_min = y2;
        let y_max = y0;

        let weight_0 = (x_max-xi)*(yi-y_min);
        let weight_1 = (xi-x_min)*(yi-y_min);
        let weight_2 = (xi-x_min)*(y_max-yi);
        let weight_3 = (x_max-xi)*(y_max-yi);
        println!("The weights are {:?}",(weight_0,weight_1,weight_2,weight_3));
        let area = ((x_max-x_min)*(y_max-y_min));

        let interpolated_data:Vec<f32> =
            q0.into_iter().flatten().zip(
                q1.into_iter().flatten().zip(
                    q2.into_iter().flatten().zip(
                        q3.into_iter().flatten()))).map(
            |(q0,(q1,(q2,q3)))| {
            (q0*weight_0 as f32 + q1*weight_1 as f32 + q2*weight_2 as f32 + q3*weight_3 as f32)/area as f32
        }).collect();
        interpolated_data.chunks(64).map(|i| i.to_vec()).collect()
    }



/*
    pub fn print_points(&self, extra:bool){
        let mut points = Vec::new();
        let red = "red";
        let blue = "blue";
        let green = "green";
        for data_frame in self.data.clone(){
            let (x,y) = self.location(data_frame.0);
            points.push((x,y,blue))
        }
        if extra{
            let extra_x  = 0.05;
            let extra_y = 0.02;
            let (corner_0,corner_1,corner_2,corner_3) = self.find_corners(extra_x,extra_y);

            let (x0,y0) = self.location(corner_0);
            let (x1,y1) = self.location(corner_1);
            let (x2,y2) = self.location(corner_2);
            let (x3,y3) = self.location(corner_3);

            points.push((x0,y0,red));
            points.push((x1,y1,red));
            points.push((x2,y2,red));
            points.push((x3,y3,red));
            points.push((extra_x,extra_y,green));
        }

        let mut file = File::create("points").expect("Couldn't create the pretty picture file");
        write!(file, "{:?}",points).expect("Failed to write pretty picture ");



    }
    pub fn print_frames(&self, path:&str,trim_x:usize,trim_y:usize) {

        let mut pretty_picture: Vec<f32> = Vec::new();
        let (data_x, data_y) = self.data_size;
        for grid_row  in 0..self.y_num{

            let grid_index_start = grid_row*self.x_num;
            let grid_index_end = grid_row*self.x_num + self.x_num;
            println!("Grid row {grid_row}  {grid_index_start} {grid_index_end}");
            for data_row  in 0+trim_y..64-trim_y{

                for frame_number in (grid_index_start..grid_index_end){
                    let mut frame_row = self.get_frame(frame_number).data[data_row].clone();
                    assert_eq!(frame_row.len(),64);
                    pretty_picture.append(&mut frame_row[0+trim_x..64-trim_x].iter().map(|i| *i).collect())
                }
            }
        }
        println!("Display IS {:?} long",pretty_picture.len());
        let mut file = File::create(path).expect("Couldn't create the pretty picture file");
        write!(file, "{:?}",pretty_picture).expect("Failed to write pretty picture ");

    }
    
 */

    pub fn display_interpolation(&self, x:usize,y:usize){

        let mut points = Vec::new();
        let (x0,y0) = self.location(self.grid_number(x,y));
        let (x_center,y_center) = (x0-self.x_step_size/2.0,y0-self.y_step_size/2.0);
        let (x_a,y_a) = (x_center,y_center + self.y_step_size/2.0);
        let (x_b,y_b) = (x_center+self.x_step_size/2.0,y_center);
        let (x_c,y_c) = (x_center,y_center - self.y_step_size/2.0);
        let (x_d,y_d) = (x_center-self.x_step_size/2.0,y_center);
        points.push((x_a,y_a,"red"));
        points.push((x_b,y_b,"green"));
        points.push((x_c,y_c,"blue"));
        points.push((x_d,y_d,"yellow"));
        points.push((x_center,y_center,"pink"));

        let corners = match self.find_corners((x_center,y_center)){
            Corners::Four(corner_0,corner_1,corner_2,corner_3) => {vec![corner_0,corner_1,corner_2,corner_3]}
            Corners::Two(corner_0,corner_1) => {vec![corner_0,corner_1]}
            Corners::One(corner_0) => {vec![corner_0]}
            Corners::None => {vec![]}
        };

        let mut file = File::create("interp_points").expect("Couldn't create the pretty picture file");
        write!(file, "{:?}",points).expect("Failed to write pretty picture ");


    }
}



