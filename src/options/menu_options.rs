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

use crate::console::console_menu::{print_bot_border,
                                   print_inp_message,
                                   print_mid_border,
                                   print_top_border};
use crate::options;
use crate::product;

///Function for running the first option

pub fn run_first_option(products: &mut Vec<product::Product>) {
	loop {
		print_inp_message();

		let amount = options::general_options::make_choise();
		let amount: u32 = match amount.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		for i in 0..amount {
			print_top_border(&i);
			let name = options::general_options::read_product();
			print_mid_border(&i);
			let price = options::general_options::read_product();
			print_bot_border();

			let prod = product::Product::new(name, price);

			products.push(prod);
		}

		break;
	}
}

///Function for running the second option

pub fn run_second_option(products: &Vec<product::Product>) {
	options::general_options::display_list(products);
}

///Function for running the second menu option

pub fn run_third_option(products: &Vec<product::Product>, sum: &mut f64) {
	options::general_options::count_total_sum(products, sum);
}