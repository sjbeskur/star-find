use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author="Sam Beskur <sam.beskur@gmail.com>", version, about="Blob detector for stars", long_about = "A very basic demonstration of OpenCV CCL w/Stats funtionality.")]
pub struct Config{
    pub filename: String,

    #[arg(short = 'c', long = "connectivity", default_value_t = 4 )]
    pub connectivity: u8,  // TODO: figure out how to only allow 4 or 8?
}
