mod edit_distance;
mod ascii;
use ascii::ascii_to_bytes;
use edit_distance::get_edit_distance;

fn main() {
    let distance = get_edit_distance(
        &ascii_to_bytes("this is a test"), &ascii_to_bytes("wokka wokka!!!")
    );
    println!("{}", distance);
}
