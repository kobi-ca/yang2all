use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let path = Path::new("ultraconfig-interfaces.yang");
    let display = path.display();
    println!("{}", display);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
