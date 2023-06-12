/*
Copyright 2023 Andrew Kushyk

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::io;

use crate::console::console_menu::print_list;
use crate::console::console_menu::print_percentage;
use crate::product::Product;

///Function for choosing an option in menus

pub fn make_choise() -> String {
	let mut choise = String::new();

	io::stdin().read_line(&mut choise).expect("Failed to read line");

	choise
}

///Function for input products

pub fn read_product() -> String {
	let mut tmp = String::new();

	io::stdin().read_line(&mut tmp).expect("failed to read line");

	tmp.to_string()
}

///Function for showing the list of products

pub fn display_list(products: &Vec<Product>) {
	print_list(products);
}

///Function for counting total sum

pub fn count_total_sum(products: &Vec<Product>, sum: &mut f64) -> f64 {
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

	// use super::*;

	#[test]
	fn test_make_choise() {
		//TODO: reimplement the test to make it automated

		// println!("Enter \"1\" to pass the test");
		// assert_eq!("1\n", make_choise());
		// println!("Enter \"2\" to pass the test");
		// assert_eq!("2\n", make_choise());
		// println!("Enter \"3\" to pass the test");
		// assert_eq!("3\n", make_choise());
		// println!("Enter \"0\" to pass the test");
		// assert_eq!("0\n", make_choise());
	}
}