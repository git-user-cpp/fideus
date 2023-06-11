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

//Module for using this program via console/terminal mod console;
mod console;

mod product_structure;
mod options;

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
