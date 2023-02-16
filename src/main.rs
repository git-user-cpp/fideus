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

use colored::Colorize;
mod menu_main;
mod menu_first;
mod menu_second;
mod menu_third;
mod options;
mod product;

fn main() {
    
    //vector for holding products data
    let mut products_list: Vec<product::Product> = Vec::new();

    //loop for main menu
    loop {
        //main menu call
        menu_main::show_menu();

        //choosing an option
        let choise = options::make_choise();

        //checks if input is correct
        let choise: u8 = match choise.trim().parse() {
            Ok(0) => break,
            Ok(1) => 1,
            Ok(2) => 2,
            Ok(3) => 3,
            Err(_) => continue,
            Ok(i32::MIN..=-1_i32) | Ok(3_i32..=i32::MAX) => continue,
        };

        //checks which option to show
        if choise == 1 {
            //running the first option
            menu_first::show_first_option();
            menu_first::run_first_option(&mut products_list);
        }else if choise == 2 {
            //variable for total sum
            let mut total_sum: f64 = 0.0;

            //running the second option
            menu_second::show_second_option();
            menu_second::run_second_option(&products_list, &mut total_sum);
        }else if choise == 3 {
            //running the third option
            menu_third::show_third_option();
        }
    }
    println!(" {}\n{} {} {}\n {}", "------------------------".red(), "|".red(), "The program is stopped".green(), "|".red(), "------------------------".red());
}
