
fn main() {
    if let Err(error) = starrynight::get_args().and_then(starrynight::find_stars) {
        eprint!("{}",error);
        std::process::exit(1)
    }
}
