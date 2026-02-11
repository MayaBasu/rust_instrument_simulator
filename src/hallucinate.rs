use std::fs::{File};
use std::io::{BufWriter, Write};
use rand::{Rng, rng};
use csv::*;



pub fn example(line_size:usize, num_lines:usize, file_name:&str) -> Result<()> {
    let items_per_line = line_size;
    let num_lines = num_lines;

    let writer_result = Writer::from_path(file_name);

    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(err),
    };


    for line_num in 0..num_lines{
        if line_num%1000 ==0{
            println!("Generating line {line_num}");
        }
        let mut line: Vec<String> = Vec::with_capacity(items_per_line);
        for _item in 0..items_per_line {
            line.push(rng().random_range(0.0..1.0).to_string())
        }

        let _ = writer.write_record(line);

    }


    // When writing records without Serde, the header record is written just
    // like any other record.

    Ok(())
}
pub fn hallucinate_bytes(spectral_resolution:usize, spatial_resolution:usize, file_name:&str,generate_human_readable_as_well:bool) -> Result<()> {
    println!("Generating data for {file_name}");
    let human_readable_file_name = "human_readable".to_owned() + file_name;
    let human_writer_result = Writer::from_path(&human_readable_file_name);
    let mut human_writer = match human_writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(err),
    };

    if generate_human_readable_as_well{
        println!("Storing human readable version to {human_readable_file_name}");
    };


    let items_perLine = spectral_resolution;
    let num_lines = spatial_resolution*spatial_resolution; //assume the data is square shaped
    println!("{items_perLine} items per line and {num_lines} lines");

    let f1 = File::create(file_name).unwrap();
    let mut buf = BufWriter::new(f1);



    for line_num in 0..num_lines{
        if line_num%1000 ==0{
            println!("Generating line {line_num}");
        }
        let mut line: Vec<String> = Vec::with_capacity(items_perLine);
        for _item in 0..items_perLine{
            let value: f64 = rng().random_range(0.0..1.0);

            line.push(value.to_string());
            buf.write_all(&value.to_le_bytes()[..]).unwrap();
        }
        let _ = human_writer.write_record(line);
        buf.flush().unwrap();

    }

    Ok(())
}


pub(crate) fn name_gen(linesize:usize, num_lines:usize, delinator:&str) -> String {
    (num_lines.to_string() + "_lines_" + linesize.to_string().as_str()+ "_columns_" + delinator).to_string()
}

//TODO convert from non human readable to human readable files