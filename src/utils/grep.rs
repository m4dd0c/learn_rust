mod grep {
    pub fn exec(conf: &Config) {
        // reading the file
        let content = read_file(&conf.path).expect("File should open.");
        // calling search based on case sensi
        if conf.ignore_case {
            search_case_insensitive(&conf.query, &content)
        }else {
            search_case_sensitive(&conf.query, &content)
        }
    }
    fn read_file(path: &String) -> Vec<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    fn search_case_sensitive(query: String, content: String) {
        let results = Vec::new();
        for line in content.lines() {
            if line.contains(query){
                results.push(line);
            }
        }
        results
    }
    fn search_case_insensitive(query: String, content: String) {
        let results = Vec::new();
        for line in content.lines().to_lowercase() {
            if line.contains(query.to_lowercase()){
                results.push(line);
            }
        }
        results
    }
}