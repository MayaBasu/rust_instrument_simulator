#![allow(warnings)]
//gjh
mod effects;
use bytes::Buf;
mod data_cube_management;
mod hallucinations;
pub const b:usize = 8;
use rayon::prelude::*;
use csv::*;
mod linewisedatagen;

use std::io::{BufRead, BufReader, BufWriter, Write};

use std::time::Instant;

use ndarray::prelude::*;
use std::thread;
use zerocopy::IntoBytes;


use memmap2::Mmap;
use std::fs::File;


pub const float_size:usize = 8;  //number of bytes in one float


fn name_gen(linesize:usize,num_lines:usize,delinator:&str) -> String {
    (num_lines.to_string() + "_lines_" + linesize.to_string().as_str()+ "_columns_" + delinator + ".txt").to_string()
}


fn main() {
    pipline(false)
}



fn pipline(hallucinate_data:bool){
    pub const chunk_size:usize = 1024 * 1024;
    pub const linesize:usize = 12000; //number of matrix columns
    pub const num_lines:usize = 12000; //number of matrix rows

    let name1 = name_gen(linesize,num_lines,"A_bytes"); //standard file names
    let name2 = name_gen(linesize,num_lines,"B_bytes");

    println!("Generating files {name1} and {name2}...");
    //generate files if they haven't been made already
    if hallucinate_data {
        linewisedatagen::byte_version(linesize,num_lines,name1.as_str());
        linewisedatagen::byte_version(linesize,num_lines,name2.as_str());
    }
    println!("adding the files");

    let result_file = File::create("result").unwrap(); //file to write the result to
    let mut result_buf = BufWriter::new(result_file);


    let file1 = File::open(name1).unwrap();
    let file2 = File::open(name2).unwrap();
    let mmap1 = unsafe { Mmap::map(&file1).unwrap() };
    let mmap2 = unsafe { Mmap::map(&file2).unwrap() };



    let now = Instant::now();
    let mut final_sum = 0.0;
    thread::scope(|s|{
    for (chunk1,chunk2) in mmap1.chunks(chunk_size).zip(mmap2.chunks(chunk_size)) {
            let result_chunk:f64 = s.spawn(move ||{process_chunk(chunk1,chunk2)}).join().unwrap();
            final_sum += result_chunk;
           // s.spawn(move||{write_chunk(&result_chunk,&result_buf)}).join().unwrap();
        }
    });
    println!("{final_sum}");
    result_buf.flush().unwrap();
    println!("computation took {:?}",now.elapsed().as_millis());
}

fn process_chunk(chunk1:&[u8],chunk2:&[u8]) -> f64{
    let bytes1 = chunk1.chunks(float_size);
    let bytes2 = chunk2.chunks(float_size);
    let result_chunk: Vec<f64> = bytes1.zip(bytes2).map(|(mut byte1,mut byte2)|{
        let float1 = byte1.try_get_f64().unwrap();
        let float2 = byte2.try_get_f64().unwrap();
        float1 + float2
    }).collect();
    let sum:f64 = result_chunk.into_iter().sum();

    sum


    /*
    let bytes1 = chunk1.into_par_iter().chunks(float_size);
    let bytes2 = chunk2.into_par_iter().chunks(float_size);

     */

    //result_chunk

}
/*

fn write_chunk(result:&Vec<f64>, mut result_buf:&BufWriter<File>){

    for result_element in result{
        result_buf.write_all(&result_element.to_be_bytes()[..]).unwrap()
    }
}

 */


/*
fn stream_with_mmap(file1: &str, file2: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // Process the data (mmap derefs to &[u8])
    for chunk in mmap.chunks(1024 * 1024) {
        // Your processing here - this is zero-copy
        process_chunk(chunk);
    }
    Ok(())
}

 */









