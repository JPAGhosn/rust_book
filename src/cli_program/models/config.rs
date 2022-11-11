
pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument to process the command");
        }
        let query = args.get(1).expect("Query cannot be null").clone();
        let file_path = args.get(2).expect("Filepath cannot be null").clone();
        Ok(Config {
            query,
            file_path
        })
    }
}