use tokio::io::{self, AsyncWrite};
mod effects;
mod data_cube_management;
mod hallucinations;
pub const b:usize = 8;
use csv::*;
mod linewisedatagen;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use memmap2::Mmap;
use ndarray::prelude::*;
use zerocopy::IntoBytes;
use data_cube_management::SpatialSpectralEffect;
use crate::hallucinations::hallucinate_spatial_spectral;
pub const float_size:usize = 8;  //number of bytes in one float
fn name_gen(linesize:usize,num_lines:usize,delinator:&str) -> String {
    (num_lines.to_string() + "_lines_" + linesize.to_string().as_str()+ "_columns_" + delinator + ".txt").to_string()

}

fn bytes_method(){

    const linesize:usize = 12000;
    const num_lines:usize = 12000;



    let name1 = name_gen(linesize,num_lines,"A_bytes");
    let name2 = name_gen(linesize,num_lines,"B_bytes");

    println!("Generating files {name1} and {name2}...");
    linewisedatagen::byte_version(linesize,num_lines,name1.as_str());
    linewisedatagen::byte_version(linesize,num_lines,name2.as_str());
    println!("adding the files");



    let file1 = File::open(name1.clone()).unwrap();
    let file2 = File::open(name2.clone()).unwrap();

    let result_file = File::create("result").unwrap();

   // let mut file = File::create("foo.txt").unwrap();

    let mut result_buf = BufWriter::new(result_file);
    let mmap1 = unsafe { Mmap::map(&file1).unwrap() };
    let mmap2 = unsafe { Mmap::map(&file2).unwrap() };

    /*
    let (data1, []) = &mmap1[..].as_chunks::<b>() else {
        panic!("slice didn't have even length")
    };
    let (data2, []) = &mmap2[..].as_chunks::<b>() else {
        panic!("slice didn't have even length")
    };

     */
  //  let mut final_sum = 0.0;
    let now = Instant::now();
    use rayon::prelude::*;

    let result: Vec<f64> = (0..num_lines*linesize).into_par_iter().map(|i| {
        let start = i*8;
        let end = start+8;
        let e1 = f64::from_le_bytes(mmap1[start..end].try_into().unwrap());
        let e2 = f64::from_le_bytes(mmap2[start..end].try_into().unwrap());
        let addition = e1 + e2;
        addition

    })
        .collect();



        // Write data

        // Ensure all data is written


    println!("computation took {:?}",now.elapsed().as_millis());

    for result_element in result{

        result_buf.write_all(&result_element.to_be_bytes()[..]).unwrap()

    }
    result_buf.flush();
    println!("computation took {:?}",now.elapsed().as_millis());




   // result_buf.write(result_array)

    }




fn cv_method(){
    const linesize:usize = 12000;
    const num_lines:usize = 12000;
    const write_chunk_size:usize = 1;

    let name1 = name_gen(linesize,num_lines,"Ac");
    let name2 = name_gen(linesize,num_lines,"Bc");

    println!("Generating files {name1} and {name2}...");
   // linewisedatagen::example(linesize,num_lines,name1.as_str());
  //  linewisedatagen::example(linesize,num_lines,name2.as_str());
    println!("adding the files");



    let file1 = File::open(name1.clone()).unwrap();
    let file2 = File::open(name2.clone()).unwrap();
    let result_file = File::create("result").unwrap();
    let mut result_buf = BufWriter::new(result_file);
    let mmap1 = unsafe { Mmap::map(&file1).unwrap() };
    let mmap2 = unsafe { Mmap::map(&file2).unwrap() };

    /*
    let (data1, []) = &mmap1[..].as_chunks::<b>() else {
        panic!("slice didn't have even length")
    };
    let (data2, []) = &mmap2[..].as_chunks::<b>() else {
        panic!("slice didn't have even length")
    };

     */
    let now = Instant::now();
    let mut lines2 = mmap2.split(|c| *c==b'\n');
    for line in mmap1.split(|c| *c==b'\n'){


        if line.is_empty(){
            break;
        }
        let line2 = lines2.next().unwrap();
        let elements = line.split(|c| *c==b',');
        let mut elements2 = line2.split(|c| *c==b',');

        for element in elements{
            let element2 = elements2.next();
            let num1 = str::from_utf8(element).unwrap().parse::<f64>().unwrap();
            let num2 = str::from_utf8(element2.unwrap()).unwrap().parse::<f64>().unwrap();
            let addition = num1+num2;
            result_buf.write(&addition.to_be_bytes()[..]);

        }
        // Write data

        // Ensure all data is written

    }
    result_buf.flush();

    println!("took {:?}",now.elapsed().as_secs());

}





