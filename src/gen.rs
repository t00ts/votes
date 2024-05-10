// gen.rs

use std::collections::HashSet;

use rand::Rng;

use crate::tally::FlatVote; 
use crate::model::{Contest, ContestBuilder, ContestChoice, DecodedContestVote, DecodedVoteChoice};

static NAMES: [&str; 100] = ["Alexander", "Olivia", "William", "Emma", "Ethan", "Sophia", "Benjamin", "Isabella", "James", "Mia", "Michael", "Charlotte", "Daniel", "Amelia", "Matthew", "Harper", "Jackson", "Evelyn", "David", "Abigail", "Joseph", "Emily", "Samuel", "Elizabeth", "Henry", "Avery", "Christopher", "Sofia", "Andrew", "Ella", "Lucas", "Scarlett", "Gabriel", "Grace", "Joshua", "Lily", "John", "Chloe", "Isaac", "Zoey", "Nathan", "Madison", "Oliver", "Aria", "Dylan", "Riley", "Elijah", "Layla", "Caleb", "Penelope", "Anthony", "Victoria", "Mason", "Natalie", "Logan", "Lucy", "Aaron", "Nora", "Jack", "Lillian", "Jonathan", "Hannah", "Ryan", "Addison", "Nicholas", "Eleanor", "Adam", "Aubrey", "Zachary", "Stella", "Levi", "Savannah", "Aiden", "Brooklyn", "Julian", "Claire", "Christian", "Violet", "Brayden", "Skylar", "Samuel", "Paisley", "Xavier", "Audrey", "Cameron", "Leah", "Connor", "Sadie", "Jeremiah", "Ariana", "Hunter", "Allison", "Thomas", "Sarah", "Charles", "Caroline", "Eli", "Naomi", "Jordan", "Katherine"];

/// Generate a [Contest] with `num_winners` and `choices`.
/// 
/// `max_choices` and `min_choices` will be randomly generated according to
/// the specified `num_winners`.
pub fn gen_random_contest(num_winners: i64, choices: Vec<ContestChoice>) -> Contest {

    let max_choices = match num_winners {
        1 => 1,
        _ => rand::thread_rng().gen_range(1..num_winners),
    };
    let min_choices = match max_choices {
        1 => 1,
        _ => rand::thread_rng().gen_range(1..max_choices),
    };

    ContestBuilder::new(num_winners, &choices)
        .description("A random contest")
        .max_choices(max_choices)
        .min_choices(min_choices)
        .build()

}

/// Generate `count` random choices for a [Contest]
/// - ID's for choices will be randomly generated integers in the range `0..1000`
/// and are guaranteed to be unique on every generated set.
/// - Names for the choices are chosen from a basket of 100 names and are not
/// guaranteed to be unique.
pub fn gen_random_choices(count: usize) -> Vec<ContestChoice> {

    let ids: HashSet<i64> = HashSet::new();
    (0..count).map(|_| {
        // Generate a unique, random ID
        let mut id = rand::thread_rng().gen_range(0..1_000);
        while ids.contains(&id) {
            id = rand::thread_rng().gen_range(0..1_000);
        }
        // Use a random name (could be repeated)
        let name = NAMES[rand::thread_rng().gen_range(1..100)];
        // Create a `ContestChoice`
        ContestChoice::new(id, name)
    }).collect()

}

/// Generate `count` random votes for `contest`
/// 
/// **Note:** Both valid and invalid votes will be generated
/// 
/// - Each voter will submit at least one vote (choice)
/// - Each voter will submit a maximum of all the available choices for the contest
/// 
/// 
/// 
pub fn gen_random_votes(count: usize, contest: &Contest) -> Vec<FlatVote> {

    (0..count).map(|_| {
        let mut available_choices = contest.choices().clone();

        // Generate a random number of choices for this voter
        let num_choices = match available_choices.len() {
            1 => 1,
            _ => rand::thread_rng().gen_range(1..available_choices.len())
        };
        let choices = (0..num_choices).map(|_| {
            // Select a random choice
            let j = rand::thread_rng().gen_range(0..available_choices.len());
            let choice = DecodedVoteChoice::new(available_choices[j].clone());
            // Prevent double voting
            available_choices.remove(j);
            choice
        }).collect();

        // Use provided vote struct for the sake of using it and
        // checking vote validity
        DecodedContestVote::new(contest, choices).into()

    }).collect()
    
}
