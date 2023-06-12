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

use crate::console::console_menu::{print_first_option,
                                   print_list,
                                   print_menu,
                                   print_second_option,
                                   print_third_option,
                                   print_total_sum,
                                   print_license};
use crate::console::console_menu::print_percentage;
use crate::options::menu_options::{run_first_option, run_second_option, run_third_option};
use crate::product::Product;

///Function for launching main menu in terminal

pub fn launch_main_console_menu(mut products_list: Vec<Product>) {
	loop {
		print_menu();

		let choice = make_choice();

		let choice: u8 = match choice.trim().parse() {
			Ok(0) => break,
			Ok(1) => 1,
			Ok(2) => 2,
			Ok(3) => 3,
			Ok(4) => 4,
			Err(_) => continue,
			Ok(i32::MIN..=-1_i32) | Ok(3_i32..=i32::MAX) => continue,
		};

		if choice == 1 {
			print_first_option();
			run_first_option(&mut products_list);
		} else if choice == 2 {
			print_second_option();
			run_second_option(&products_list);
		} else if choice == 3 {
			let mut total_sum: f64 = 0.0;

			print_third_option();
			run_third_option(&products_list, &mut total_sum);

			print_total_sum(&total_sum);

			percentage(&products_list, total_sum);
		} else if choice == 4 {
			print_license();
		}
	}
}

///Function for choosing an option in menus

pub fn make_choice() -> String {
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