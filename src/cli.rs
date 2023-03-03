use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author="Sam Beskur <sam.beskur@gmail.com>", version, about="Blob detector for stars", long_about = "A very basic demonstration of OpenCV CCL w/Stats funtionality.")]
#[command(propagate_version = true)]
pub struct Config{
    pub filename: String,

    #[arg(short = 'c', long = "connectivity", default_value_t = 4 )]
    pub connectivity: u8,  // TODO: figure out how to only allow 4 or 8?
}


/* 
use clap::{Arg, Command};

#[derive(Debug)]
pub struct Config{
    pub filename: String,
    pub connectivity: u8,
}

pub fn get_args() -> super::AppResult<Config> {
    let matches = Command::new("starrynight")
        .version("0.1.2")
        .author("Sam Beskur <sam.beskur@gmail.com>")
        .about("A very basic demonstration of OpenCV CCL w/Stats funtionality.")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s) to read")
                .required(true)
                .num_args(1)
        )
        .arg(
            Arg::new("connectivity")
                //.value_name("CONNECTS")
                .long("connectivity")
                .short('c')
                .help("8 or 4 for 8-way or 4-way connectivity respectively")
                .required(false)
                .default_value("8")
                .value_parser(clap::value_parser!(u8))
                .num_args(1)
        )        

        .get_matches();

    Ok(Config {
        filename: matches.get_one::<String>("file").unwrap().to_string(),
        connectivity: *matches.get_one::<u8>("connectivity").unwrap(),
    })
}

*/
