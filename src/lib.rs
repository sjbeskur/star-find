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
    //highgui,
};

pub use cli::{Config, get_args};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Stats{
    u: i32,
    v: i32,
    width: i32,
    height: i32,
    area: i32,
}


#[derive(Debug)]
struct Centroids{
    x: f64,
    y: f64,
}


pub fn find_stars(config: Config) -> AppResult<()>{
    dbg!(&config);
    let filename = config.file;
    let connectivity = config.connectivity as i32; 

    let src = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?;// )?;
    //let grayscale_image  = cv::imgproc::cvt_color(src, cv::imgproc::COLOR_HSV2BGR);

    /* 
    let mut gray_image = Mat::default();
    if src.channels() == 3{
         imgproc::cvt_color(&src, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)?;
    }else{
        gray_image = src.clone();
    }
    */

    let mut thresh = Mat::default();

    // Threshold it so it becomes binary
    let _t =
        imgproc::threshold(&src, &mut thresh, 0.0, 255.0, imgproc::THRESH_BINARY | imgproc::THRESH_OTSU)?;

    
    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    // Perform the operation
    let output = imgproc::connected_components_with_stats(&thresh, &mut labels, &mut stats, &mut centroids, connectivity, core::CV_16U);//core::CV_32S);

    println!("stats: {:#?}\n", stats);

    for r in 1..stats.rows(){    // 0 is the background
        let stat = Stats{
            u: *stats.at_2d::<i32>(r, imgproc::CC_STAT_LEFT)?,
            v: *stats.at_2d::<i32>(r, imgproc::CC_STAT_TOP)? ,
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

    //highgui::imshow("", &labels)?;
    //highgui::wait_key(0)?;

    Ok(())

}




pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename).expect("File not found"))))
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_blob_finder() {
        assert_eq!(true, false);
    }
}
