
fn main() {
    if let Err(error) = starrynight::get_args().and_then(starrynight::run) {
        eprint!("{}",error);
        std::process::exit(1)
    }
}
