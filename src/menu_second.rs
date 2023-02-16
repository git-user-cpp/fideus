use crate::options;
use crate::product;
use colored::Colorize;

//function for showing the second option
pub fn show_second_option() {
    println!(" {}\n{}    {}    {}\n {}", "-----------------------------------------".blue(), "|".blue(), "         List of products        ".green(), "|".blue(), "-----------------------------------------".blue());
}

//function for running the second option
pub fn run_second_option(products: &Vec<product::Product>) {
    options::show_list(products);
}
