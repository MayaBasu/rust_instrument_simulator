use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use rand::{Rng, rng};
use csv::*;




use std::{error::Error, io, process};

pub fn example(line_size:usize, num_lines:usize,file_name:&str) -> Result<()> {
    let items_perLine = line_size;
    let num_lines = num_lines;

    let writer_result = Writer::from_path(file_name);
    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(err),
    };

    for line_num in 0..num_lines{
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
