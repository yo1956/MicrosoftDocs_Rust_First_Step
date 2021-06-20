use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    
    // let mut num = 10;
    // num = 9;
    // println!("{}",num);
    
    // let shadow_num = 5;
    // let shadow_num = shadow_num + 5;
    // let shadow_num = shadow_num * 2;
    // println!("{}",shadow_num);
    // println!("1+2 = {} and 8-5 = {} and 15 * 3 = {}", 1i32 + 2, 8u32 - 5, 15*3);
    
    // let mut zeroes = vec![0; 5];
    // println!("zeroes: {:?}",zeroes);
    // zeroes.push(5);
    // println!("zeroes: {:?}",zeroes);
    
    // let mut reviews: HashMap<String, String> = HashMap::new();
    let mut reviews = HashMap::new();
    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great examples.".to_string());
    println!("{:?}",reviews);
}
