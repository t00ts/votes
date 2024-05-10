// lib.rs

//! Vote tallying for plurality-at-large[^1] procedures.
//! 
//! This library processes vote data for arbitrary contests and
//! calculates the vote result.
//! 
//! All data can be read from and stored into JSON-encoded files.
//! 
//! ## Tallying
//! The main purpose of this library is processing votes ([DecodedContestVote])
//! for a [Contest] and generating accurate results.
//! 
//! ### Example: Counting votes
//! ```no_run
//! use votes::{ContestBuilder, ContestChoice, DecodedContestVote, DecodedVoteChoice};
//! use votes::{FlatVote, Tally};
//! 
//! // Generate 3 choices
//! let choices = vec![
//!     ContestChoice::new(100, "Mark Knopfler"),
//!     ContestChoice::new(200, "Eric Clapton"),
//!     ContestChoice::new(300, "Jimmy Page"),
//! ];
//! 
//! // Create a contest with 3 winners
//! let contest = ContestBuilder::new(3, &choices)
//!     .description("Guitar legends!")
//!     .max_choices(1)
//!     .min_choices(1)
//!     .build();
//! 
//! // Submit a few votes
//! let decoded_votes = vec![
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
//!     DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[2].clone())]),
//! ];
//! 
//! // Flatten them (remove redundant contest info)
//! let flat_votes = decoded_votes.into_iter().map(|v| FlatVote::from(v)).collect();
//! let tally = Tally::new(&contest).with_votes(flat_votes);
//! 
//! // Tally and get contest results
//! let result = tally.result();
//! 
//! // Save results to a file
//! let filename = result.save_to_file()
//!     .expect("Failed to save contest results to disk");
//! 
//! println!("Results saved to {}", filename);
//! ```
//! 
//! ## Random generation
//! Generate random contest and vote data for testing purposes.
//! 
//! There are three methods for random data generation available for use:
//! 
//! - [gen_random_choices] generates random choices for a [Contest]
//! - [gen_random_contest] generates a random [Contest] with a set of choices
//! - [gen_random_votes] generates random votes for a given [Contest]
//! 
//! ### Example
//! 
//! ```no_run
//! use votes::{gen_random_choices, gen_random_contest, gen_random_votes};
//! use votes::{Tally};
//! 
//! // Generate a contest with random choices
//! let contest = gen_random_contest(5, gen_random_choices(10));
//! 
//! // Generate some random votes for this contest
//! let tally = Tally::new(&contest)
//!     .with_votes(gen_random_votes(10, &contest));
//! 
//! // Get results
//! let result = tally.result();
//! ```
//! 
//! ## Loading and saving data
//! Data can be loaded and saved to JSON-encoded files.
//! 
//! ### Example 1: Saving and loading a [Contest] with its choices
//! 
//! ```no_run
//! use rand::Rng;
//! use votes::{gen_random_contest, gen_random_choices};
//! use votes::{Contest};
//! 
//! // Generate a contest
//! let num_winners = rand::thread_rng().gen_range(1..5);
//! let contest = gen_random_contest(num_winners, gen_random_choices(15));
//! 
//! // Save it to a file
//! let path = contest.save_to_file()
//!     .expect("Failed to save contest data to file");
//! 
//! // Load it again
//! let loaded = Contest::load_from_file(&path)
//!     .expect("Failed to load contest data from file");
//! 
//! // Make sure they match
//! assert_eq!(contest, loaded);
//! ```
//! 
//! ### Example 2: Storing and loading votes
//! 
//! ```no_run
//! use votes::{gen_random_contest, gen_random_choices, gen_random_votes};
//! use votes::Tally;
//! 
//! // Generate a contest
//! let contest = gen_random_contest(5, gen_random_choices(10));

//! // Generate some random votes for this contest
//! let tally = Tally::new(&contest)
//!     .with_votes(gen_random_votes(10, &contest));
//! 
//! // Save these votes to a file
//! let votes_file = tally.save_to_file()
//!     .expect("Failed to save votes");
//! 
//! // Load these votes back
//! let loaded_tally = Tally::load_from_file(&votes_file, &contest)
//!     .expect("Failed to load votes from file");
//! 
//! // Make sure they match
//! assert_eq!(tally, loaded_tally);
//! ```
//! 
//! ### Example 3: Saving contest results
//! 
//! ```no_run
//! use votes::{gen_random_contest, gen_random_choices, gen_random_votes};
//! use votes::Tally;
//! 
//! // Generate a contest
//! let contest = gen_random_contest(5, gen_random_choices(10));
//! 
//! // Generate some random votes for this contest
//! let tally = Tally::new(&contest)
//!     .with_votes(gen_random_votes(10, &contest));
//! 
//! // Get results
//! let result = tally.result();
//! 
//! // Save them to a file
//! let filename = result.save_to_file()
//!     .expect("Failed to save contest results to disk");
//! ```
//! 
//! [^1]: [Plurality Block Voting - Wikipedia](https://en.wikipedia.org/wiki/Plurality_block_voting)


// Where the vote counting takes place
mod tally;
pub use tally::*;

// All our data structures should be available to the end-user
mod model;
pub use model::*;

// The `gen` module exposes generation functions to create random
mod gen;
pub use gen::*;

// Errors produced by the library
mod error;
pub use error::Error;