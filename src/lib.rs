use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("недостаточно аргументов");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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

    #[test]
    fn config_new_creates_config_with_query_and_filename() {
        let args = vec![
            String::from("program_name"),
            String::from("test_query"),
            String::from("test_file.txt"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, "test_query");
        assert_eq!(config.filename, "test_file.txt");
    }

    #[test]
    fn config_new_returns_error_with_insufficient_arguments() {
        let args = vec![String::from("program_name")];
        let result = Config::new(&args);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "недостаточно аргументов");
    }

    #[test]
    fn run_reads_file_contents() {
        // Создаем временный файл для теста
        let test_filename = "test_file.txt";
        let test_contents = "test file contents";
        fs::write(test_filename, test_contents).unwrap();

        let config = Config {
            query: String::from("query"),
            filename: String::from(test_filename),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };

        // Запускаем функцию и проверяем что она завершается без ошибок
        let result = run(config);
        assert!(result.is_ok());

        // Удаляем временный файл
        fs::remove_file(test_filename).unwrap();
    }

    #[test]
    fn run_returns_error_for_nonexistent_file() {
        let config = Config {
            query: String::from("query"),
            filename: String::from("nonexistent_file.txt"),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };

        let result = run(config);
        assert!(result.is_err());
    }
}