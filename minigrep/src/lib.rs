use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Parameter");
        }
        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut f = File::open(&self.filename)?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        for line in search(&self.query, &contents) {
            println!("{}", line)
        }

        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Rust";
        let contents = "\
              Rust:
              safe, fast, productive,
              Pick three.";
        assert_eq!(vec!["Rust"], search(query, contents))
    }
}
