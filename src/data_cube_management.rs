use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Instant;
use memmap2::Mmap;

use crate::float_size;
use crate::hallucinations::hallucinate_spatial_spectral;

pub enum ElementWiseCombinationType {
    ComponentWiseMultiplicative,
    ComponentWiseAddition,
}



fn float_combine(floats:Vec<f64>, element_wise_combination_type: &ElementWiseCombinationType) -> f64{
    match element_wise_combination_type{
        ElementWiseCombinationType::ComponentWiseMultiplicative => {floats.iter().sum()}
        ElementWiseCombinationType::ComponentWiseAddition => {
            let mut product = 1.0;
            for float in floats{product *= float}
            product
        }
    }

}

pub struct SpatialSpectralEffect {
    label: String,
    active: bool,
    number_of_pixels: usize,
    sample_frequencies_in_nm: Vec<usize>,
    data:Mmap,
}

impl SpatialSpectralEffect{
    pub fn flat_pack_size(&self)-> usize{
        self.number_of_pixels*
            self.number_of_pixels*
            self.sample_frequencies_in_nm.len()*
            float_size
    }

}
impl SpatialSpectralEffect {
    pub fn initialize(label:String,active:bool,number_of_pixels:usize,sample_frequencies_in_nm: Vec<usize>) -> SpatialSpectralEffect{

        let string = "/Users/mayabasu/Desktop/data/".to_owned();

        let input = string + &label;
        File::create(&input);
        hallucinate_spatial_spectral(&input.clone(),number_of_pixels,sample_frequencies_in_nm.clone());

        let file = File::open(input).unwrap();

        // Create a memory map for the file
        let now = Instant::now();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        println!("Created a memory map in {} ms", now.elapsed().as_millis());

        SpatialSpectralEffect{label,active,number_of_pixels,sample_frequencies_in_nm,data:mmap}

    }







    fn reconstitute(&self) {
        let expected_array_size = self.number_of_pixels*self.number_of_pixels*self.sample_frequencies_in_nm.len();
        println!("File content: {}", String::from_utf8_lossy(&self.data[..]));
    }




}


pub fn combine_data_cubes(data_cubes:Vec<SpatialSpectralEffect>, combination_type: ElementWiseCombinationType, result_file_name:&str){
    println!("WLKEJHrliwuehfjwnekf");
    /*
    Takes in a vector of data cubes and combines them in the specified manner (a CombinationType)

    Checks:
    1. 2 or more data cubes are inputted
    2. All data cubes have the same size and are for the same frequencies

    Writes the result to "result_file_name"
     */

    let now = Instant::now();

    //assert_eq!(data_cubes.len() < 2, false, "Must Pass in at least 2 cubes");
    let flat_pack_data_size = data_cubes[0].flat_pack_size();


    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 8*1000000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;

    for chunk_number in 1..num_chunks+1{
        let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);

        //move a chunk to RAM
        let mut ram_data_cubes = Vec::new();
        for data_cube in &data_cubes{
            let rammed_data:&[u8;chunk_size] = data_cube.data[starting_chunk_index..(chunk_size* chunk_number)]
                .try_into()
                .expect("Problem when moving data to RAM");
            ram_data_cubes.push(rammed_data)
        }

        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let mut floats = Vec::new();

            for rammed_data_cube in &ram_data_cubes{
                let byte_array:[u8;8] = rammed_data_cube[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
                let float = f64::from_le_bytes(byte_array);
                floats.push(float);

            }

            result_chunk.push(float_combine(floats,&combination_type));
            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;

        write_to_disk(result_chunk, result_file_name);
    }

    println!("arbitrary  Preformed component-wise addition or mulitpication on disk in {} ms", now.elapsed().as_millis());



}