fn main() {
   // bytes_method()
    bytes_method()



/*
    for i in 0..data1.len(){
        let e1 = f64::from_be_bytes(data1[i]);
        let e2 = f64::from_be_bytes(data2[i]);
        let result:f64 = e1+e2;
       // println!("e1 {:?}   e2 {:?}",e1,e2);
        result_buf.write_all(&result.to_be_bytes()[..]).unwrap()
    }

 */







   // let f1 = File::open(name1).unwrap();




    //let f1 = BufReader::new(file1);

    /*
    for line in f1.lines(){
        println!("line is {:?}",line.unwrap().into_bytes())
    }
    let file1 = File::open(name1).unwrap();

     */


 //   println!("ljs;efjesl {:?}   {:?}", b',',  b'\n');
//    for i in 0..40{
 //       println!("{}",mmap1[i]);
  //  }












  //  let file2 = File::open(name2).unwrap();
  //  let mmap2 = unsafe { Mmap::map(&file2).unwrap() };

    /*

    for line_number in 0..num_lines{
        for element in 0..linesize{
            // The starting index into the mmap is the addition of
            /*
            1. The number of new line characters, each getting one byte
            2. Each element prior included 18 bytes
            3. Each comma prior was a byte

            So in total we want the starting index to be 19*element + line_number, and the ending index to be the starting index + 18
             */

            let starting_index = line_number + 19*element;
            let ending_index = starting_index + 18;
            println!("start: {starting_index} end: {ending_index}");
            let result = str::from_utf8(&mmap1[starting_index..ending_index]).unwrap().parse::<f64>().unwrap();
            let result2 = str::from_utf8(&mmap2[starting_index..ending_index]).unwrap().parse::<f64>().unwrap();
            println!("Element 1 is {:?} and element 2 is {:?}", result, result2)

        }
        println!("New line")
    }

     */


    // let f1 = BufReader::new(f1);

   // let f2 = File::open(name2).unwrap();
   // let f2 = BufReader::new(f2);


    //let result_file = File::create("result").unwrap();
   // let mut result_file = BufWriter::new(result_file);








   // let mut result_data:[f64;linesize]  = [0.0;linesize];
   // let mut result_data:[&[u8];linesize*write_chunk_size]  = [&[0];linesize*write_chunk_size];
  //  let mut line_num_ticker = 0;

    /*

    println!("new line is {:?}", b'\n');

    let mut lines2 = f2.split(b'\n');
    for line1 in f1.split(b'\n'){
        let line1 = line1.unwrap();
        let line2 = lines2.next().unwrap().unwrap();
        let result = str::from_utf8(&line1[0..18]).unwrap().parse::<f64>().unwrap();
        let result2 = str::from_utf8(&line1[0..18]).unwrap().parse::<f64>().unwrap();
        println!("line is {:?} ,{:?}",line1 ,str::from_utf8(&line1[0..18]).unwrap());
        println!("The result is {}",result+result2);

     */
        /*






        let mut line1_split = line1.split(|c| *c ==b',');
        let mut line2_split = line2.split(|c| *c ==b',');
        for element in 0..linesize{
            let element1 = line1_split.next().unwrap();
            let element2 = line2_split.next().unwrap();
            println!("e1 {:?}  e2{:?} ",str::from_utf8(&element2[..]).unwrap(), str::from_utf8(&element1[..]).unwrap());
           // let element1 = str::from_utf8(line1_split.next().unwrap()).unwrap().parse::<f64>().unwrap();
           // let element2 = str::from_utf8(line2_split.next().unwrap()).unwrap().parse::<f64>().unwrap();

         //   println!("elemet 1 and 2{:?}, {:?}",element1,element2);
        }
        println!("new row")

     */



    }/*

    let mut lines2 = f2.split(b'\n');
    for line1 in f1.split(b'\n'){
        println!("line {:?}",line1);
        let line1 = line1.unwrap();
        let line2 = lines2.next().unwrap().unwrap();
        println!("line w {:?}",line1);
        let mut line1_split = line1.rsplitn(2, |c| *c==b',');
        let mut line2_split = line2.rsplitn(2, |c| *c==b',');

        for i in 0..linesize{
            println!("lj;oijl{:?}",line1_split.next().unwrap());
           // println!("{:?}",line1_split);
            let sum: f64 = str::from_utf8(line1_split.next().unwrap()).unwrap().parse::<f64>().unwrap() +
                str::from_utf8(line2_split.next().unwrap()).unwrap().parse::<f64>().unwrap();
            result_data[i] = sum;
          //  println!("sum as bytes{:?}",sum.as_bytes())

        }
       // println!("result {:?}",result_data.as_bytes());


        //if line_num_ticker%write_chunk_size ==0{
        result_file.write(result_data.as_bytes()).unwrap();

        if line_num_ticker%10 ==0{
            result_file.flush().unwrap();
            //}

        }

        line_num_ticker +=1;




    }
    */

   // println!("completed task in {:?}", now.elapsed().as_secs());








    /*
    let num_pixles:usize = 12000;
    let frequencies = vec![1];


    let result = "/Users/mayabasu/Desktop/data/output.txt";

    File::create(result);


    let qe = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());

    let qe2 = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());

    let qe3 = SpatialSpectralEffect::initialize("lske1".to_string(), true,num_pixles, frequencies.clone());

    let qe4 = SpatialSpectralEffect::initialize("lske1".to_string(), true, num_pixles, frequencies.clone());
    /*
    let qe5 = SpatialSpectralEffect::initialize("lske5".to_string(), true, num_pixles, frequencies.clone());
    let qe6 = SpatialSpectralEffect::initialize("lske6".to_string(), true, num_pixles, frequencies.clone());
    let qe7 = SpatialSpectralEffect::initialize("lske7".to_string(), true,num_pixles, frequencies.clone());
    let qe8 = SpatialSpectralEffect::initialize("lske8".to_string(), true, num_pixles, frequencies.clone());
    let qe9 = SpatialSpectralEffect::initialize("lske12".to_string(), true, num_pixles, frequencies.clone());
    let qe10 = SpatialSpectralEffect::initialize("lske22".to_string(), true, num_pixles, frequencies.clone());
    let qe11 = SpatialSpectralEffect::initialize("lske32".to_string(), true,num_pixles, frequencies.clone());
    let qe12 = SpatialSpectralEffect::initialize("lske42".to_string(), true, num_pixles, frequencies.clone());
    let qe13 = SpatialSpectralEffect::initialize("lske52".to_string(), true, num_pixles, frequencies.clone());
    let qe14 = SpatialSpectralEffect::initialize("lske62".to_string(), true, num_pixles, frequencies.clone());
    let qe15 = SpatialSpectralEffect::initialize("lske72".to_string(), true,num_pixles, frequencies.clone());
    let qe16 = SpatialSpectralEffect::initialize("lske82".to_string(), true, num_pixles, frequencies.clone());

     */


    data_cube_management::wide_compare_quad_unrolled_combine_data_cubes(qe,qe2,qe3,qe4, data_cube_management::ElementWiseCombinationType::ComponentWiseMultiplicative, result);

  //  data_cube_management::oct_unrolled_combine_data_cubes(qe, qe2, qe3, qe4,qe5, qe6, qe7, qe8, data_cube_management::ElementWiseCombinationType::ComponentWiseAddition, result);


    /*
    let fake_light =Array3::random(
        (4000,4000, 10),
        Uniform::new(0., 1.)).to_owned();

     */
    //println!("FAke light is {:?}",fake_light);
    //qe.apply(fake_light)
   // QuantumEfficiency::hallucinate(4000,vec![1000,1500,2000,2500]);
  //  println!("serializing!");

     */



//integrate bs checking/data verification and link up gneration/storage


