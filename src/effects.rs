use ndarray::{Array, ArrayBase, Dim, Ix, OwnedRepr, Array3, array};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::f64::*;
use std::io::Bytes;
use std::any::type_name;
use ndarray_rand::rand_distr::num_traits::ToBytes;
use memmap2::Mmap;
use std::fs::File;



use std::time::Instant;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


pub struct QuantumEfficiency {
    label: String,
    active: bool,
    number_of_pixels: usize,
    sample_frequencies_in_nm: Vec<usize>,
    quantum_efficiency:Array3<f64>,
}

impl QuantumEfficiency {
    pub fn hallucinate(label:String, number_of_pixels: usize,sample_frequencies_in_nm: Vec<usize>)->std::io::Result<()> {

        let quantum_efficiency= Array3::random(
            (number_of_pixels, number_of_pixels, sample_frequencies_in_nm.len()),
            Uniform::new(0., 1.)).to_owned();

        /*

        println!("packing");
        let now = Instant::now();
        let flat_qe= quantum_efficiency.iter().map(|x: &f64| x.to_le_bytes()).collect::<Vec<[u8;8]>>();
        println!("{}", now.elapsed().as_millis());


        println!("unpacking");
        let now = Instant::now();
        let tented_qe: Vec<f64> = flat_qe.into_iter().map(|x: [u8;8]| f64::from_le_bytes(x)).collect();
        let qe= Array::from_shape_vec((4000, 4000,4), tented_qe.clone());
        println!("{}", now.elapsed().as_millis());

         */

        println!("writting to a memory map");
        let now = Instant::now();
        let file = File::open("hallucinated_data/qe")?;

        // Create a memory map for the file
        let mmap = unsafe { Mmap::map(&file)? };

        println!("{}", now.elapsed().as_nanos());

        println!("reading from to a memory map");
        let now = Instant::now();

        // Access file content as a byte slice
        let content = &mmap[..];

        println!("{}", now.elapsed().as_nanos());

        // Print it to the console
        //println!("File content: {}", String::from_utf8_lossy(content));
        Ok(())






        //let qe = QuantumEfficiency{label,active:false, number_of_pixels,sample_frequencies_in_nm,quantum_efficiency};


    }
    /*

    pub fn serialize(&self,filename:&str)-> std::io::Result<()> {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = File::create(filename)?;
            file.write_all(serialized.as_bytes())?;
            Ok(())
    }

     */
/*
pub fn load(filename:&str) -> QuantumEfficiency {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let deserialized = serde_json::from_reader(buf_reader);
    deserialized.unwrap()
}

 */


}


