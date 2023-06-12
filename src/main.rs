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

//Module for using this program via console/terminal mod console; mod console;
mod console;

mod product_structure;
mod options;

use crate::console::console_menu::{print_first_option,
                                   print_menu,
                                   print_second_option,
                                   print_stop_message,
                                   print_third_option,
                                   print_total_sum};
use crate::options::general_options::{make_choise,
                                      percentage};
use crate::options::menu_options::{run_first_option,
                                   run_second_option,
                                   run_third_option};
use crate::product_structure::product;

fn main() {
	let mut products_list: Vec<product::Product> = Vec::new();

	loop {
		print_menu();

		let choice = make_choise();

		let choice: u8 = match choice.trim().parse() {
			Ok(0) => break,
			Ok(1) => 1,
			Ok(2) => 2,
			Ok(3) => 3,
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
		}
	}

	print_stop_message();
}
