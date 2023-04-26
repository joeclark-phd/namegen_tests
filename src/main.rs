use std::fs::File;
use std::io::{BufReader, BufRead};
use markov_namegen::{CharacterChainGenerator, ClusterChainGenerator};
use markov_namegen::RandomTextGenerator;

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

    println!("\n10 more with a pattern that keeps the length from 4-10 characters:");
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


    println!("\n10 more with a pattern that requires they end with -ia:");
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

    println!("\n10 names based on uk_surnames.txt:");
    let file4 = File::open("resources/uk_surnames.txt").unwrap();
    let reader4 = BufReader::new(file4);
    let lines4 = reader4.lines().map(|l| l.unwrap() );
    let uk_names = CharacterChainGenerator::builder()
        .train(lines4)
        .build();
    for _i in 0..10 {
        println!("{}", uk_names.generate_one());
    }

    println!("\n10 names based on periodic_elements.txt:");
    let file5 = File::open("resources/periodic_elements.txt").unwrap();
    let reader5 = BufReader::new(file5);
    let lines5 = reader5.lines().map(|l| l.unwrap() );
    let elements_names = CharacterChainGenerator::builder()
        .train(lines5)
        .build();
    for _i in 0..10 {
        println!("{}", elements_names.generate_one());
    }

    println!("\n10 names based on pokemon_modern.txt (length 6-12):");
    let file6 = File::open("resources/pokemon_modern.txt").unwrap();
    let reader6 = BufReader::new(file6);
    let lines6 = reader6.lines().map(|l| l.unwrap() );
    let poke_names = CharacterChainGenerator::builder()
        .train(lines6)
        .with_pattern("^[a-z]{6,12}$")
        .build();
    for _i in 0..10 {
        println!("{}", poke_names.generate_one());
    }


    // ClusterChainGenerator examples


    println!("\nHere are some tests of the ClusterChainGenerator!\n");

    println!("10 names based on romans.txt with default settings:");
    let file11 = File::open("resources/romans.txt").unwrap();
    let reader11 = BufReader::new(file11);
    let lines11 = reader11.lines().map(|l| l.unwrap() );
    let roman_names = ClusterChainGenerator::builder()
        .train(lines11)
        .with_prior(0.0001)
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }

    println!("\n10 more with a pattern that keeps the length from 4-10 characters:");
    let file12 = File::open("resources/romans.txt").unwrap();
    let reader12 = BufReader::new(file12);
    let lines12 = reader12.lines().map(|l| l.unwrap() );
    let roman_names = ClusterChainGenerator::builder()
        .train(lines12)
        .with_prior(0.0001)
        .with_pattern("^[a-z]{4,10}$")
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }


    println!("\n10 more with a pattern that requires they end with -ia:");
    let file13 = File::open("resources/romans.txt").unwrap();
    let reader13 = BufReader::new(file13);
    let lines13 = reader13.lines().map(|l| l.unwrap() );
    let roman_names = ClusterChainGenerator::builder()
        .train(lines13)
        .with_pattern("^[a-z]{2,8}ia$")
        .build();
    for _i in 0..10 {
        println!("{}", roman_names.generate_one());
    }

    println!("\n10 names based on uk_surnames.txt, without priors:");
    let file14 = File::open("resources/uk_surnames.txt").unwrap();
    let reader14 = BufReader::new(file14);
    let lines14 = reader14.lines().map(|l| l.unwrap() );
    let uk_names = ClusterChainGenerator::builder()
        .train(lines14)
        .without_prior()
        .build();
    for _i in 0..10 {
        println!("{}", uk_names.generate_one());
    }

    println!("\n10 names based on periodic_elements.txt:");
    let file15 = File::open("resources/periodic_elements.txt").unwrap();
    let reader15 = BufReader::new(file15);
    let lines15 = reader15.lines().map(|l| l.unwrap() );
    let elements_names = ClusterChainGenerator::builder()
        .train(lines15)
        .build();
    for _i in 0..10 {
        println!("{}", elements_names.generate_one());
    }

    println!("\n10 names based on pokemon_modern.txt (length 6-12):");
    let file16 = File::open("resources/pokemon_modern.txt").unwrap();
    let reader16 = BufReader::new(file16);
    let lines16 = reader16.lines().map(|l| l.unwrap() );
    let poke_names = ClusterChainGenerator::builder()
        .train(lines16)
        .with_prior(0.0001)
        .with_pattern("^[a-z]{6,12}$")
        .build();
    for _i in 0..10 {
        println!("{}", poke_names.generate_one());
    }



}
