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

use finance_manager::options;

/*
    TO AVOID MISTAKES YOU SHOULD
    RUN TESTS VIA cargo test -- --test-threads 1 --nocapture
*/

//Test for checking make_choise function
#[test]
fn test_make_choise() {
    println!("Write digit '1' to pass the test:");
    assert_eq!(options::make_choise(), "1\n");

    println!("Write digit '2' to pass the test:");
    assert_eq!(options::make_choise(), "2\n");

    println!("Write digit '3' to pass the test:");
    assert_eq!(options::make_choise(), "3\n");

    println!("Write digit '0' to pass the test:");
    assert_eq!(options::make_choise(), "0\n");
}

//Test for checking read_product function
#[test]
fn test_read_product() {
    let mut string: String = String::new();

    println!("Write 'product' to pass the test:");
    assert_eq!(options::read_product(&mut string), "product\n");
}
