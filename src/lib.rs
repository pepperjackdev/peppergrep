use std::{env, error::Error, fs, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough arguments passed... please use `minigrep <query> <file> [--ignore-case]`".to_string());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let mut ignore_case = env::var("IGNORE_CASE").is_ok(); 

        // if the fourth argument is provided and equals "--ignore-case", the env. variable is overrided
        if args.len() > 3 {
            let ignore_case_flag = &args[3];
            if ignore_case_flag == "--ignore-case" {
                ignore_case = true;
            } else {
                return Err(format!("invalid flag `{ignore_case_flag}`, try using --ignore-case instead."));
            }
        }

        Ok(Config {
            query, 
            file_path, 
            ignore_case
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}