use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Parameter");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let query = &args[1];
        let filename = &args[2];

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut f = File::open(&self.filename)?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        let result = if self.case_sensitive {
            search(&self.query, &contents)
        } else {
            search_case_insensitive(&self.query, &contents)
        };

        for line in result {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
