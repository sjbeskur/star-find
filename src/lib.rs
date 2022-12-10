#![allow(dead_code)]
mod cli;

use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader};

use opencv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    types::VectorOfu8,
    //highgui,
};

pub use cli::{Config, get_args};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Stats{
    point: opencv::core::Point,
    width: i32,
    height: i32,
    area: i32,
}


#[derive(Debug)]
struct Centroids{
    x: f64,
    y: f64,
}

/// This is vestigial at the moment but I want to move to this as an alternative
/// to imread(...) as it offers a bit more fine grain control and I can clean up the
/// error handling
pub fn run(config: Config) -> AppResult<()> {
    match open(&config.filename) {
        Err(err) => eprintln!("{}", err),

        Ok(mut file) =>{
            let mut buffer : Vec<u8> = Vec::new();
            let _read_count = file.read_to_end(&mut buffer)?;
            let result = imgcodecs::imdecode(&VectorOfu8::from_iter(buffer), imgcodecs::IMREAD_GRAYSCALE); // IMREAD_GRAYSCALE);
            //let src = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?;// )?;
            find_stars(result?,config.connectivity as i32)?
        },        
    };
    Ok(())    
}

pub fn find_stars(src: Mat, connectivity: i32) -> AppResult<()>{

    // Threshold it so it becomes binary
    let mut thresh = Mat::default();
    imgproc::threshold(&src, &mut thresh, 0.0, 255.0, imgproc::THRESH_BINARY | imgproc::THRESH_OTSU)?;
    
    // Perform the operation
    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    let output = imgproc::connected_components_with_stats(&thresh, &mut labels, &mut stats, &mut centroids, connectivity, core::CV_16U);//core::CV_32S);

    println!("stats: {:#?}\n", stats);

    for r in 1..stats.rows(){    // 0 is the background
        let p = opencv::core::Point::new(
            *stats.at_2d::<i32>(r, imgproc::CC_STAT_LEFT)?,
            *stats.at_2d::<i32>(r, imgproc::CC_STAT_TOP)? ,
        );

        let stat = Stats{            
            point: p,
            width: *stats.at_2d::<i32>(r, imgproc::CC_STAT_WIDTH)? ,
            height: *stats.at_2d::<i32>(r, imgproc::CC_STAT_HEIGHT)? ,
            area: *stats.at_2d::<i32>(r, imgproc::CC_STAT_AREA)? ,
        };
        println!("{:?}",stat);
    }


    println!("centroids: {:#?}\n", centroids);
    for r in 1..centroids.rows(){    // 0 is the background
        let cent = Centroids{
            x: *centroids.at_2d::<f64>(r, 0)?,
            y: *centroids.at_2d::<f64>(r, 1)?,
        };
        println!("{:?}",cent);
    }

    println!("\n total stars: {:#?}", output? - 1);

    Ok(())

}




pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}


