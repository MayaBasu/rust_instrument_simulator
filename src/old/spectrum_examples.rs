use plotpy::Plot;
use crate::datafile_reader::FrequencyFile;
use crate::grid1d::GRID1D;
/*
pub fn show_interpolation(){
    let path= "/Users/mayabasu/Desktop/uvex_psf_files/example.dat";
    let data_grid = GRID1D::new_empty(7, 10.0, 120.0, 0.01, 1.0);

    let mut plot = Plot::new();

    let mut file = FrequencyFile::new_from_grid(path, data_grid);
    let mut file = file.load_data(false);
    println!("{:?}",file);
    let curve1 = file.plot("red","none");

    println!("got first plot");
    let new_grid = GRID1D::new_empty(100,1.0,100.0,0.01,1.0);

    let file = file.re_grid(new_grid);
    println!("{:?}",file);
    let curve2 = file.plot("green","dotted");

    for curve in curve2{
        println!("adding curve");
        plot.add(&curve);
    }
    for curve in curve1{
        plot.add(&curve);

    }





    plot.show("slefje").unwrap();


}

 */