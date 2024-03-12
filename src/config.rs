use std::fmt::Error;

#[derive(Clone)]
pub struct Config {
    pub p: i32,
    pub braid_rep: String,
    pub n: i32,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, Error> {
        let p = args[3]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse int"));
        let n = args[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse int"));
        Ok(Config {
            p,
            braid_rep: args[2].to_string(),
            n,
        })
    }
}
