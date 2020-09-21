#![deny(warnings)]

use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let host = "http://10.10.10.50:8080/from cont2";
    println!("GET {}", host);
    
    loop {
        println!("New Request \n ==========================");

        thread::sleep(time::Duration::from_secs(3));

        let mut res = match reqwest::blocking::get(host){
            Ok(x)   => {x},
            Err(e)  => { 

                eprintln!("Error could not connect to {}", host);
                eprintln!("Error {}", e);
                
                continue
            },
        };

        println!("Status: {}", res.status());
        println!("Headers:\n{:?}", res.headers());
        res.copy_to(&mut std::io::stdout())?;
        println!("\n\nDone.");

    }

} 
