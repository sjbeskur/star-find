use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{Arg, Command};

use opencv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    highgui,
};

#[derive(Debug)]
pub struct Config{
    file: String,
    // todo: add more options (color resize?)
}

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn find_stars(config: Config) -> AppResult<()>{
    dbg!(&config);
    let filename = config.file;
    let src = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?;


    //let grayscale_image  = cv::imgproc::cvt_color(src, cv::imgproc::COLOR_HSV2BGR);
    // Threshold it so it becomes binary
    let mut thresh = Mat::default();

    let _t =
        imgproc::threshold(&src, &mut thresh, 0.0, 255.0, imgproc::THRESH_OTSU)?;//imgproc::THRESH_BINARY | 
    //& cv::imgproc::THRESH_OTSU)

    let connectivity = 8; 

    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();
    // Perform the operation
    let output = imgproc::connected_components_with_stats(&thresh, &mut labels, &mut stats, &mut centroids, connectivity, core::CV_16U);//core::CV_32S);
    //let output = imgproc::connected_components(&thresh, &mut labels, connectivity,  core::CV_16U);//core::CV_32S);

///    println!("labels: {:#?}\n", labels);
    println!("stats: {:#?}\n", stats.at_2d::<i32>(1, 1));
    println!("stats: rows: {} \t cols: {}\n", stats.rows(), stats.cols());

    for r in 1..stats.rows(){    
        println!("({} , {}) => {} - {} - {}", stats.at_2d::<i32>(r, 0)?
                                , stats.at_2d::<i32>(r, 1)?
                                , stats.at_2d::<i32>(r, 2)? 
                                , stats.at_2d::<i32>(r, 3)? 
                                , stats.at_2d::<i32>(r, 4)?);
    }
//    println!("centroids: {:#?}\n", centroids);
    println!("{:#?}", output);

    //highgui::imshow("", &labels)?;
    //highgui::wait_key(0)?;

    Ok(())

}

pub fn get_args() -> AppResult<Config> {
    let matches = Command::new("showimg")
        .version("0.1.2")
        .author("Sam Beskur <sam.beskur@gmail.com>")
        .about("A simple image viewer program.")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s) to read")
                .required(true)
                .num_args(1)
        ).get_matches();

    Ok(Config {
        file: matches
            .get_one::<String>("file").unwrap().to_owned()
    })
}



pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_blob_finder() {
        assert_eq!(true, false);
    }
}
