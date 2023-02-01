use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments!");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(config.filename)?;
    let mut buff = BufReader::new(f);

    for line in extract(&mut buff).lines() {
        println!("{}", line);
    }

    Ok(())
}

pub fn extract(buff: &mut BufReader<File>) -> String {
    let mut results = String::new();
    let lines_num = 3;

    for _ in 0..lines_num {
        buff.read_line(&mut results).expect("failed to read file");
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_test1() {
        let f = File::open("spec.md").unwrap();
        let mut buff = BufReader::new(f);
        let ok_contents = "\
# head
## NAME
       head - output the first part of files
";
        assert_eq!(ok_contents, extract(&mut buff));
    }
}
