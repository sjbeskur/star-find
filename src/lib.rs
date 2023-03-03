#![allow(dead_code)]
mod cli;
use log::{debug}; //info, trace, warn, error, 
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use serde::{ Serialize, Deserialize };

use opencv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    types::VectorOfu8,
    //highgui,
};

pub use cli::{Config};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlobStats{
    pub point: Point,
    pub width: i32,
    pub height: i32,
    pub area: i32,
}


/// This is vestigial at the moment but I want to move to this as an alternative
/// to imread(...) as it offers a bit more fine grain control and I can clean up the
/// error handling
pub fn run(config: Config) -> AppResult<Vec<BlobStats>> {
    let mut file = open(&config.filename)?;

    let mut buffer : Vec<u8> = Vec::new();
    let _read_count = file.read_to_end(&mut buffer)?;
    let result = imgcodecs::imdecode(&VectorOfu8::from_iter(buffer), imgcodecs::IMREAD_GRAYSCALE); // IMREAD_GRAYSCALE);
    //let src = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?;// )?;
    
    find_stars(result?,config.connectivity as i32)
}

/// 
/// 
/// 
/// 
fn find_stars(src: Mat, connectivity: i32) -> AppResult<Vec<BlobStats>>{

    // Threshold it so it becomes binary
    let mut thresh = Mat::default();
    imgproc::threshold(&src, &mut thresh, 0.0, 255.0, imgproc::THRESH_BINARY | imgproc::THRESH_OTSU)?;
    
    // Perform the operation
    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    let output = imgproc::connected_components_with_stats(&thresh, &mut labels, &mut stats, &mut centroids, connectivity, core::CV_16U);//core::CV_32S);

    let mut blobs = Vec::new();
    for r in 1..stats.rows(){    // 0 is the background
        let stat = BlobStats{            
            point: Point{
                x: *centroids.at_2d::<f64>(r, 0)?,
                y: *centroids.at_2d::<f64>(r, 1)?,
            },
            width:  *stats.at_2d::<i32>(r, imgproc::CC_STAT_WIDTH)? ,
            height: *stats.at_2d::<i32>(r, imgproc::CC_STAT_HEIGHT)? ,
            area:   *stats.at_2d::<i32>(r, imgproc::CC_STAT_AREA)? ,
        };
        blobs.push(stat);
    }

    let count = output? as usize - 1;
    assert!(blobs.len() == count );

    debug!("Total blobs: {:#?}", count );

    Ok(blobs)

}




pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}


