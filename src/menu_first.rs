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

//function for showing the first menu option
pub fn show_first_option() {
   println!(" -----------------------\n|    Insert products    |\n -----------------------"); 
}

//function for the first option
pub fn run_first_option() {
    //vector for holding products data
    let mut products: Vec<String> = Vec::new();

    loop {
        println!("| Please input amount of your products:");

        //using options function to get user's input (yeah, I'm relatively lazy :D)
        let amount = crate::options::make_choise();

        let amount: u32 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        for i in 0..amount {
            println!("| Input {} product", i+1);

            let mut tmp = String::new();
            
            products.push(crate::options::read_product(&mut tmp));
        }

        break;
    }

    for element in products {
        println!("{}", element);
    }
}
