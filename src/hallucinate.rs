use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::time::Instant;
use rand::{Rng, rng};
use csv::*;
use csv::QuoteStyle;




use std::{error::Error, io, process};
use zerocopy::IntoBytes;

pub fn example(line_size:usize, num_lines:usize, file_name:&str) -> Result<()> {
    let items_perLine = line_size;
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
        let mut line: Vec<String> = Vec::with_capacity(items_perLine);
        for item in 0..items_perLine{
            line.push(rng().random_range(0.0..1.0).to_string())
        }

        writer.write_record(line);

    }


    // When writing records without Serde, the header record is written just
    // like any other record.

    Ok(())
}
pub fn byte_version(line_size:usize, num_lines:usize, file_name:&str) -> Result<()> {
    println!("Generating data for {file_name}");
    let items_perLine = line_size;
    let num_lines = num_lines;

    let f1 = File::create(file_name).unwrap();
    let mut buf = BufWriter::new(f1);


    for line_num in 0..num_lines{
        if line_num%1000 ==0{
            println!("Generating line {line_num}");
        }
        let mut line: Vec<String> = Vec::with_capacity(items_perLine);
        for item in 0..items_perLine{
            let value: f64 = rng().random_range(0.0..1.0);

            line.push(value.to_string());

            buf.write_all(&value.to_le_bytes()[..]).unwrap();

        }
     //   println!("Line is {:?}",line);
        buf.flush().unwrap();

        //writer.write_record(line);

    }


    // When writing records without Serde, the header record is written just
    // like any other record.

    Ok(())
}
