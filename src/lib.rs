use std::{fs, env, process, error::Error};

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
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

    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();
        
        let (query, file_path, ignore_case) = Config::parse_arguements(args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        Ok(Config {query,file_path,ignore_case,})
    }

    fn parse_arguements(mut args: impl Iterator<Item = String>) -> Result<(String, String, bool), &'static str> {
        let mut query = String::new();
        let mut file_path = String::new();
        let mut ignore_case = env::var("IGNORE_CASE").map_or(false, |var| var.eq("1"));
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-q" | "--query" => {
                    query = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a query string"),
                    }
                },
                "-p" | "--path" => {
                    file_path = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a file path"),
                    };
                },
                "-i" | "--ignore_case" => {
                    ignore_case = match args.next() {
                        Some(arg) if arg.as_str().eq("true") => true,
                        Some(arg) if arg.as_str().eq("false")  => false,
                        _ => return Err("wrong input after -i or --ignore_case")
                    };
                }
                _ => return Err("Illegal arguments")
            };
        };

        Ok((query, file_path, ignore_case))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a> (
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], search_case_insensitive(query, contents));
    }
}