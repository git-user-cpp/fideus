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

use crate::console::console_menu::{print_bot_border,
                                   print_mid_border,
                                   print_top_border,
                                   print_inp_message};
use crate::product;
use crate::options;

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