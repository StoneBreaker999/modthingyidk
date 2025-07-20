mod modulefile;
mod offsets;

use modulefile::ModFile;

fn main() {
    // TODO: make this better
    let bytes = include_bytes!("../vivalaluna-damla.mod");

    println!("ModFile::new() \n - {:#?}", ModFile::new());

    println!(
        "ModFile::new().from_file(bytes) \n - {:#?}",
        ModFile::new().from_file(bytes)
    );
    println!(
        "SONG NAME - {:#?}",
        ModFile::new().from_file(bytes).song_name()
    );
}
