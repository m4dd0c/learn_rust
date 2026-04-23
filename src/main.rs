use std::env;
fn main () {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).expect("Must be valid config.");
    // searching for query in provided file_path
    //exec(config);
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
        let mut ignore_case = false;
        if args[3] == "-i" {
            ignore_case = true;
        }
        Ok(Config {query, path, ignore_case})
    }
}