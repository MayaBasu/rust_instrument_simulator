use std::fs;
use crate::data_frame::{DataFile, DataFrame, Load};
/*
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


 */

