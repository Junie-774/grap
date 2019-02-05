use std::env;
use std::fs;

use regex::Regex;

struct ArgParseResult(bool, bool, Vec<Regex>, Vec<String>);

fn parse_args(args: Vec<String>) -> Result<ArgParseResult, String> {
    let mut ignore_case = false;
    let mut invert_match = false;
    let mut patterns: Vec<Regex> = Vec::new();
    let mut files_to_search: Vec<String> = Vec::new();

    let mut ignore_next = false;
    for (i, arg) in args.iter().enumerate() {

        if ignore_next {
            ignore_next = false;
            continue
        }

        match arg.as_str() {
            "-i" => ignore_case = true,
            "-v" => invert_match = true,

            "-e" => {
                let val: String = args.get(i + 1).unwrap().to_string();
                patterns.push(Regex::new(val.as_str()).unwrap());
                ignore_next = true;
            },

            arg => {
                if patterns.is_empty() {
                    patterns.push(Regex::new(arg).unwrap());
                } else {
                    files_to_search.push(String::from(arg))
                }
            }
        }
    }

    Ok(ArgParseResult(ignore_case, invert_match, patterns, files_to_search))
}

fn main() {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let ArgParseResult(ignore_case, invert_match, mut patterns, files) = match parse_args(args) {
        Ok(result) => result,
        Err(_) => {
            println!("Usage: grap [OPTIONS]... PATTERN [FILES]...");
            return;
        }
    };

    if ignore_case {
        patterns = patterns.into_iter().map(|reg: Regex| {
            Regex::new(reg.as_str()
                .to_lowercase()
                .as_str())
                .unwrap()}
        ).collect();
    }

    let patterns = patterns;


    // Used for filtering out non-matching lines
    let matches_pattern= |contents: &String| -> bool {
        let mut has_match = false;

        let contents = if ignore_case {contents.to_lowercase()} else {contents.clone()};
        for pat in &patterns {
            has_match = has_match || pat.is_match(contents.as_str());
        }

        if invert_match { !has_match } else {has_match}
    };


    let mut lines: Vec<String> = Vec::new();
    for mut file in files {
        println!("File: {}", file);

        let contents: Vec<String> = fs::read_to_string(file.clone())
           .expect((String::from("Could not open file: ") + &mut file).as_str())
           .lines().map(String::from)
           .collect();

        let mut matches:Vec<String> = contents.into_iter()
            .filter(matches_pattern)
            .collect();

        for line in &matches {
            println!("  {}", line);
        }
        lines.append(&mut matches);

        println!();
    }

}
