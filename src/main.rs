use modthingyidk::AmigaMod;
fn main() {
    // TODO: make this pretty
    let bytes = include_bytes!("../vivalaluna-damla.mod");

    println!("{:#?}", AmigaMod::new().from_bytes(bytes));
}
