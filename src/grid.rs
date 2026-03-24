use std::io::Write;
use crate::data_frame::DataFrame;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::slice::Chunks;

#[derive(Debug)]
pub struct Grid {
    pub x_num: usize,
    pub x_step_size: f64,
    pub y_num: usize,
    pub y_step_size: f64,
    pub num_points: usize,
    pub center: (f64,f64),
    pub top_left_corner: (f64,f64),
    pub rotation: f64,
    pub data: Vec<(usize, DataFrame)>,
    pub data_size: (usize,usize),
    pub snap_precision: f64,
    valid: bool,
}


impl Grid{
    pub fn new_empty(x_num: usize,
               x_step_size: f64,
               y_num: usize,
               y_step_size: f64,
               center: (f64,f64),
               rotation: f64,
                     data_size: (usize,usize),
               snap_precision: f64,) -> Grid{

        let size = (x_step_size*(x_num-1) as f64,y_step_size*(y_num -1) as f64);
        let num_points = x_num*y_num;
        let (x_0,y_0) = center;
        let (x_size,y_size) = size;
        let top_left_corner = (x_0 - x_size/2.0,y_0 - y_size/2.0);

        Grid{
            x_num,
            x_step_size,
            y_num,
            y_step_size,
            num_points,
            center,
            top_left_corner,
            rotation,
            data: vec![],
            data_size,
            snap_precision,
            valid: false,
        }

    }

    pub fn empty_fuv()->Grid{
        Grid::new_empty(
            18, //x_num
            0.2, //x_step_size
            18, //y_num
            0.2, //y_step_size
            (-0.56, -0.06),
            0.0,
            (64,64),
            0.01,
        )
    }

    pub fn load_fuv(directory_path:&str)-> Grid{
        let mut fuv_grid = Grid::empty_fuv();

        let paths = fs::read_dir(directory_path).unwrap();
        let mut counter = 0;
        for path in paths {
            counter += 1;
            let path = path.unwrap().path();
            //println!("Loading file {}", path.display());
            let (x_pixels,y_pixels) = fuv_grid.data_size;
            let frame = DataFrame::load_psf(x_pixels,y_pixels,40000.0,path);
            let frame_index = fuv_grid.snap(frame.x_pos,frame.y_pos);
            fuv_grid.data.push((frame_index,frame))
        }

        fuv_grid.data.sort_by_key(|x|x.0);

        assert_eq!(counter,fuv_grid.num_points);
        println!("Loaded {counter} files into an FUV Grid Struct from {directory_path}");
        for i in 0..fuv_grid.data.len(){
            println!("{:?}",fuv_grid.data[i].0)
        }
        fuv_grid
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

    pub fn location(&self, grid_number:usize) -> (f64,f64){
        assert!(grid_number <= self.x_num*self.y_num-1);
        let (x_corner,y_corner) = self.top_left_corner;
        let (x_index,y_index) = self.xy_indices(grid_number);
        (x_corner + x_index as f64 *self.x_step_size,
         y_corner + y_index as f64 *self.y_step_size)

    }
    pub fn snap(&self, x_pos:f64,y_pos:f64)-> usize{
        let (x_corner,y_corner) = self.top_left_corner;
        let (x_distance,y_distance) = (x_pos-x_corner,y_pos-y_corner);
        let (x_steps,y_steps) =
            ((x_distance/self.x_step_size).round(),
             (y_distance/self.y_step_size).round());
        //panic if the point can't be snapped within precision
        //TODO: add error handling
        let snap_x = (x_steps*self.x_step_size - x_distance);
        let snap_y = (y_steps*self.y_step_size - y_distance);
        println!("Snap x,y: {:?}",(snap_x,snap_y));
        assert!(snap_x<self.snap_precision);
        assert!(snap_y<self.snap_precision);
        assert!(y_steps as usize <= self.y_num);
        assert!(x_steps as usize <= self.x_num);
        let index = (y_steps as usize)*self.x_num + (x_steps as usize);
        assert!(index <= self.num_points);
        index
    }



    pub fn find_corners(&self,x:f64,y:f64) -> (usize,usize,usize,usize){
        let (x_corner,y_corner) = self.top_left_corner;
        let (x_distance, y_distance) = (x-x_corner,y-y_corner);

        let (lower_x,lower_y) = ((x_distance/self.x_step_size).floor() as usize,
                                 (y_distance/self.y_step_size).floor() as usize);
        let (upper_x,upper_y) = ((x_distance/self.x_step_size).ceil() as usize,
                                 (y_distance/self.y_step_size).ceil() as usize);

        let corner_0 = self.grid_number(lower_x,lower_y);
        let corner_1 = self.grid_number(upper_x,lower_y);
        let corner_2 = self.grid_number(upper_x,upper_y);
        let corner_3 = self.grid_number(lower_x,upper_y);


        (corner_0,corner_1,corner_2,corner_3)
    }
    pub fn get_frame(&self,grid_number:usize)-> DataFrame{

        //TODO: remove this clone?
        let (index,data_frame) = self.data[grid_number].clone();
        assert_eq!(index,grid_number);
        data_frame

    }

    pub fn bilinear_interpolation_factors(&self, x:f64,y:f64)-> (f64,(f64,f64,f64,f64), (usize,usize,usize,usize)){
        let corners = self.find_corners(x,y);
        //TODO add validation of the corners (check the x is shared etc)
        let (x1,y1) = self.location(corners.0);
        let (x2,y2) = self.location(corners.2);
        let q0 = (x2-x)*(y-y2);
        let q1 = (x-x1)*(y-y2);
        let q2 = (x-x1)*(y1-y);
        let q3 = (x2-x)*(y1-y);
        let common_factor = 1.0/((x2-x1)*(y2-y1));
        (common_factor,(q0,q1,q2,q3), corners)
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
        //check to make sure there are the same number of data frames as there are grid points
        if self.data.len() != self.num_points{
            println!("Validation failed: expected {:?} data frames, have {:?}",self.num_points,self.data.len());
            self.valid = false;
            return false
        };
        //check to make sure that every grid point has a data frame
        let mut missing = Vec::new();
        let mut counter = 0;
        for grid_number in 0..self.num_points{
            counter += 1;
            if !self.data.iter().any(|(index,_)| *index==grid_number) {
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
        }
    }
    /*
    pub fn interpolate(&self,x:f64,y:f64) -> Vec<f32>{
        if self.valid == false{
            panic!("Must validate Grid before attempting to interpolate")
        };
        let (common_factor,(q0,q1,q2,q3), corners) = self.bilinear_interpolation_factors(x,y);
        let (Q0,Q1,Q2,Q3) = (self.get_frame(corners.0),
                             self.get_frame(corners.1),
                             self.get_frame(corners.2),
                             self.get_frame(corners.3));

        let (Q0,Q1,Q2,Q3) = (Q0.data,Q1.data,Q2.data,Q3.data);

        let result: Vec<f32> = Q0.iter()
            .zip(Q1.iter())
            .zip(Q2.iter())
            .zip(Q3.iter())
            .map(|(((Q0,Q1),Q2),Q3)|
                     (Q0*q0 as f32 + Q1*q1 as f32 + Q2*q2 as f32 + Q3*q3 as f32)*common_factor as f32
             ).collect();
        result
    }

     */
    pub fn print_points(&self){

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
        println!("DISLAPY IS {:?} long",pretty_picture.len());
        let mut file = File::create(path).expect("Couldn't create the pretty picture file");
        write!(file, "{:?}",pretty_picture).expect("Failed to write pretty picture ");

    }
}



