// test_votes.rs

use votes::{ContestBuilder, ContestChoice, DecodedContestVote, DecodedVoteChoice, FlatVote, Tally};
use votes::{gen_random_choices, gen_random_contest, gen_random_votes};


#[test]
fn simple_contest_test() {

    // Generate 3 choices
    let choices = vec![
        ContestChoice::new(100, "Mark Knopfler"),
        ContestChoice::new(200, "Eric Clapton"),
        ContestChoice::new(300, "Jimmy Page"),
    ];

    // Create a contest with 3 winners
    let contest = ContestBuilder::new(3, &choices)
        .description("A contest with 3 winners")
        .max_choices(1)
        .min_choices(1)
        .build();

    // Submit a few votes
    let decoded_votes = vec![
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[2].clone())]),
    ];

    // Flatten them (remove redundant contest info)
    let flat_votes = decoded_votes.into_iter().map(|v| FlatVote::from(v)).collect();
    let tally = Tally::new(&contest).with_votes(flat_votes);

    // Tally and get contest results
    let result = tally.result();

    // Vote validity checks
    assert_eq!(6, result.total_valid_votes);
    assert_eq!(0, result.total_invalid_votes);
    assert_eq!(3, result.winners.len());

    // Winner checks
    let winner_choice = &result.winners[0];
    assert_eq!(100, winner_choice.id);
    assert_eq!("Mark Knopfler", winner_choice.text);

    let winner_choice_res = result.results.iter().find(|cc| cc.contest_choice.id == 100)
        .expect("Failed to find winner among contest results");

    assert_eq!(3, winner_choice_res.total_count);
    assert_eq!(1, winner_choice_res.winner_position);

    // Second place checks
    let second_choice = &result.winners[1];
    assert_eq!(200, second_choice.id);
    assert_eq!("Eric Clapton", second_choice.text);

    let second_choice_res = result.results.iter().find(|cc| cc.contest_choice.id == 200)
        .expect("Failed to find second among contest results");

    assert_eq!(2, second_choice_res.total_count);
    assert_eq!(2, second_choice_res.winner_position);

    // Third place checks
    let third_choice = &result.winners[2];
    assert_eq!(300, third_choice.id);
    assert_eq!("Jimmy Page", third_choice.text);

    let third_choice_res = result.results.iter().find(|cc| cc.contest_choice.id == 300)
        .expect("Failed to find third among contest results");

    assert_eq!(1, third_choice_res.total_count);
    assert_eq!(3, third_choice_res.winner_position);

}



#[test]
fn simple_contest_test_with_ties () {

    // Generate 4 choices
    let choices = vec![
        ContestChoice::new(100, "John Lennon"),
        ContestChoice::new(200, "Paul McCartney"),
        ContestChoice::new(300, "George Harrison"),
        ContestChoice::new(400, "Ringo Starr"),
    ];

    // Create a contest with 3 winners
    let contest = ContestBuilder::new(3, &choices)
        .description("Rate the best Beatle!")
        .max_choices(1)
        .min_choices(1)
        .build();

    // Submit a few votes
    let decoded_votes = vec![
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[0].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[1].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[2].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[2].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[3].clone())]),
        DecodedContestVote::new(&contest, vec![DecodedVoteChoice::new(choices[3].clone())]),
    ];

    // Flatten them (remove redundant contest info)
    let flat_votes = decoded_votes.into_iter().map(|v| FlatVote::from(v)).collect();
    let tally = Tally::new(&contest).with_votes(flat_votes);

    // Tally and get contest results
    let result = tally.result();

    // Vote validity checks
    assert_eq!(10, result.total_valid_votes);
    assert_eq!(0, result.total_invalid_votes);
    assert_eq!(3, result.winners.len());

    // Sort results
    let mut results = result.results;
    results.sort_by_key(|r| std::cmp::Reverse(r.total_count));

    // Tie checks
    assert_eq!(results[0].total_count, results[1].total_count);
    assert_eq!(results[2].total_count, results[3].total_count);
    assert_eq!(1, results[0].winner_position);
    assert_eq!(1, results[1].winner_position);
    assert_eq!(2, results[2].winner_position);
    
    // Non-winner
    assert_eq!(0, results[3].winner_position);

}


