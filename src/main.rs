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

use colored::Colorize;

///Modules
mod product;
mod menu_main;
mod options;
mod menu_first;
mod menu_second;
mod menu_third;

fn main() {
    let mut products_list: Vec<product::Product> = Vec::new();

    loop {
        menu_main::show_menu();

        let choise = options::make_choise();

        let choise: u8 = match choise.trim().parse() {
            Ok(0) => break,
            Ok(1) => 1,
            Ok(2) => 2,
            Ok(3) => 3,
            Err(_) => continue,
            Ok(i32::MIN..=-1_i32) | Ok(3_i32..=i32::MAX) => continue,
        };

        if choise == 1 {
            menu_first::show_first_option();
            menu_first::run_first_option(&mut products_list);
        }else if choise == 2{
            menu_second::show_second_option();
            menu_second::run_second_option(&products_list);
        }else if choise == 3 {
            let mut total_sum: f64 = 0.0;

            menu_third::show_third_option();
            menu_third::run_third_option(&products_list, &mut total_sum);

            println!(" {}\n {} {}\n {}",
                     "-----------------------------------------".red(),
                     "Total sum =".yellow(),
                     total_sum,
                     "-----------------------------------------".red());

            for element in &products_list {
                println!(" {}\n Product: {}\n Price: {}\n Percentage of the purchase (%) : {}%\n {}",
                "-----------------------------------------".red(),
                element.name,
                element.price,
                (element.price * 100.0) / total_sum,
                "\n -----------------------------------------".red());
            }
        }
    }

    println!(" {}\n{}          {}         {}\n {}",
             "-----------------------------------------".red(),
             "|".red(),
             "The program is stopped".green(),
             "|".red(),
             "-----------------------------------------".red());
}
