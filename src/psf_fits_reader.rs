use uvex_fitrs::*;


#[derive(Debug)]
pub struct Grid {
    x_start: f64, //location of the left most box center
    x_end: f64, //location of the right most box center
    x_num: usize, //how many box centers there are total
    y_start: f64,
    y_end: f64,
    y_num: usize,
    precision: usize,
}




impl Grid{
    pub fn pretty_print(&self){
        if self.x_num == 1{
            panic!("Diva Down! x_num must be at least 2");
        }
        let x_increment = (self.x_end - self.x_start )/(self.x_num as f64-1.0);
        let y_increment = (self.y_end - self.y_start )/(self.y_num as f64 -1.0);

        let x_pos:Vec<f64> = (0..self.x_num).into_iter().map(|i|{
        let x = self.x_start + i as f64 * x_increment;
        (x * 10_f64.powf(self.precision as f64)).round() /(10_f64.powf(self.precision as f64))}).collect();

        let y_pos:Vec<f64> = (0..self.y_num).into_iter().map(|i|{
            let y = self.y_start + i as f64 * y_increment;
            (y * 10_f64.powf(self.precision as f64)).round() /(10_f64.powf(self.precision as f64))}).collect();


        println!("The x positions are {:?}",x_pos);
        println!("y pos are {:?}",y_pos);

    }

}
#[derive(Debug)]
pub struct PSF {
    data: Vec<f32>,
    size: (usize,usize),
    xpos: f64,
    ypos: f64,
    scale: f64,
}
pub struct PSF_data{
    PSFs: Vec<PSF>
}
pub fn open_psf_directory(bottom:usize, top:usize) ->PSF_data{

    let fits_paths:Vec<String>= (bottom..top).into_iter().filter(|i|
    (*i !=37) && (*i !=74) && (*i !=111) && (*i !=130)  && (*i !=167) && (*i !=204)  && (*i !=223)
        && (*i !=260) && (*i !=297)  && (*i !=316)
    ).map(|x| {

        let x =  format!("{:0>3}", x);
        let mut string = "/Users/mayabasu/RustroverProjects/image_simulator_outline/data/demo/demo_psf/FUV PSF/UVEX_FUV_PSF_1um_F".to_string();
        string.push_str(x.as_str());
        string.push_str(".fits");
        string
    }
    ).collect();
    open_psf_fits(fits_paths)

}
pub fn open_psf_fits(paths:Vec<String>) -> PSF_data {
    let mut xpositions = Vec::new();
    let mut ypositions = Vec::new();
    let psf_data:Vec<PSF> = paths.iter().map(|path|{
        println!("opening {:?}",path);

        let fits = uvex_fitrs::Fits::open(path).expect("Failed to open PSF FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        //println!("{:?}",primary_hdu);
        let primary_hdu_data = match primary_hdu.read_data() {
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
                (data,shape)
            }
            _ => {panic!("Could not unpack PSF data") }
        };
        let xpos = match primary_hdu.value("XFLD").expect("failed to get xpos") {
            HeaderValue::RealFloatingNumber(xpos)=> xpos,
            _ => panic!("could not unpack xpos")
        };

        if !xpositions.contains(xpos){
            xpositions.push(*xpos)
        }


        let ypos = match primary_hdu.value("YFLD").expect("failed to get xpos") {
            HeaderValue::RealFloatingNumber(ypos)=> ypos,
            _ => panic!("could not unpack ypos")
        };
        if !ypositions.contains(ypos){
            ypositions.push(*ypos)
        }
        let psf = PSF {
            data: primary_hdu_data.0,
            size: (primary_hdu_data.1[0],primary_hdu_data.1[1]),
            xpos: *xpos,
            ypos: *ypos,
            scale: 10.0,
        };
        println!("XPOS: {:?}",xpos);
        println!("YPOS: {:?}",ypos);
        psf
    }).collect();
    println!("x positions, deg: {:?}, {:?} total",xpositions,xpositions.len());
    println!("y pos deg: {:?}, {:?} total",ypositions,ypositions.len());
    println!("Loaded  {:?} psf files",psf_data.len());

    let grid = Grid{
        x_start: -2.26,
        x_end: 1.14,
        x_num: 18,
        y_start: -1.76,
        y_end: 1.64,
        y_num: 18,
        precision: 2,
    };
    grid.pretty_print();
    PSF_data{PSFs:psf_data}

}

impl PSF_data{
    pub fn get_psf(xpos:f64,ypos:f64){

    }
}

