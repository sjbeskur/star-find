
fn main() {
    let args = starrynight::get_args();

    match starrynight::run(args.unwrap()){
        Ok(blobs) => {
            let json = serde_json::to_string_pretty(&blobs).unwrap();
            println!("{}", json);    //?;
        }
        Err(err) => {
            eprintln!("{}",err);
        }
    }

    // if let Err(error) = starrynight::get_args().and_then(starrynight::run) {
    //     std::process::exit(1)
    // }
}
