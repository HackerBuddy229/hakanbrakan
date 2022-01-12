use std::env;
use std::fs;
use std::string::String;
use std::path::Path;



fn main() {
    println!("Hello, world!");

    //get file name from arg
    let mut args: Vec<String> = env::args().collect();
    let filename = args.pop().unwrap();

    let path = Path::new(&filename);

    println!("{:?}", path);

    //get stream
    //fetch text
    let code: String = fs::read_to_string(&path).unwrap();
    
    //split

    let split_char: char = char::from_u32(0x000A).unwrap();
    let split_code = code.split(split_char);
    
    //shorten

    let mut out: String = String::new();

    for line in split_code {
        let mut new_line:String;

        if line.len() <= 80 { 
            new_line = line.to_string(); 
        } else {
            new_line = line[..80].to_string();
        }

        

        out = format!("{}{}\n", out, new_line)
    }


    //save

    fs::write(&path, out.as_bytes());
}
