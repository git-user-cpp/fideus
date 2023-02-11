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

fn main() {    
    loop {
        //mutable variable for main menu
        let mut choise = String::new();

        //main menu
        menu_main();

        //choosing an option
        io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read line");

        //checks if it's a number
        let choise: u8 = match choise.trim().parse() {
            Ok(0) => break,
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", choise);
    }
}

//main menu function
fn menu_main() {
    println!(" -----------------------\n|    Finance manager    |\n -----------------------\n|   Choose an option:   |\n|  [1] Insert products  |\n|  [2] Show percentage  |\n|  [0] Exit             |\n -----------------------\n");
}
