use crate::data_frame::DataFrame;
use std::fs;
#[derive(Debug)]
pub struct Grid {
    pub x_num: usize,
    pub x_step_size: f64,
    pub y_num: usize,
    pub y_step_size: f64,
    pub center: (f64,f64),
    pub rotation: f64,
    pub data: Vec<(usize, DataFrame)>,
    pub snap_precision: f64,
}


impl Grid{
    pub fn new_empty(x_num: usize,
               x_step_size: f64,
               y_num: usize,
               y_step_size: f64,
               center: (f64,f64),
               rotation: f64,
               snap_precision: f64,) -> Grid{
        Grid{
            x_num,
            x_step_size,
            y_num,
            y_step_size,
            center,
            rotation,
            data: vec![],
            snap_precision,
        }

    }

    pub fn empty_fuv()->Grid{
        Grid{
            x_num:18,
            x_step_size:0.2,
            y_num:18,
            y_step_size:0.2,
            center:(-0.56,-0.06),
            rotation:0.0,
            data: vec![],
            snap_precision:0.01,
        }

    }

    pub fn load_fuv(directory_path:&str){
        let mut fuv_grid = Grid::empty_fuv();

        let paths = fs::read_dir(directory_path).unwrap();
        let mut counter = 0;
        for path in paths {
            counter += 1;
            let path = path.unwrap().path();
            println!("Loading file {}", path.display())


        }
        assert_eq!(counter,fuv_grid.num_points());
        println!("Found {counter} files total");

    }

    pub fn corner(&self)->(f64,f64){
        //return the coordinates in degrees of the top left corner
        let (x_0,y_0) = self.center;
        let (x_size,y_size) = self.size();
        (x_0 - x_size/2.0,y_0 - y_size/2.0)
    }
    pub fn size(&self)->(f64,f64){
        //return the size of the grid, boundaries dictated by the positions of the outermost points
        (self.x_step_size*(self.x_num-1) as f64,self.y_step_size*(self.y_num -1) as f64)
    }

    pub fn xy_indices(&self, grid_number:usize) -> (usize,usize){

        assert!((grid_number <= self.num_points() - 1)&&(grid_number >= 0));
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
        let (x_corner,y_corner) = self.corner();
        let (x_index,y_index) = self.xy_indices(grid_number);
        (x_corner + x_index as f64 *self.x_step_size,
         y_corner + y_index as f64 *self.y_step_size)

    }
    pub fn snap(&self, x_pos:f64,y_pos:f64)-> usize{
        let (x_corner,y_corner) = self.corner();
        let (x_distance,y_distance) = (x_pos-x_corner,y_pos-y_corner);
        let (x_steps,y_steps) =
            ((x_distance/self.x_step_size).round(),
             (y_distance/self.y_step_size).round());
        //panic if the point can't be snapped within precision
        //TODO: add error handling
        assert!((x_steps*self.x_step_size - x_distance)<self.snap_precision);
        assert!((y_steps*self.y_step_size - y_distance)<self.snap_precision);
        assert!(y_steps as usize <= self.y_num);
        assert!(x_steps as usize <= self.x_num);
        let index = (y_steps as usize)*self.x_num + (x_steps as usize);
        assert!(index <= self.num_points());
        index
    }

    pub fn num_points(&self)-> usize{
        self.x_num*self.y_num
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
}