#[test]
fn contest_with_invalid_votes () {

    // Generate 4 choices
    let choices = vec![
        ContestChoice::new(100, "BB King"),           // 0
        ContestChoice::new(200, "Robert Johnson"),
        ContestChoice::new(300, "Muddy Waters"),      // 2
        ContestChoice::new(400, "John Lee Hooker"),
        ContestChoice::new(500, "Etta James"),        // 4
        ContestChoice::new(600, "Buddy Guy"),
        ContestChoice::new(700, "Stevie Ray Vaughan"),// 6
        ContestChoice::new(800, "Elmore James"),
    ];

    // Create a contest with 3 winners
    let contest = ContestBuilder::new(3, &choices)
        .description("The Blues All-Star Showdown")
        .max_choices(3)
        .min_choices(2)
        .build();

    // Submit a few votes
    let decoded_votes = vec![
        // Valid votes (5)
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[0].clone()), 
            DecodedVoteChoice::new(choices[2].clone()),
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[0].clone()), 
            DecodedVoteChoice::new(choices[2].clone()),
            DecodedVoteChoice::new(choices[4].clone()),
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[0].clone()), 
            DecodedVoteChoice::new(choices[6].clone()),
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[3].clone()), 
            DecodedVoteChoice::new(choices[6].clone()),
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[0].clone()), 
            DecodedVoteChoice::new(choices[5].clone()), 
            DecodedVoteChoice::new(choices[7].clone()),
        ]),
        // Invalid votes (4)
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[3].clone()), 
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[0].clone()), 
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[4].clone()),
        ]),
        DecodedContestVote::new(&contest, vec![
            DecodedVoteChoice::new(choices[3].clone()), 
            DecodedVoteChoice::new(choices[4].clone()), 
            DecodedVoteChoice::new(choices[6].clone()),
            DecodedVoteChoice::new(choices[7].clone()),
        ]),
    ];

    // Flatten them (remove redundant contest info)
    let flat_votes = decoded_votes.into_iter().map(|v| FlatVote::from(v)).collect();
    let tally = Tally::new(&contest).with_votes(flat_votes);

    // Tally and get contest results
    let result = tally.result();

    // Vote validity checks
    assert_eq!(5, result.total_valid_votes);
    assert_eq!(4, result.total_invalid_votes);
    assert_eq!(3, result.winners.len());

    // Sort results
    let mut results = result.results;
    results.sort_by_key(|r| std::cmp::Reverse(r.total_count));

    // One winner w/ 4 votes
    assert_eq!(100, results[0].contest_choice.id);
    assert_eq!(1, results[0].winner_position);
    assert_eq!(4, results[0].total_count);

    // Tie on second place (rules out invalid votes)
    assert_eq!(2, results[1].winner_position);
    assert_eq!(results[1].winner_position, results[2].winner_position);
    assert_eq!(results[1].total_count, results[2].total_count);

    // The rest of participants that didn't win
    assert_eq!(0, results[3].winner_position);
    assert_eq!(0, results[4].winner_position);
    assert_eq!(0, results[5].winner_position);
    assert_eq!(0, results[6].winner_position);

}

#[test]
fn test_with_random_generator() {

    for num_winners in 1..5 {

        // Generate a contest
        let contest = gen_random_contest(num_winners, gen_random_choices(10));

        // Generate some random votes for this contest
        let tally = Tally::new(&contest)
            .with_votes(gen_random_votes(200, &contest));

        // Tally the votes
        let result = tally.result();

        // Make sure we have `num_winners` winners
        assert_eq!(result.winners.len() as i64, num_winners);

        // Make sure there's max `num_winners` in the result positions
        let mut win_count = 0;
        let mut at_least_one_winner = false;
        for ccr in result.results {
            if ccr.winner_position > 0 {
                win_count += 1;
            }
            if ccr.winner_position == 1 {
                at_least_one_winner = true;
            }
        }
        assert!(at_least_one_winner);
        assert!(win_count <= num_winners)

    }

}
