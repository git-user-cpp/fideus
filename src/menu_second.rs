use crate::options;
use crate::product;

//function for running the second option
pub fn run_second_option(products: &Vec<product::Product>) {
    options::show_list(products);
}
