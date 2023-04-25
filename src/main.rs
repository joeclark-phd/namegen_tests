use std::fs::File;
use std::io::{BufReader, BufRead};
use markov_namegen::characterchain::generator::CharacterChainGenerator;
use markov_namegen::interface::RandomTextGenerator;

fn main() {
    println!("Here are some tests of the markov_namegen crate!\n");

    println!("10 names based on romans.txt with default settings:");
    let file1 = File::open("resources/romans.txt").unwrap();
    let reader1 = BufReader::new(file1);
    let lines1 = reader1.lines().map(|l| l.unwrap() );
    let roman_names = CharacterChainGenerator::builder()
        .train(lines1)
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }

    println!("\n10 more with a pattern that keeps the length from 4-10 characters");
    let file2 = File::open("resources/romans.txt").unwrap();
    let reader2 = BufReader::new(file2);
    let lines2 = reader2.lines().map(|l| l.unwrap() );
    let roman_names = CharacterChainGenerator::builder()
        .train(lines2)
        .with_pattern("^[a-z]{4,10}$")
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }


    println!("\n10 more with a pattern that requires they end with -ia");
    let file3 = File::open("resources/romans.txt").unwrap();
    let reader3 = BufReader::new(file3);
    let lines3 = reader3.lines().map(|l| l.unwrap() );
    let roman_names = CharacterChainGenerator::builder()
        .train(lines3)
        .with_pattern("^[a-z]{2,8}ia$")
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }

    println!("\n10 names based on uk_surnames.txt");
    let file4 = File::open("resources/uk_surnames.txt").unwrap();
    let reader4 = BufReader::new(file4);
    let lines4 = reader4.lines().map(|l| l.unwrap() );
    let uk_names = CharacterChainGenerator::builder()
        .train(lines4)
        .build();
    for _i in 0..10 {
        println!("{}", uk_names.generate_one());
    }

    println!("\n10 names based on periodic_elements.txt");
    let file5 = File::open("resources/periodic_elements.txt").unwrap();
    let reader5 = BufReader::new(file5);
    let lines5 = reader5.lines().map(|l| l.unwrap() );
    let elements_names = CharacterChainGenerator::builder()
        .train(lines5)
        .build();
    for _i in 0..10 {
        println!("{}", elements_names.generate_one());
    }





}
