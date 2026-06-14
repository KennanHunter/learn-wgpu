use std::{ascii::AsciiExt, fs, io::Write, ops::Add, path::PathBuf};

const SINGLE_DIMENSION_SIZE: usize = 16;

fn main() {
    env_logger::init();

    let mut args = std::env::args();

    args.next().expect("Should have CWD");

    let a = match args.next() {
        Some(a) => {
            log::info!("Writing to : {}", &a);
            a
        }
        None => {
            log::error!("Should include file name");

            return;
        }
    };

    let pth = PathBuf::from(a);

    // if pth.exists() {
    //     fs::remove_file(&pth).expect("Should be able to delete file at pth");
    // }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&pth)
        .expect("msg");

    // create a 16 by 16 f32 array
    let mut arr = [[0.0f32; SINGLE_DIMENSION_SIZE]; SINGLE_DIMENSION_SIZE];

    let radius = 4.0;

    for (row_index, row) in arr.iter_mut().enumerate() {
        for (column_index, row_element) in row.iter_mut().enumerate() {
            let distance_from_center = Add::add(
                (row_index as f64 - (SINGLE_DIMENSION_SIZE as f64) / 2.0).powf(2.0),
                (column_index as f64 - (SINGLE_DIMENSION_SIZE as f64) / 2.0).powf(2.0),
            )
            .sqrt();

            *row_element = (radius - distance_from_center) as f32;
        }
    }

    println!("Array of circle with radius {}:", radius);
    for row in arr {
        for el in row {
            print!("{:>6.2} ", el);
        }
        println!();
    }

    for element in arr.iter().flatten() {
        file.write(&f32::to_le_bytes(element.clone()))
            .expect("Should be able to write");
    }

    file.flush().expect("Should be able to flush file");
}
