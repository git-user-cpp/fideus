/*
MIT License
Copyright (c) 2023 Andrew Kushyk
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::io;

use crate::product;
use crate::product::Product;

use crate::console::console_menu::print_list;
use crate::console::console_menu::print_percentage;

///Function for choosing an option in menus
pub fn make_choise() -> String {
	let mut choise = String::new();

	io::stdin()
		.read_line(&mut choise)
		.expect("Failed to read line");

	choise
}

///Function for input products
pub fn read_product() -> String {
	let mut tmp = String::new();

	io::stdin()
		.read_line(&mut tmp)
		.expect("failed to read line");

	tmp.to_string()
}

///Function for showing the list of products
pub fn display_list(products: &Vec<product::Product>) {
	print_list(products);
}

///Function for counting total sum
pub fn count_total_sum(products: &Vec<product::Product>, sum: &mut f64) -> f64 {
	for element in products {
		*sum += element.price;
	}

	*sum
}

///Function for counting percentage
pub fn percentage(products_list: &Vec<Product>, total_sum: f64) {
	for element in products_list {
		print_percentage(element, &total_sum);
	}
}

//TESTS

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_make_choise() {
		println!("Enter \"1\" to pass the test");
		assert_eq!("1\n", make_choise());
		println!("Enter \"2\" to pass the test");
		assert_eq!("2\n", make_choise());
		println!("Enter \"3\" to pass the test");
		assert_eq!("3\n", make_choise());
		println!("Enter \"0\" to pass the test");
		assert_eq!("0\n", make_choise());
	}
}