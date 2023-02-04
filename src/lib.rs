use getopts::Options;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const LINE_OPTION: &'static str = "n";
const LINE_MODE: &'static str = "line";
const CHAR_OPTION: &'static str = "c";
const CHAR_MODE: &'static str = "char";
const DEAFULT_LINES_NUMBER: usize = 10;

pub struct Config {
    filename: String,
    output_mode: String,
    limit_num: usize,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    println!("{}", opts.usage(&brief));
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let program = &args[0];
        let mut opts = Options::new();

        opts.optopt(LINE_OPTION, "lines", "output first NUM lines", "NUM");
        opts.optopt(CHAR_OPTION, "bytes", "output first NUM chars", "NUM");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(msg) => {
                println!("Error: {}", msg.to_string());
                print_usage(program, opts);
                return Err("error caused in parsing auguments");
            }
        };

        let mut limit_num = DEAFULT_LINES_NUMBER;
        let mut output_mode = LINE_MODE.to_string();

        if matches.opt_present(LINE_OPTION) {
            if let Some(text) = matches.opt_str(LINE_OPTION) {
                match text.parse::<usize>() {
                    Ok(number) => {
                        limit_num = number;
                    }
                    Err(msg) => {
                        println!("Error: {}", msg.to_string());
                        print_usage(program, opts);
                        return Err("invalid number of lines");
                    }
                }
            }
        } else if matches.opt_present(CHAR_OPTION) {
            if let Some(text) = matches.opt_str(CHAR_OPTION) {
                match text.parse::<usize>() {
                    Ok(number) => {
                        output_mode = CHAR_MODE.to_string();
                        limit_num = number;
                    }
                    Err(msg) => {
                        // display usage
                        println!("Error: {}", msg.to_string());
                        print_usage(program, opts);
                        return Err("invalid number of chars");
                    }
                }
            }
        }

        if matches.free.is_empty() {
            print_usage(program, opts);
            return Err("set filename in arguments");
        } else {
            let filename = String::from(&matches.free[0]);
            Ok(Config {
                filename,
                output_mode,
                limit_num,
            })
        }
    }
}

pub fn extract_line(buff: &mut BufReader<File>, limit_num: usize) -> String {
    let mut results = String::new();

    for _ in 0..limit_num {
        buff.read_line(&mut results).expect("failed to read file");
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let result = match config.output_mode.as_str() {
        LINE_MODE => {
            let mut buff = BufReader::new(f);
            extract_line(&mut buff, config.limit_num)
        }
        CHAR_MODE => {
            let mut buff = Vec::with_capacity(config.limit_num);
            f.read(&mut buff)?;
            String::from_utf8(buff)?
        }
        _ => return Err("no match output mode".into()),
    };

    for line in result.lines() {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{process::Output, ptr::null};

    use super::*;

    #[test]
    fn config_normal_test1() {
        let command_input = "minihead test1.txt -n 10";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let config = Config::new(&args).unwrap();
        assert_eq!("test1.txt", config.filename);
        assert_eq!("line", config.output_mode);
        assert_eq!(10, config.limit_num);
    }

    #[test]
    fn config_normal_test2() {
        let command_input = "minihead test2.txt -c 5";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let config = Config::new(&args).unwrap();
        assert_eq!("test2.txt", config.filename);
        assert_eq!("char", config.output_mode);
        assert_eq!(5, config.limit_num);
    }

    #[test]
    fn config_normal_test3() {
        let command_input = "minihead test3.txt -n 10 -c 5";
        let args: Vec<String> = command_input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let config = Config::new(&args).unwrap();
        assert_eq!("test3.txt", config.filename);
        assert_eq!("line", config.output_mode);
        assert_eq!(10, config.limit_num);
    }

    #[test]
    fn extract_normal_test1() {
        let testdata_path = "tests/testdata/spec.md";
        let f = File::open(testdata_path).unwrap();
        let mut buff = BufReader::new(f);
        let ok_contents = "\
# head
## NAME
       head - output the first part of files
";
        assert_eq!(ok_contents, extract_line(&mut buff, 3));
    }

    #[test]
    fn run_normal_test1() {
        let filename = "tests/testdata/spec.md".to_string();
        let output_mode = "char".to_string();
        let limit_num = 5;

        let config: Config = Config {
            filename,
            output_mode,
            limit_num,
        };
        let ok_contents = "\
# head
## NAME
       head - output the first part of files
";
        // assert_eq!(true, return_true());
    }
}
