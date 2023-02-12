use std::io;

//function for choosing an option
pub fn make_choise() -> String {
    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read line");

    choise
}
