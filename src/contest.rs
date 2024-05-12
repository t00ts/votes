// contest.rs

use std::{fs::File, path::Path};
use std::io::{self, Read, Write};

use rand::Rng;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

/// A contest with its choices
/// 
/// Use [ContestBuilder] to create a new [Contest] with all possible options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contest {
    id: i64,
    description: String,
    tally_type: String,
    num_winners: i64,
    min_choices: i64,
    max_choices: i64,
    choices: Vec<ContestChoice>,
}

impl Contest {

    pub fn builder() -> ContestBuilder {
        ContestBuilder::default()
    }

    /// The contest ID
    pub fn id(&self) -> i64 {
        self.id
    }

    /// All available choices for this contest
    pub fn choices(&self) -> &Vec<ContestChoice> {
        &self.choices
    }

    /// Number of winners
    pub fn num_winners(&self) -> i64 {
        self.num_winners
    }

    /// Save contest JSON data to a file
    pub fn save_to_file(&self) -> Result<String, io::Error> {
        let fname = format!("contest-{}.json", self.id);
        let mut file = File::create(&fname)?;
        let serialized = serde_json::to_string_pretty(&self)?;
        file.write_all(serialized.as_bytes())?;
        file.flush()?;
        Ok(fname)
    }

    /// Load contest data from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: Self = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }

}

/// Factory to create and configure all properties of a new [Contest]
#[derive(Default)]
pub struct ContestBuilder {
    id: i64,
    description: String,
    tally_type: String,
    num_winners: i64,
    min_choices: i64,
    max_choices: i64,
    choices: Vec<ContestChoice>,
}

impl ContestBuilder {

    pub fn new(num_winners: i64, choices: &[ContestChoice]) -> ContestBuilder {
        ContestBuilder{
            id: rand::thread_rng().gen_range(0..1_000_000),
            num_winners,
            choices: choices.to_owned(),
            tally_type: "plurality-at-large".to_string(),
            ..Default::default()
        }
    }

    /// Set a custom [Contest] ID
    pub fn custom_id(mut self, id: i64) -> ContestBuilder {
        self.id = id;
        self
    }

    /// Provide a description
    pub fn description(mut self, desc: &str) -> ContestBuilder {
        self.description = desc.to_owned();
        self
    }

    /// Sets the minimum number of choices per vote
    pub fn min_choices(mut self, choices: i64) -> ContestBuilder {
        self.min_choices = choices;
        self
    }

    /// Sets the maximum number of choices per vote
    pub fn max_choices(mut self, choices: i64) -> ContestBuilder {
        self.max_choices = choices;
        self
    }

    /// Builds the [Contest]
    pub fn build(self) -> Contest {
        Contest{
            id: self.id,
            description: self.description,
            tally_type: self.tally_type,
            num_winners: self.num_winners,
            min_choices: self.min_choices,
            max_choices: self.max_choices,
            choices: self.choices,
        }
    }


}


/// A choice for a [Contest]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContestChoice {
    pub id: i64,
    pub text: String,
    pub urls: Vec<String>,
}

impl ContestChoice {

    /// Create a new `ContestChoice`
    pub fn new(id: i64, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            urls: vec![],
        }
    }

    /// Add a URL to this `ContestChoice`
    pub fn add_url(&mut self, url: &str) {
        self.urls.push(url.to_string());
    }

}

/// A vote for a [Contest]. It can include many choices.
#[derive(Clone,  Deserialize)]
pub struct DecodedContestVote {
    /// Indicates that this vote is invalid
    pub is_explicit_invalid: bool,
    /// The choices submitted within this vote
    pub choices: Vec<DecodedVoteChoice>,
    /// The [Contest] for which this vote was emitted
    pub contest: Contest,
}

impl DecodedContestVote {

    pub fn new(contest: &Contest, choices: Vec<DecodedVoteChoice>) -> Self {
        let is_valid = Self::is_valid(contest, &choices);
        Self{
            choices,
            contest: contest.clone(),
            is_explicit_invalid: !is_valid,
        }
    }

    /// Explicitly invalidates this vote
    pub fn invalidate(&mut self) {
        self.is_explicit_invalid = true;
    }

    /// Determines whether a vote is valid or not according to contest rules
    pub fn is_valid(contest: &Contest, choices: &[DecodedVoteChoice]) -> bool {
        let count = choices.len() as i64;
        count <= contest.max_choices && count >= contest.min_choices
    }

}

// Note: We don't really use this serializer as we favor `Votes` and `FlatVote` instead
impl Serialize for DecodedContestVote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("DecodedContestVote", 3)?;
        state.serialize_field("is_explicit_invalid", &self.is_explicit_invalid)?;
        state.serialize_field("choices", &self.choices)?;
        state.serialize_field("contest", &self.contest.id)?;
        state.end()
    }
}


/// A choice with the number of votes assigned to that choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecodedVoteChoice {
    /// The choice that was made
    pub contest_choice: ContestChoice,
    /// The number of votes that were assigned
    pub selected: u64,
}

impl DecodedVoteChoice {

    pub fn new(choice: ContestChoice) -> Self {
        Self {
            contest_choice: choice,
            selected: 1,
        }
    }

}
