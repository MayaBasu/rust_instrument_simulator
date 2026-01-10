use ndarray::{Array1, Array2};
use memmap2::Mmap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Instant;
use memmap2::*;

pub enum EffectType {
    ComponentWiseMultiplicative,
    ComponentWiseAddition,
    Convolution
}
/*
pub struct MemoryMap{
    size: Dimension,
    memory_map: Mmap,

}

 */

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



    fn reconstitute(&self) {
        let expected_array_size = self.number_of_pixels*self.number_of_pixels*self.sample_frequencies_in_nm.len();
        println!("File content: {}", String::from_utf8_lossy(&self.data[..]));
    }




}


/*
fn apply_additive_effect<T>(effect_mmap:&Mmap, data_cube:Array<f64,T>) where T: Dimension{
        let now = Instant::now();

        let mut effect_bytes = effect_mmap[..].chunks(8);
        let flat_packed_added_data = data_cube.iter().map(|x: &f64|{
            let effect_bytes: &[u8;8] = effect_bytes.next().unwrap().try_into().expect("REASON");
            let effect_float:f64 = f64::from_le_bytes(*effect_bytes);
            //println!("x is {:?} and the bytes complie to {:?}, the addition is {:?}", x,effect_float,x+effect_float);
            x+effect_float
        }
        ).collect::<Vec<f64>>();
        let added_data= Array::from_shape_vec(data_cube.shape(), flat_packed_added_data);
        println!(" Preformed component-wise addition in {} s", now.elapsed().as_secs());
      //  println!(" the resulting added data cube is{:?}", added_data)
}

 */

/*
pub fn apply_additive_effect_mmap(data_cube_1:SpatialSpectralEffect, data_cube_2:SpatialSpectralEffect, result_file_name:&str){
    let now = Instant::now();

    let num_blocks =  1;
    let data_length = &data_cube_1.data[..].len();
    println!("data length is {:?}", data_length);

    let mut result_file = OpenOptions::new()
        .append(true)
        .open(result_file_name)
        .expect("Unable to open file");

    let mut dc_1_blocks = &data_cube_1.data[..].chunk(data_length/num_blocks);
    let mut dc_2_blocks = &data_cube_2.data[..].chunk(data_length/num_blocks);

    for block_num in 1..num_blocks{
        let data_cube_1_block = dc_1_blocks.next().

        let mut data_cube_2_bytes = data_cube_2_block.array_chunks::<8>();
        for data_cube_1_8bytes in data_cube_1_block.array_chunks::<8>(){
            /*
            let data_cube_1_8byte_array: &[u8; 8] = data_cube_2_bytes.next().unwrap().try_into().expect("REASON");
            let data_cube_2_8byte_array: &[u8; 8] = data_cube_1_8bytes.try_into().expect("REASON");

             */

            let data_cube_1_float: f64 = f64::from_le_bytes(*data_cube_1_8bytes);
            let data_cube_2_float: f64 = f64::from_le_bytes(*data_cube_2_bytes);

            let sum = data_cube_1_float + data_cube_2_float;

            result_file.write_all(&(sum).to_le_bytes()[..]).expect("REASON")
        }



    }


    let mut data_cube_2_block_slice = &data_cube_2.data[..];
    let mut data_cube_2_block_slice = &data_cube_2.data[..];

    let data_cube_1_block: [u8;320000]= data_cube_1_block_slice.try_into().expect("REASON");
    let data_cube_2_block: [u8;320000] = data_cube_2_block_slice.try_into().expect("REASON");



    println!(" Preformed component-wise addition on disk in {} s", now.elapsed().as_secs());
}

 */





pub fn add(data_cube_1:SpatialSpectralEffect, data_cube_2:SpatialSpectralEffect, result_file_name:&str){
    let now = Instant::now();

    let mut result_file = OpenOptions::new()
        .append(true)
        .open(result_file_name)
        .expect("Unable to open file");

    //How many u8's are in the memory map of data cube 1
    let flat_pack_data_size =
            data_cube_1.number_of_pixels*
            data_cube_1.number_of_pixels*
            data_cube_1.sample_frequencies_in_nm.len()*
            8;

    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 11520000000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
        flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;
    let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);
    for chunk_number in 1..num_chunks+1{

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");

       // println!("data on ram is {:?}",chunk_1);

        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let sum = float_1+float_2;
            result_chunk.push(sum);

            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;
    }




    let sum: f64 =result_chunk.iter().sum();
    println!("the result chunk is {:?}",sum);
    println!(" Preformed component-wise addition on disk in {} ms", now.elapsed().as_millis());

}



