use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if required amount of arguments not supplied
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].to_string();
        let file_path = args[2].to_string();

        let ignore_case: bool = if args.len() >= 4 {
            parse_bool_arg(&args[3])?
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn parse_bool_arg(arg: &str) -> Result<bool, &'static str> {
    match arg.to_lowercase().as_str() {
        "true" | "yes" | "1" => Ok(true),
        "false" | "no" | "0" => Ok(false),
        _ => Err("Invalid boolean value defining case sensitivity"),
    }
}

/// # Errors
///
/// Will return `Err` if file at end of`file_path` does not exist or the
/// user does not have permission to read it.
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

#[must_use]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

#[must_use]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ips";
        let contents = "\
Lorem ipsum
dolor
sit amet";

        assert_eq!(vec!["Lorem ipsum"], search(query, contents));
    }

    #[test]
    fn no_result() {
        let query = "hello";
        let contents = "\
Lorem ipsum
dolor
sit amet";

        assert_eq!(0, search(query, contents).len());
    }

    #[test]
    fn case_sensitive() {
        let query = "olo";
        let contents = "\
Lorem ipsum
dolor
sit amet
dOLOr";

        assert_eq!(vec!["dolor"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "iPs";
        let contents = "\
Lorem ipsum
dolor
sit amet";

        assert_eq!(
            vec!["Lorem ipsum"],
            search_case_insensitive(query, contents)
        );
    }
}
