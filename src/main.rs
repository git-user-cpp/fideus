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

//Module for using this program via console/terminal;

mod console;

mod product_structure;
mod options;

use crate::options::general_options::launch_main_console_menu;
use crate::console::console_menu::print_stop_message;
use crate::product_structure::product::Product;

fn main() {
	// let mut products_list: Vec<Product> = Vec::new();
	//
	// launch_main_console_menu(products_list);
	//
	// print_stop_message();
}
