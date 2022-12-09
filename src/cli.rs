use clap::{Arg, Command};


#[derive(Debug)]
pub struct Config{
    pub file: String,
    pub connectivity: u8,
}

pub fn get_args() -> super::AppResult<Config> {
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
        )
        .arg(
            Arg::new("connectivity")
                //.value_name("CONNECTS")
                .long("connectivity")
                .short('c')
                .help("8 or 4 for 8-way or 4-way connectivity respectively")
                .required(false)
                .default_value("4")
                .value_parser(clap::value_parser!(u8))
                .num_args(1)
        )        
        .get_matches();

    Ok(Config {
        file: matches.get_one::<String>("file").unwrap().to_string(),
        connectivity: *matches.get_one::<u8>("connectivity").unwrap(),
    })
}
