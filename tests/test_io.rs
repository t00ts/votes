// test_io.rs

use std::fs;
use rand::Rng;
use votes::{Contest, Tally};
use votes::{gen_random_choices, gen_random_contest, gen_random_votes};


#[test]
fn test_io_contest() {

    // Generate a contest
    let num_winners = rand::thread_rng().gen_range(1..5);
    let contest = gen_random_contest(num_winners, gen_random_choices(15));

    // Save it to a file
    let path = contest.save_to_file()
        .expect("Failed to save contest data to file");
    
    // Load it again
    let loaded = Contest::load_from_file(&path)
        .expect("Failed to load contest data from file");
    
    // Make sure they match
    assert_eq!(contest, loaded);

    // Remove tmp file
    fs::remove_file(&path)
        .expect("Failed to remove file after test");

}


#[test]
fn test_io_votes() {

    // Generate a contest
    let contest = gen_random_contest(5, gen_random_choices(10));

    // Generate some random votes for this contest
    let tally = Tally::new(&contest)
        .with_votes(gen_random_votes(10, &contest));

    // Save these votes to a file
    let votes_file = tally.save_to_file()
        .expect("Failed to save votes");

    // Load these votes and check they match
    let loaded_tally = Tally::load_from_file(&votes_file, &contest)
        .expect("Failed to load votes from file");

    assert_eq!(tally, loaded_tally);

    // Remove tmp file
    fs::remove_file(&votes_file)
        .expect("Failed to remove file after test");

}


#[test]
fn test_io_results() {

    // Generate a contest
    let contest = gen_random_contest(5, gen_random_choices(10));

    // Generate some random votes for this contest
    let tally = Tally::new(&contest)
        .with_votes(gen_random_votes(10, &contest));

    let result = tally.result();
    let results_file = result.save_to_file()
        .expect("Failed to save contest results to disk");

    // Remove tmp file
    fs::remove_file(&results_file)
        .expect("Failed to remove file after test");

}