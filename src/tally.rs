// tally.rs

use std::collections::HashMap;
use std::{fs::File, path::Path};
use std::io::{BufRead, BufReader, Write};

use serde::{Deserialize, Serialize};

use crate::{Contest, ContestChoice, DecodedContestVote, Error};
use crate::DecodedVoteChoice;

/// The aggregated result of a [Tally]
#[derive(Debug, Serialize)]
pub struct ContestResult {
    /// The [Contest] to which these results belong
    pub contest: Contest,
    /// Total valid votes
    pub total_valid_votes: i64,
    /// Total invalid votes
    pub total_invalid_votes: i64,
    /// The results for every choice
    pub results: Vec<ContestChoiceResult>,
    /// The winners for the contest
    pub winners: Vec<ContestChoice>,
}

impl ContestResult {

    /// Saves the results to a JSON-encoded file and returns the filename
    pub fn save_to_file(&self) -> Result<String, Error> {
        let fname = format!("results-{}.json", self.contest.id());
        let mut file = File::create(&fname)?;
        let serialized = serde_json::to_string_pretty(&self)?;
        file.write_all(serialized.as_bytes())?;
        file.flush()?;
        Ok(fname)
    }

}


/// Detailed tally results for a given [ContestChoice]
#[derive(Debug, Serialize)]
pub struct ContestChoiceResult {
    /// The choice
    pub contest_choice: ContestChoice,
    /// Total number of votes for this choice
    pub total_count: u64,
    /// The position if this choice is among the winners (otherwise zero)
    pub winner_position: u64,
}

/// Vote tallying for any [Contest].
/// Includes the [Contest] object and the collection of submited votes as [FlatVote]s.
#[derive(Debug, PartialEq)]
pub struct Tally {
    contest: Contest,
    votes: Vec<FlatVote>,
}

/// Homologous to [DecodedContestVote] but doesn't include the full
/// [Contest] object.
/// 
/// When working with large samples of vote data, having the [Contest] object
/// included in each vote is redundant and leads to unnecessary memory and
/// disk usage.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FlatVote {
    is_explicit_invalid: bool,
    choices: Vec<DecodedVoteChoice>,
    contest: i64,
}

impl Tally {

    pub fn new(contest: &Contest) -> Self {
        Self{
            contest: contest.clone(),
            votes: Vec::new(),
        }
    }

    /// Sets all votes
    pub fn with_votes(self, votes: Vec<FlatVote>) -> Self {
        Self{
            votes,
            ..self
        }
    }

    /// Adds a single vote
    pub fn add_vote(&mut self, vote: FlatVote) {
        if vote.contest == self.contest.id() {
            self.votes.push(vote);
        }
    }

    /// Saves votes to a file and returns the filename
    pub fn save_to_file(&self) -> Result<String, Error> {
        let fname = format!("votes-{}.json", self.contest.id());
        let mut file = File::create(&fname)?;
        for v in &self.votes {
            let serialized: String = serde_json::to_string(v)?;
            file.write_all(serialized.as_bytes())?; 
            file.write_all("\n".as_bytes())?;
        }
        file.flush()?;
        Ok(fname)
    }

    /// Loads votes for `contest` from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P, contest: &Contest) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut votes = Self::new(contest);
        for line in reader.lines().map_while(Result::ok) {
            let vote: FlatVote = serde_json::from_str(&line)?;
            votes.add_vote(vote);
        }
        Ok(votes)
    }

    /// Count votes and return 
    pub fn result(&self) -> ContestResult {

        let mut invalid_votes: i64 = 0;
        let mut counts = HashMap::new();

        for vote in &self.votes {

            // Skip invalid votes
            if vote.is_explicit_invalid {
                invalid_votes += 1;
                continue;
            }

            // Count all choices for this voter
            for choice in &vote.choices {
                if choice.selected > 0 {
                    counts.entry(choice.contest_choice.id)
                        .and_modify(|e| *e += choice.selected)
                        .or_insert(choice.selected);
                }
            }

        }

        // Calculate winners
        let mut sorted_results: Vec<(i64, u64)> = counts.into_iter().collect();
        sorted_results.sort_by_key(|(_, votes)| std::cmp::Reverse(*votes));

        // Calculate positions
        let positions = Self::calc_positions(&sorted_results, self.contest.num_winners());

        // Accumulate results for each choice as per exercise requirements
        let results = sorted_results.iter().map(|(choice_id, vote_count)| {
            let choice = self.contest.choices().iter()
                .find(|c| c.id == *choice_id)
                .expect("Got a vote for a choice that's not part of the contest");
            let pos = positions.iter()
                .find(|p| p.0 == *choice_id)
                .map(|p| p.1)
                .unwrap_or(0);
            ContestChoiceResult{
                contest_choice: choice.clone(),
                total_count: *vote_count,
                winner_position: pos as u64,
            }
        }).collect();

        // Fill in the winners metadata
        let cutoff = (self.contest.num_winners() as usize).min(sorted_results.len());
        let winners = sorted_results[..cutoff].to_vec();
        let winners = winners.into_iter().map(|(choice_id, _)| {
            self.contest.choices().iter().find(|c| c.id == choice_id)
                .expect("Failed to find winner choice")
                .clone()
        }).collect();

        ContestResult{
           contest: self.contest.clone(),
           total_valid_votes: self.votes.len() as i64 - invalid_votes,
           total_invalid_votes: invalid_votes,
           results,
           winners, 
        }

    }

    /// Calculate positions taking into account potential ties
    /// Receives a sorted array of participants with their votes and
    /// returns an array of participants with their positions
    fn calc_positions(sorted_votes: &[(i64, u64)], num_winners: i64) -> Vec<(i64, i64)> {
    
        if sorted_votes.is_empty(){
            return vec![];
        }

        let mut current_position = 1;
        let mut current_votes = sorted_votes[0].1;

        let mut positions = vec![(sorted_votes[0].0, current_position)];

        for &vote in &sorted_votes[1..] {
            if positions.len() >= num_winners as usize {
                positions.push((vote.0, 0));
            } else {
                if vote.1 != current_votes {
                    current_position += 1;
                    current_votes = vote.1;
                }
                positions.push((vote.0, current_position));    
            }
        }

        positions

    }

}


#[test]
fn test_calc_positions() {

    let votes = vec![
        (100, 10),
        (200, 6),
        (300, 6),
        (400, 4),
        (500, 1),
        (600, 1),
    ];

    let positions = Tally::calc_positions(&votes, 5);

    assert_eq!(1, positions[0].1);
    assert_eq!(2, positions[1].1);
    assert_eq!(2, positions[2].1);
    assert_eq!(3, positions[3].1);
    assert_eq!(4, positions[4].1);
    // Last one should be zero because there's
    // only 5 winners in this contest
    assert_eq!(0, positions[5].1);

}

impl From<DecodedContestVote> for FlatVote {
    fn from(value: DecodedContestVote) -> Self {
        Self{
            is_explicit_invalid: value.is_explicit_invalid,
            choices: value.choices.clone(),
            contest: value.contest.id(),
        }
    }
}
