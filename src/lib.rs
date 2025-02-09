use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if required amount of arguments not
    /// supplied
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].to_string();
        let file_path = args[2].to_string();

        Ok(Config { query, file_path })
    }
}

/// # Errors
///
/// Will return `Err` if `file_path` does not exist or the user does not have
/// permission to read it.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
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
}
