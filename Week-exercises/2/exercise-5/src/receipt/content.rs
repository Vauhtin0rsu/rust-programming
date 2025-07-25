use super::product::StoreProduct;
use std::fs::File;
use std::io::{self, Write};

const PRODUCT_1_NAME: &str = "Zbox 720";
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
const PRODUCT_3_NAME: &str = "Potato";

pub struct ReceiptContent{
    pub products: Vec<StoreProduct>,
    pub store: String
}

pub fn complete_purchase( rc: &mut ReceiptContent ) -> io::Result<()> {
    let mut prod_1_amount: i32 = 0;
    let mut prod_2_amount: i32 = 0;
    let mut prod_3_amount: i32 = 0;

    let mut prod_1_sum: i32 = 0;
    let mut prod_2_sum: i32 = 0;
    let mut prod_3_sum: i32 = 0;

    for product in &rc.products {
        if product.name == PRODUCT_1_NAME {
            prod_1_amount = prod_1_amount + 1;
            prod_1_sum = prod_1_sum + product.price;

        } else if product.name == PRODUCT_2_NAME {
            prod_2_amount = prod_2_amount + 1;
            prod_2_sum = prod_2_sum + product.price;

        } else if product.name == PRODUCT_3_NAME {
            prod_3_amount = prod_3_amount + 1;
            prod_3_sum = prod_3_sum + product.price;
        }
    }

    let receipt_line: String = "------------------------------".to_string();

    let mut receipt = File::create("receipt.txt").expect("Failed with creating the file");
    writeln!(receipt, "{}", rc.store);
    writeln!(receipt, "{}", receipt_line);

    if prod_1_amount > 0 {
        writeln!(receipt, "{} ({}) - {}€", PRODUCT_1_NAME, prod_1_amount, prod_1_sum);
    }
    if prod_2_amount > 0 {
        writeln!(receipt, "{} ({}) - {}€", PRODUCT_2_NAME, prod_2_amount, prod_2_sum);
    }
    if prod_3_amount > 0 {
        writeln!(receipt, "{} ({}) - {}€", PRODUCT_3_NAME, prod_3_amount, prod_3_sum);
    }

    let total_sum: i32 = prod_1_sum + prod_2_sum + prod_3_sum;
    writeln!(receipt, "{}", receipt_line);
    writeln!(receipt, "Final price: {}€", total_sum);
    writeln!(receipt, "{}", receipt_line);
    
    Ok(())
}