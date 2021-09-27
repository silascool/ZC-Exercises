//use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::{File, write};
use std::collections::HashMap;
use std::io::{Result, Read};

fn read_from_libary(file: &str) -> Result<HashMap<String, bool>> {
    let file = File::open(file);
    let mut contents = String::new();
    let lib: HashMap<String, bool> = match file {
        Ok(mut f) => {
            f.read_to_string(&mut contents)?;
            from_str(&contents)?
        }
        Err(_)=> initilize_libary()
    };
    Ok(lib)
}

fn initilize_libary() -> HashMap<String, bool> {    
    let mut libary: HashMap<String, bool> = HashMap::new();
    libary.insert(String::from("Winds Of Winter"), false);
    libary.insert(String::from("Brothers Karamazov"), true);
    libary.insert(String::from("War And Peace"), true);
    libary.insert(String::from("The Catcher In The Rye"), true);
    libary.insert(String::from("Where The Crawdads Sing"), true);
    libary
}

fn borrow(book: &str, libary: &mut HashMap<String, bool>) {
    match libary.get_mut(book){
        Some(bo) if *bo => {
            println! ("    Here is '{}', remember to give it back when your finished", book);
            *bo = false;
        },
        Some(_) => println! ("    '{}' is already rented out", book),
        
        None => println! ("   We do not have that book in the libary"),
    }
}

fn returning(book: &str, libary: &mut HashMap<String, bool>) {
    match libary.get_mut(book) {
        Some(available) if !*available => {
            println! ("    Hope you liked '{}'.", book);
            *available = true;
        }
        Some(_) => {
            println! ("    That is not our book.");
        }
        None => println! ("    We do not have that book in the libary"),
    }
}

fn list(libary: &HashMap<String, bool>) {
    println!("----------------------------------------" );
    println!("{:<25}| {:<10}", "Books", "Availability");
    println!("----------------------------------------" );
    for (book, inn) in libary {
        let availability = if *inn {"In"} else {"out"};
        println!("{:<25}| {:<10}", book, availability);
    }
    println!("----------------------------------------" );
}

fn main() -> Result<()>{
    // opens libary if it exits else it initilizes a new one
    let mut libary: HashMap<String, bool> = read_from_libary("lib.json").unwrap();
    let mut input = String::new();
    // our program loop, we read the input from the user and call list, borrow
    // or returning depending on users input.
    loop {
        std::io::stdin().read_line(&mut input)?; 
        let words = input.trim_start().split_once(|c| c==' ' || c=='\n');
        match words {
            Some(("q",_)) | Some(("quit",_)) => {
                println!("    Have a nice day");
                break;
            },
            Some(("list",_)) => list(&libary),
            Some(("borrow", rest)) => {
                borrow(rest.trim_end(), &mut libary);
            }
            Some(("return", rest)) => returning(rest.trim_end(), &mut libary),
            _ => println!("    Sorry did not understand, try again"),
        }
        input = String::from("");
    }
    // converts our libary two json and writes it to lib.json
    let lib_str = serde_json::to_string_pretty(&libary)?;
    File::create("lib.json")?;
    write("lib.json", lib_str)?;
    Ok(())
}
