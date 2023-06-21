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

/// Structure of the product
pub struct Product {
    name: String,
    price: f64,
}

/// Implementation block for product
impl Product {
    pub fn new(name: String, price: String) -> Result<Product, &'static str> {
        let float_price: Result<f64, _> = price.trim().parse();

        match float_price {
            Ok(parsed_price) => Ok(Product {
                name,
                price: parsed_price,
            }),
            Err(_) => Err("Invalid price format. Please enter a valid price."),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }
}
