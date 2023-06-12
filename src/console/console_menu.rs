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

use colored::Colorize;

use crate::product;
use crate::product::Product;

///Function for printing main menu

pub fn print_menu() {
	println!(" {}\n{}             {}             {}\n {}\n{}            Choose an option:            {}\n{}           {}           {}\n{}             {}               {}\n{}  {}  {}\n{}           {}                      {}\n {}\n{}",
	         "-----------------------------------------".blue(),
	         "|".blue(),
	         "Finance manager".yellow(),
	         "|".blue(),
	         "-----------------------------------------".blue(),
	         "|".blue(),
	         "|".blue(),
	         "|".blue(),
	         "[1] Insert products".green(),
	         "|".blue(),
	         "|".blue(),
	         "[2] Show list".green(),
	         "|".blue(),
	         "|".blue(),
	         "[3] Show total sum & Show percentage ".green(),
	         "|".blue(),
	         "|".blue(),
	         "[0] Exit".red(),
	         "|".blue(),
	         "-----------------------------------------".blue(),
	         "> Your choise:".red()
	);
}

///Function for printing the first menu option

pub fn print_first_option() {
	println!(" {}\n{}    {}    {}\n {}",
	         "-----------------------------------------".blue(),
	         "|".blue(),
	         "         Insert products         ".green(),
	         "|".blue(),
	         "-----------------------------------------".blue()
	);
}

///Function for printing the second option

pub fn print_second_option() {
	println!(" {}\n{}    {}    {}\n {}",
	         "-----------------------------------------".blue(),
	         "|".blue(),
	         "         List of products        ".green(),
	         "|".blue(),
	         "-----------------------------------------".blue()
	);
}

///Function for printing the third menu option

pub fn print_third_option() {
	println!(" {}\n{} {} {}\n {}",
	         "-----------------------------------------".blue(),
	         "|".blue(),
	         "   Show total sum & Show percentage    ".green(),
	         "|".blue(),
	         "-----------------------------------------".blue()
	);
}

///Function for printing the list of products

pub fn print_list(products: &Vec<product::Product>) {
	for element in products {
		println!(" {}\n Product: {} Price: {} {}",
		         "-----------------------------------------".red(),
		         element.name,
		         element.price,
		         "\n -----------------------------------------".red()
		);
	}
}

///Function for printing a stop message

pub fn print_stop_message() {
	println!(" {}\n{}          {}         {}\n {}",
	         "-----------------------------------------".red(),
	         "|".red(),
	         "The program is stopped".green(),
	         "|".red(),
	         "-----------------------------------------".red()
	);
}

///Function for printing total sum

pub fn print_total_sum(total_sum: &f64) {
	println!(" {}\n {} {}\n {}",
	         "-----------------------------------------".red(),
	         "Total sum =".yellow(),
	         total_sum,
	         "-----------------------------------------".red()
	);
}

///Function for printing percentage

pub fn print_percentage(element: &Product, total_sum: &f64) {
	println!(" {}\n Product: {}\n Price: {}\n Percentage of the purchase (%) : {}%\n {}",
	         "-----------------------------------------".red(),
	         element.name,
	         element.price,
	         (element.price * 100.0) / total_sum,
	         "\n -----------------------------------------".red()
	);
}

///Function for printing input message

pub fn print_inp_message() {
	println!("{} {}", ">".red(), "Please input amount of your products:".green());
}

///Function for printing top border

pub fn print_top_border(i: &u32) {
	println!(" {}", "-----------------------------------------".red());
	println!("{} {} {} {}{}", "> Input".red(), i + 1, "product".red(), "name".yellow(), ":".red());
}

///Function for printing border

pub fn print_mid_border(i: &u32) {
	println!("{} {} {} {}{}", "> Input".red(), i + 1, "product".red(), "price".yellow(), ":".red());
}

///Function for printing bottom border

pub fn print_bot_border() {
	println!(" {}", "-----------------------------------------".red());
}