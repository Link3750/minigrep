use std::{error::Error, fs, process};

use clap::Parser;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case == "1" {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[derive(Parser)]
#[command(name = "config")]
#[command(author = "短途游")]
#[command(version = "1.0")]
#[command(about = "搜索给定文件中的关键词")]
pub struct Config {
    /// 关键词
    #[arg(short='q', long)]
    pub query: String,
    /// 文件位置
    #[arg(short='p', long="path")]
    pub file_path: String,
    /// 是否忽略大小写
    #[arg(short='i', long, default_value = "0")]
    pub ignore_case: String,
}

impl Config {

    pub fn build() -> Result<Config, &'static str> {
        let (query, file_path, ignore_case) = Config::parse_arguements().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        Ok(Config {query,file_path,ignore_case,})
    }

    fn parse_arguements() -> Result<(String, String, String), &'static str> {
        let config = Config::parse();
        let query = config.query;
        let file_path = config.file_path;
        let ignore_case = config.ignore_case;

        Ok((query, file_path, ignore_case))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
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