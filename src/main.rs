
fn main() {
    if let Err(error) = star_find::get_args().and_then(star_find::find_stars) {
        eprint!("{}",error);
        std::process::exit(1)
    }
}
