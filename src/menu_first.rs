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
