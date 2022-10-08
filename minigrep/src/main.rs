use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        panic!(r#"
        The length of arguments must be 2.
        You sepcified '{}'.
        
        FYI:
            usage:
                <bin name> <target expression> <file path>
        "#, &args[1..].join(" "));
        return;
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).unwrap_or_else(|e| panic!("{} not found. error detail: {:?}", filename, e));

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap_or_else(|e| panic!("something went wrong reading the file. error detail: {:?}", e));
    println!("With text:\n{}", contents);
}
