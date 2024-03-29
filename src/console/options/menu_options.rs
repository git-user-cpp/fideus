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

#[cfg(feature = "console")]
use crate::console::console_menu::{print_bot_border,
                                   print_inp_message,
                                   print_mid_border,
                                   print_top_border};

#[cfg(feature = "console")]
use crate::console::options::general_options::{count_total_sum, display_list, make_choice, read_product};

#[cfg(feature = "console")]
use crate::product_structure::product::Product;

/// Function for running the first option

#[cfg(feature = "console")]
pub fn run_first_option(products: &mut Vec<Product>) {
	loop {
		print_inp_message();

		let amount = make_choice();
		let amount: u32 = match amount.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		for i in 0..amount {
			print_top_border(&i);
			let name = read_product();
			print_mid_border(&i);
			let price = read_product();
			print_bot_border();

			let prod = Product::new(name, price);

			match prod {
				Ok(prod) => products.push(prod),
				Err(_) => eprintln!("[Error] an empty element"),
			};
		}

		break;
	}
}

/// Function for running the second option

#[cfg(feature = "console")]
pub fn run_second_option(products: &Vec<Product>) {
	display_list(products);
}

/// Function for running the second menu option

#[cfg(feature = "console")]
pub fn run_third_option(products: &Vec<Product>, sum: &mut f64) {
	count_total_sum(products, sum);
}