use std::fs;
use std::str::FromStr;

pub fn read_strings(path: &str) -> Vec<String> {
    let inp = fs::read_to_string(path).expect("failed to read input file");
    inp.trim().split('\n').map(|x| x.to_string()).collect()
}

pub fn read_nums<T: FromStr>(path: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let inp = fs::read_to_string(path).expect("failed to read input file");
    inp.trim()
        .split('\n')
        .map(|s| s.parse::<T>().expect("failed to parse"))
        .collect()
}
