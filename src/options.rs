/*
MIT License
Copyright (c) 2023 m!haly4
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
use colored::Colorize;
use crate::product;

//function for choosing an option
pub fn make_choise() -> String {
    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read line");

    choise
}

//function for input products
pub fn read_product(tmp: &mut String) -> String {
    io::stdin()
        .read_line(tmp)
        .expect("failed to read line");

    tmp.to_string()
}

//function for showing the list of products
pub fn show_list(products: &Vec<product::Product>) {
    for element in products {
        println!(" {}\n product: {} price: {} {}","-----------------------".red(), element.name, element.price, "\n -----------------------".red());
    }
}

//function for counting total sum
pub fn count_total_sum(products: &Vec<product::Product>, sum: &mut f64) {
    //counting total sum
    for element in products {
        *sum += element.price;
    }

    println!(" {} {}\n {}", "Total sum =".yellow(), sum, "-----------------------".red());
}
