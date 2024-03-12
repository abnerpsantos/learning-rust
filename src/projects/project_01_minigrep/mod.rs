use std::{ env, process, fs, io };

#[derive(Clone)]
struct Config {
    query: String,
    file_path: String,
}

struct File {
    content: String,
    config: Config
}

impl File {
    fn new(config: &Config) -> Self {
        const DEFAULT_PATH: &str = "./src/projects/project_01_minigrep";
        let contents: Result<String, io::Error> = fs::read_to_string(format!("{}/{}", DEFAULT_PATH, config.file_path));

        match contents {
            Err(error) => panic!("Error reading file: {}",error),
            Ok(content) => Self {
                content,
                config: config.clone()
            }
        }
    }
    fn search(&self) -> Vec<&str> {
        let mut results: Vec<&str> = Vec::new();

        for line in self.content.lines() {
            if line.to_lowercase().contains(self.config.query.to_lowercase().as_str()) {
                results.push(line);
            }
        }
    
        results
    }
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query: String = args[1].to_string();
        let file_path: String = args[2].to_string();
        Ok(Config { query, file_path })
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file: File = File::new(&config);
    let found_lines: Vec<&str> = file.search();

    println!("found lines in {}: {:#?}",config.file_path, found_lines)

}
