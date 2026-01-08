use ndarray::{Array, ArrayBase, Dim, Ix, OwnedRepr, Array3, array, Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::f64::*;
use std::io::{Bytes, Write};
use std::any::type_name;
use ndarray_rand::rand_distr::num_traits::ToBytes;
use memmap2::Mmap;
use std::fs::File;
use std::time::Instant;

enum EffectType {
    ComponentWiseMultiplicative,
    ComponentWiseAddition,
    Convolution
}

pub struct SpatialSpectralEffect {
    label: String,
    active: bool,
    effect_types: EffectType,
    number_of_pixels: usize,
    sample_frequencies_in_nm: Vec<usize>,
    data:Mmap,
}

pub struct SpectralEffect {
    label: String,
    active: bool,
    effect_types: EffectType,
    sample_frequencies_in_nm: Vec<usize>,
    data:Array1<f64>,
}
pub struct SpatialEffect {
    label: String,
    active: bool,
    effect_types: EffectType,
    number_of_pixels: usize,
    data:Array2<f64>,
}

pub trait DataCube {
    fn name(&self) -> String;
}
impl SpatialSpectralEffect {
    pub fn initialize(label:String,active:bool,effect_type: EffectType,sample_frequencies_in_nm: Vec<usize>,file: File) -> SpatialSpectralEffect{

        // Create a memory map for the file
        let now = Instant::now();
        let mmap = unsafe { Mmap::map(&file)? };
        println!("Created a memory map in {} ns", now.elapsed().as_nanos());

        SpatialSpectralEffect{label,active,effect_types, number_of_pixels,sample_frequencies_in_nm,data:mmap}

    }


    pub fn hallucinate(number_of_pixels: usize,sample_frequencies_in_nm: Vec<usize>){


        //We initialize an array the size of the quantum efficiency data cube
        let now = Instant::now();
        let hallucinated_quantum_efficiency = Array3::random(
            (number_of_pixels, number_of_pixels, sample_frequencies_in_nm.len()),
            Uniform::new(0., 1.)).to_owned();
        println!("Hallucinated Quantum Efficiency in {:?} milliseconds", now.elapsed().as_millis());


        //Converting the quantum efficiency data cube to bytes
        let now = Instant::now();
        let flat_qe= hallucinated_quantum_efficiency.iter().map(|x: &f64| x.to_le_bytes()).flatten().collect::<Vec<u8>>();
        println!("Converted hallucinated data to bytes in {} milliseconds", now.elapsed().as_millis());

        let mut f = File::create("hallucinated_data/qem").unwrap();
        f.write_all(flat_qe.as_slice()).unwrap();

        /*




        println!("unpacking");
        let now = Instant::now();
        let tented_qe: Vec<f64> = flat_qe.into_iter().map(|x: [u8;8]| f64::from_le_bytes(x)).collect();
        let qe= Array::from_shape_vec((4000, 4000,4), tented_qe.clone());
        println!("{}", now.elapsed().as_millis());


        let now = Instant::now();

         */

        /*
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
        println!("File content: {}", String::from_utf8_lossy(content));

        Ok(())

         */



    }



}





