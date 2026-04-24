use std::env;
use crate::grep::exec;
fn main () {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).expect("Must be valid config.");
    // searching for query in provided file_path
    exec(&config);
}

struct Config {
    ignore_case: bool,
    path: String,
    query: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str>{
        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = args.get(3).map_or(false, |arg| arg == "-i");
        Ok(Config {query, path, ignore_case})
    }
}

mod grep {
    use super::Config;
    use std::fs;

    pub fn exec(conf: &Config) {
        // reading the file
        let content = read_file(&conf.path).expect("File should open.");
        let results: Vec<String>;
        // calling search based on case sensi
        if conf.ignore_case {
            results = search_case_insensitive(&conf.query, &content);
        }else {
            results = search_case_sensitive(&conf.query, &content);
        }
        println!("\nFound: {:?}\n", results)
    }
    fn read_file(path: &String) -> Result<String, std::io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    fn search_case_sensitive(query: &String, content: &String) -> Vec<String> {
        let mut results = Vec::new();
        for line in content.lines() {
            if line.contains(query){
                results.push(line.to_string());
            }
        }
        results
    }
    fn search_case_insensitive(query: &String, content: &String) -> Vec<String> {
        let mut results = Vec::new();
        for line in content.to_lowercase().lines() {
            if line.contains(&query.to_lowercase()){
                results.push(line.to_string());
            }
        }
        results
    }
}
