use std::error::Error;
use std::{fs, vec, env};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(list: &[String]) -> Result<Config, &'static str> {
        if list.len() < 3 {
            return Err("not enough arguments");
        }
        let query = list[1].clone();
        let filename = list[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents);
    } else {
        search_case_insensitive(&config.query, &contents);
    };

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }
    // search(&config.query, &contents);
    // println!("{}", contents);
    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    return result;
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query_str = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_str) {
            result.push(line);
        }
    }
    result
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
    fn case_insenstive() {
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
