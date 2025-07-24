pub mod receipt;
use receipt::content::ReceiptContent;
use receipt::product::create_products;
use receipt::product::StoreProduct;
use receipt::content::complete_purchase;

fn main() {
    let store_products: Vec<StoreProduct> = create_products();
    let mut rc: ReceiptContent = ReceiptContent {
        products: vec![],
        store: "Imaginary Town General Store".to_string(),
    };

    use std::io;
    loop {
        println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading the input");
        let input = input.trim();

        if input == "1" {
            println!("Which product would you like to add?");
            let mut i = 1;
            for product in &store_products {
                println!("{}) {} | Price - {}", i, product.name, product.price);
                i = i + 1;
            }

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error while reading the input");
            let input = input.trim();

            let index: usize = match input.trim().parse() {
                Ok(luku) => luku,
                Err(_) => {
                    println!("Parsing error. Was the input a number?");
                    continue;
                }
            };
            rc.products.push(store_products[index - 1].clone());

        } else if input == "2" {
            rc.products.pop();

        } else if input == "3" {
            break;
        } else {
            println!("Invalid input");
        }
    }
    complete_purchase(&mut rc);
    println!("Thank you for your purchase!");
}
