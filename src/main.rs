//
// #######  #######  ######   #######  #     # 
// #        #     #  #     #  #         #   #  
// #        #     #  #     #  #          # #   
// #####    #     #  ######   #####       #    
// #        #     #  #   #    #          # #   
// #        #     #  #    #   #         #   #  
// #        #######  #     #  #######  #     # 
//                                           
// FOREX - Quick Currency Conversions
// Copyright (c) Afaan Bilal ( https://afaanbilal.github.io )
//

extern crate reqwest;
extern crate serde_json;

use std::io;
use std::io::Read;
use std::io::Write;

fn get_conversion_rate(from: &String, to: &String) -> f64 {
    let url: String = format!("https://api.fixer.io/latest?base={}&symbols={},{}", from, to, from).into();
    let mut res = reqwest::get(url.as_str()).expect("Error doing GET");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Error reading to string");
    let result: serde_json::Value = serde_json::from_str(body.as_str()).expect("Error: Something went wrong.");
    if result["error"].is_null() && !result["rates"][&to].is_null() {
        result["rates"][&to].as_f64().unwrap()
    } else {
        println!("Error: Something went wrong. Are the currency codes correct?");
        std::process::exit(-1);
    }
}

fn main() {
    println!(r#"
 #######  #######  ######   #######  #     # 
 #        #     #  #     #  #         #   #  
 #        #     #  #     #  #          # #   
 #####    #     #  ######   #####       #    
 #        #     #  #   #    #          # #   
 #        #     #  #    #   #         #   #  
 #        #######  #     #  #######  #     #"#);
    io::stdout().flush().expect("");
    println!("\n\tQuick Currency Conversions");
    println!("Copyright (c) Afaan Bilal ( https://afaanbilal.github.io )\n");
    
    let mut cc_from = String::new();
    let mut cc_to   = String::new();
    let mut amt     = String::new();

    print!("Enter FROM currency code: ");
    io::stdout().flush().expect("");
    io::stdin() .read_line(&mut cc_from).expect("Input error");

    print!("Enter TO   currency code: ");
    io::stdout().flush().expect("");
    io::stdin() .read_line(&mut cc_to  ).expect("Input error");
    
    print!("Enter amount: ");
    io::stdout().flush().expect("");
    io::stdin() .read_line(&mut amt    ).expect("Input error");

    let amt: f64 = amt.trim().parse().expect("Parse error");
    println!("Please wait...\n");

    cc_from = cc_from.trim().into();
    cc_to   = cc_to  .trim().into();

    cc_from = cc_from.to_uppercase();
    cc_to   = cc_to  .to_uppercase();

    let conversion_rate = get_conversion_rate(&cc_from, &cc_to); 
    let converted_amt = amt * conversion_rate;
    println!("\t{} to {}", cc_from, cc_to);
    println!("Exchange Rate: 1 {} = {:.2} {}", cc_from, conversion_rate, cc_to);
    println!("\n=> {:.2} {} = {:.2} {}", amt, cc_from, converted_amt, cc_to);
    println!("\nFor informational purposes only. Thank you.");
}
