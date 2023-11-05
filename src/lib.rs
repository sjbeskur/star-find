#![allow(dead_code)]
mod cli;
use log::{debug}; use std::collections::HashMap;
//info, trace, warn, error, 
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::ops::Deref;
use serde::{ Serialize, Deserialize };

use opencv::{
    prelude::*,
    core::Scalar,
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
    //let mut thresh = Mat::default();
    //imgproc::threshold(&src, &mut thresh, 0.0, 65535.0, imgproc::THRESH_OTSU)?;
    
    // Perform the operation
    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    let output = imgproc::connected_components_with_stats(&src, &mut labels, &mut stats, &mut centroids, connectivity, core::CV_32S);// core::CV_16U);//core::CV_32S);


    let label_map = map_labels_to_pixel_magnitudes(&labels, &src)?;

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
            // todo:
            // magnitude:  label_map.get(  labels.at_2d(y, x)) // something like this
        };
        blobs.push(stat);
    }

    let count = output? as usize - 1;
    assert!(blobs.len() == count );

    debug!("Total blobs: {:#?}", count );

    Ok(blobs)

}


/// Raster scan the label image (Mat) pixel by pixel
/// to find non zero labels.  If a label is found get the corresponding
/// pixel from ths src image and accumulate these in the label_map.
/// NOTE:  this is currently not totally correct because as I'm not handling
///        multi-byte pixels.
fn map_labels_to_pixel_magnitudes(labels: &Mat, src: &Mat) -> Result<HashMap<i32,f32>, Box<dyn Error>> {
    let mut label_map: HashMap<i32,f32> = HashMap::new();
    for y in 0..(labels.rows()) {
        for x in 0..(labels.cols()){
            let label = *labels.at_2d::<i32>(y,x)?;
            if label != 0{
                let value = src.at_2d::<u8>(y,x)?;
                let vv = f32::from(*value);
                label_map.entry(label).and_modify(|v| { *v += vv} ).or_insert(vv);
            }
        }
    }
    Ok(label_map)
}




pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}


