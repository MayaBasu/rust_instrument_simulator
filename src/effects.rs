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
use memmap2::*;

pub enum EffectType {
    ComponentWiseMultiplicative,
    ComponentWiseAddition,
    Convolution
}

pub struct SpatialSpectralEffect {
    label: String,
    active: bool,
    effect_type: EffectType,
    number_of_pixels: usize,
    sample_frequencies_in_nm: Vec<usize>,
    data:Mmap,
}

pub struct SpectralEffect {
    label: String,
    active: bool,
    effect_type: EffectType,
    sample_frequencies_in_nm: Vec<usize>,
    data:Array1<f64>,
}
pub struct SpatialEffect {
    label: String,
    active: bool,
    effect_type: EffectType,
    number_of_pixels: usize,
    data:Array2<f64>,
}

pub trait DataCube {
    fn name(&self) -> String;
}
impl SpatialSpectralEffect {
    pub fn initialize(label:String,active:bool,effect_type: EffectType,number_of_pixels:usize,sample_frequencies_in_nm: Vec<usize>,file_name: &str) -> SpatialSpectralEffect{
        let file = File::open(file_name).unwrap();
        // Create a memory map for the file
        let now = Instant::now();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        println!("Created a memory map in {} ms", now.elapsed().as_millis());

        SpatialSpectralEffect{label,active,effect_type, number_of_pixels,sample_frequencies_in_nm,data:mmap}

    }

    pub fn apply(&self, data_cube:Array3<f64>){

        let resultant_data_cube = match self.effect_type {
            EffectType::ComponentWiseAddition => {
                let now = Instant::now();
                //println!("data_cube is {:?}", data_cube);
                //let flat_pack_data_cube = data_cube.iter().map(|x: &f64| x).collect::<Vec<&f64>>();
                //println!("flat_pack is {:?}", flat_pack_data_cube);
                let mut effect_bytes = self.data[..].chunks(8);
                let flat_packed_added_data = data_cube.iter().map(|x: &f64|{
                    let effect_bytes: &[u8;8] =effect_bytes.next().unwrap().try_into().expect("REASON");
                    let effect_float:f64 = f64::from_le_bytes(*effect_bytes);
                    println!("x is {:?} and the bytes complie to {:?}, the addition is {:?}", x,effect_float,x+effect_float);
                    x+effect_float
                }
                ).collect::<Vec<f64>>();
                let added_data= Array::from_shape_vec(data_cube.shape(), flat_packed_added_data);



                println!(" Preformed component-wise addition in {} ms", now.elapsed().as_millis());
                println!(" the resulting added data cube is{:?}", added_data)

            },
            EffectType::ComponentWiseMultiplicative => {},
            EffectType::Convolution =>{}
        };
    }

    fn reconstitute(&self) {
        let expected_array_size = self.number_of_pixels*self.number_of_pixels*self.sample_frequencies_in_nm.len();
        println!("File content: {}", String::from_utf8_lossy(&self.data[..]));
    }




}





