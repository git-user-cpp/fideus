use std::io;

//function for choosing an option
pub fn make_choise() -> String {
    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read line");

    choise
}

//function for input products
pub fn read_product(tmp: &mut String) -> String {
    io::stdin()
        .read_line(tmp)
        .expect("failed to read line");

    tmp.to_string()
}