pub fn unrolled_combine_data_cubes(data_cube_1:SpatialSpectralEffect, data_cube_2:SpatialSpectralEffect,data_cube_3:SpatialSpectralEffect, data_cube_4:SpatialSpectralEffect, combination_type: ElementWiseCombinationType, result_file_name:&str){
    println!("WLKEJHrliwuehfjwnekf");

    let now = Instant::now();


    let flat_pack_data_size = data_cube_1.flat_pack_size();


    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 1152000000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;

    for chunk_number in 1..num_chunks+1{
        let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_3:&[u8;chunk_size] = data_cube_3.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_4:&[u8;chunk_size] = data_cube_4.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");


        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let byte_array_3:[u8;8] = chunk_3[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_3 = f64::from_le_bytes(byte_array_3);


            let byte_array_4:[u8;8] = chunk_4[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_4 = f64::from_le_bytes(byte_array_4);



            let sum = float_1+float_2+float_3+float_4;
            result_chunk.push(sum);


            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;

        write_to_disk(result_chunk, result_file_name);
    }

    println!(" Preformed component-wise addition or mulitpication on disk in {} ms", now.elapsed().as_millis());
}
pub fn oct_unrolled_combine_data_cubes(data_cube_1:SpatialSpectralEffect,
                                       data_cube_2:SpatialSpectralEffect,
                                       data_cube_3:SpatialSpectralEffect,
                                       data_cube_4:SpatialSpectralEffect,
                                       data_cube_5:SpatialSpectralEffect,
                                       data_cube_6:SpatialSpectralEffect,
                                       data_cube_7:SpatialSpectralEffect,
                                       data_cube_8:SpatialSpectralEffect,
                                       combination_type: ElementWiseCombinationType, result_file_name:&str){
    println!("WLKEJHrliwuehfjwnekf");

    let now = Instant::now();


    let flat_pack_data_size = data_cube_1.flat_pack_size();


    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 8*10000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;

    for chunk_number in 1..num_chunks+1{
        let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_3:&[u8;chunk_size] = data_cube_3.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_4:&[u8;chunk_size] = data_cube_4.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_5:&[u8;chunk_size] = data_cube_5.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_6:&[u8;chunk_size] = data_cube_6.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_7:&[u8;chunk_size] = data_cube_7.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_8:&[u8;chunk_size] = data_cube_8.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");


        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let byte_array_3:[u8;8] = chunk_3[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_3 = f64::from_le_bytes(byte_array_3);


            let byte_array_4:[u8;8] = chunk_4[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_4 = f64::from_le_bytes(byte_array_4);

            let byte_array_5:[u8;8] = chunk_5[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_5 = f64::from_le_bytes(byte_array_5);

            let byte_array_6:[u8;8] = chunk_6[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_6 = f64::from_le_bytes(byte_array_6);

            let byte_array_7:[u8;8] = chunk_7[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_7 = f64::from_le_bytes(byte_array_7);


            let byte_array_8:[u8;8] = chunk_8[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_8 = f64::from_le_bytes(byte_array_8);




            let sum = float_1+float_2+float_3+float_4+float_5+float_6+float_7+float_8;
            result_chunk.push(sum);


            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;

        write_to_disk(result_chunk, result_file_name);
    }

    println!(" Preformed component-wise addition or mulitpication on disk in {} ms", now.elapsed().as_millis());
}



pub fn write_to_disk(data:Vec<f64>,result_file_name:&str){
    //println!("Writting to disk");
    let mut result_file = OpenOptions::new()
        .append(true)
        .open(result_file_name)
        .expect("Unable to open file");

    let now = Instant::now();

    let num_floats= data.len();
    let bus_size = 10000*8;
    let num_batches = num_floats/bus_size;
   // println!("num floats is {:?}batches is {:?}",num_floats,num_batches);

    for batch in 0..num_batches{

        //println!("Loading the bus for batch number {:?}",batch);
        let mut bus: Vec<u8> = Vec::with_capacity(bus_size);

        for float_index in 0..bus_size{
            for byte in data[float_index].to_le_bytes(){
                bus.push(byte)
            }
        }
        result_file.write_all(&bus[..]).unwrap()
    }

   // println!("Wrote data to disk in {} ms", now.elapsed().as_nanos());

}




fn verify_data_cube_compatibility(data_cubes:Vec<SpatialSpectralEffect>) {
    let num_cubes = data_cubes.len();
    let num_pixles:Vec<usize> = data_cubes.iter().map(|cube:&SpatialSpectralEffect|cube.number_of_pixels).collect();
    let frequencies:Vec<Vec<usize>> = data_cubes.iter().map(|cube:&SpatialSpectralEffect|cube.sample_frequencies_in_nm.clone()).collect();

    //check that the data cubes have the same size
    assert_eq!(num_pixles.iter().min(), num_pixles.iter().max());
}



pub fn wide_compare_quad_unrolled_combine_data_cubes(data_cube_1:SpatialSpectralEffect,
                                       data_cube_2:SpatialSpectralEffect,
                                       data_cube_3:SpatialSpectralEffect,
                                       data_cube_4:SpatialSpectralEffect,
                                       combination_type: ElementWiseCombinationType, result_file_name:&str){
    println!("WLKEJHrliwuehfjwnekf");

    let now = Instant::now();


    let flat_pack_data_size = data_cube_1.flat_pack_size();


    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 8*10000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;

    for chunk_number in 1..num_chunks{
        let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index+20..(chunk_size* chunk_number+20)].try_into().expect("Problem when moving data to RAM");
        let chunk_3:&[u8;chunk_size] = data_cube_3.data[starting_chunk_index+40..(chunk_size* chunk_number+40)].try_into().expect("Problem when moving data to RAM");
        let chunk_4:&[u8;chunk_size] = data_cube_4.data[starting_chunk_index+100..(chunk_size* chunk_number+100)].try_into().expect("Problem when moving data to RAM");


        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let byte_array_3:[u8;8] = chunk_3[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_3 = f64::from_le_bytes(byte_array_3);


            let byte_array_4:[u8;8] = chunk_4[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_4 = f64::from_le_bytes(byte_array_4);



            let sum = float_1+float_2+float_3+float_4;
            result_chunk.push(sum);


            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;

        write_to_disk(result_chunk, result_file_name);
    }

    println!(" Preformed component-wise addition or mulitpication on disk in {} ms", now.elapsed().as_millis());
}






pub fn wide_quad_unrolled_combine_data_cubes(data_cube_1:SpatialSpectralEffect,
                                                     data_cube_2:SpatialSpectralEffect,
                                                     data_cube_3:SpatialSpectralEffect,
                                                     data_cube_4:SpatialSpectralEffect,
                                                     combination_type: ElementWiseCombinationType, result_file_name:&str){
    println!("WLKEJHrliwuehfjwnekf");

    let now = Instant::now();


    let flat_pack_data_size = data_cube_1.flat_pack_size();


    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 8*10000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;

    for chunk_number in 1..num_chunks+1{
        let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_3:&[u8;chunk_size] = data_cube_3.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_4:&[u8;chunk_size] = data_cube_4.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");


        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let byte_array_3:[u8;8] = chunk_3[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_3 = f64::from_le_bytes(byte_array_3);


            let byte_array_4:[u8;8] = chunk_4[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_4 = f64::from_le_bytes(byte_array_4);



            let sum = float_1+float_2+float_3+float_4;
            result_chunk.push(sum);


            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;

        write_to_disk(result_chunk, result_file_name);
    }

    println!(" Preformed component-wise addition or mulitpication on disk in {} ms", now.elapsed().as_millis());
}






