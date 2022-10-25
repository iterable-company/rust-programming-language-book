use std::{error::Error, fs::File, io::Read};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            Err("two arguments must be specified.".to_string())?;
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        let contents = r#"
Rust:
safe, fast, productive.
Pick three."#;

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
