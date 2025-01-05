use std::collections::HashMap;

use tcrate::process;

use process::{substitute, subtract, process};

#[test]
fn test_substitute(){
    let mut eq = String::from("ab*cd = e(e+e)+a(c-d(a+b))");
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), "5".to_string());
    vars.insert("b".to_string(), "3".to_string());
    vars.insert("c".to_string(), "2".to_string());
    vars.insert("d".to_string(), "8".to_string());

    println!("{}",substitute(&mut eq, vars));
}

#[test]
fn test_subtract(){
    let mut eq = String::from("ab*cd = e(e+e)+a(c-d(a+b))");
    println!("{}", subtract(&mut eq));
}

#[test]
fn test_process(){
    let mut eq = String::from("ab*cd = e(e+e)+a(c-d(a+b))");
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), "5".to_string());
    vars.insert("b".to_string(), "3".to_string());
    vars.insert("c".to_string(), "2".to_string());
    vars.insert("d".to_string(), "8".to_string());

    println!("{}", process(&mut eq, vars));
}