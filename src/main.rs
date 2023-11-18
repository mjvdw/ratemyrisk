extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

fn main() {
    load_risk_framework();
    // println!("Hello, world!");
}

fn load_risk_framework() {
    let mut file = File::open("src/framework.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    println!("{}", contents);
}

#[derive(Deserialize, Debug)]
struct Root {
    consequence: Consequence,
    likelihood: Likelihood,
    ratings: Vec<Vec<String>>,
    governance: Governance,
}

#[derive(Deserialize, Debug)]
struct Consequence {
    financial: Severity,
    reputation: Severity,
    regulatory: Severity,
    resilience: Severity,
    people: Severity,
}

#[derive(Deserialize, Debug)]
struct Severity {
    severe: Description,
    major: Description,
    moderate: Description,
    minor: Description,
    insignificant: Description,
}

#[derive(Deserialize, Debug)]
struct Description {
    description: String,
    index: u8,
}

#[derive(Deserialize, Debug)]
struct Likelihood {
    almost_certain: LikelihoodIndex,
    likely: LikelihoodIndex,
    possible: LikelihoodIndex,
    unlikely: LikelihoodIndex,
    rare: LikelihoodIndex,
}

#[derive(Deserialize, Debug)]
struct LikelihoodIndex {
    title: String,
    index: u8,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Governance {
    low: String,
    medium: String,
    high: String,
    extreme: String,
}