pub fn tripple_add(data_cube_1:SpatialSpectralEffect, data_cube_2:SpatialSpectralEffect,data_cube_3:SpatialSpectralEffect, result_file_name:&str){
    let now = Instant::now();

    let mut result_file = OpenOptions::new()
        .append(true)
        .open(result_file_name)
        .expect("Unable to open file");

    //How many u8's are in the memory map of data cube 1
    let flat_pack_data_size =
        data_cube_1.number_of_pixels*
            data_cube_1.number_of_pixels*
            data_cube_1.sample_frequencies_in_nm.len()*
            8;

    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 1440000000;
    let num_chunks: usize = flat_pack_data_size/chunk_size;

    println!("The flat pack data is {:?} bytes long, and will be broken into {:?} chunks of {:?} with a remaining chunk of size {:?}",
             flat_pack_data_size,
             (flat_pack_data_size/chunk_size),
             chunk_size,
             flat_pack_data_size - (flat_pack_data_size/chunk_size)*chunk_size
    );

    let mut starting_chunk_index = 0;
    let mut result_chunk: Vec<f64> = Vec::with_capacity(chunk_size);
    for chunk_number in 1..num_chunks+1{

        //move a chunk to RAM
        let chunk_1:&[u8;chunk_size] = data_cube_1.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_2:&[u8;chunk_size] = data_cube_2.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");
        let chunk_3:&[u8;chunk_size] = data_cube_3.data[starting_chunk_index..(chunk_size* chunk_number)].try_into().expect("Problem when moving data to RAM");

        // println!("data on ram is {:?}",chunk_1);

        let mut starting_byte_index = 0;
        for byte_number in 1..chunk_size/8+1{

            let byte_array_1:[u8;8] = chunk_1[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_1 = f64::from_le_bytes(byte_array_1);

            let byte_array_2:[u8;8] = chunk_2[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_2 = f64::from_le_bytes(byte_array_2);

            let byte_array_3:[u8;8] = chunk_3[starting_byte_index..(8*byte_number)].try_into().expect("problem with data conversion");
            let float_3 = f64::from_le_bytes(byte_array_3);


            let sum = float_1+float_2+float_3;
            result_chunk.push(sum);

            starting_byte_index = starting_byte_index + 8;
        }

        starting_chunk_index = starting_chunk_index + chunk_size;
    }




    let sum: f64 =result_chunk.iter().sum();
    println!("the result chunk is {:?}",sum);
    println!(" Preformed component-wise addition on disk in {} ms", now.elapsed().as_millis());

}


pub fn quad_add(data_cube_1:SpatialSpectralEffect, data_cube_2:SpatialSpectralEffect,data_cube_3:SpatialSpectralEffect, data_cube_4:SpatialSpectralEffect,result_file_name:&str){
    let now = Instant::now();

    //How many u8's are in the memory map of data cube 1
    let flat_pack_data_size =
        data_cube_1.number_of_pixels*
            data_cube_1.number_of_pixels*
            data_cube_1.sample_frequencies_in_nm.len()*
            8;

    //Now we set a chunk size, which must be a multiple of 8
    pub const chunk_size:usize = 288000000;
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

        // println!("data on ram is {:?}",chunk_1);

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

        write_to_disk(result_chunk,result_file_name);
    }

    println!(" Preformed component-wise addition on disk in {} ms", now.elapsed().as_nanos());



}

pub fn write_to_disk(data:Vec<f64>,result_file_name:&str){
    println!("Writting to disk");
    let mut result_file = OpenOptions::new()
        .append(true)
        .open(result_file_name)
        .expect("Unable to open file");

    let now = Instant::now();

    let num_floats= data.len();
    let bus_size = 100000*8;
    let num_batches = num_floats/bus_size;
    println!("num floats is {:?}batches is {:?}",num_floats,num_batches);

    for batch in 0..num_batches{

        println!("Loading the bus for batch number {:?}",batch);
        let mut bus: Vec<u8> = Vec::with_capacity(bus_size);

        for float_index in 0..bus_size{
            for byte in data[float_index].to_le_bytes(){
                bus.push(byte)
            }
        }
        result_file.write_all(&bus[..]).unwrap()
    }

    println!("Wrote data to disk in {} ms", now.elapsed().as_nanos());

}








