/*
	Name: test_match
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing match
*/

// Declare a 'VoterCard' struct
struct VoterCard{
	cast_vote: Option<String>,
}

// Report a match
fn report_match(this_choice: &str){
	match this_choice{
		"y" => println!("You did agree"),
		"n" => println!("You did not agree"),
		_ => println!("Please only choose 'y' or 'n' "),
	}
}

// Report option 
fn report_vote(voter_card: &VoterCard){
	match &voter_card.cast_vote{
		Some(choice) => println!("You casted a vote for : {:?}", choice),
		None => println!("You did not cast a vote!"),
	}
}

// Main test driver 
fn main(){
	let this_choice = "y";
	match this_choice{
		"y" => println!("You did agree"),
		"n" => println!("You did not agree"),
		_ => println!("Please only choose 'y' or 'n' "),
	}
	report_match("n");
	let voter_card: VoterCard = VoterCard{cast_vote: Some("Bill".to_string())};
	report_vote(&voter_card);
	let anoter_voter_card: VoterCard = VoterCard{cast_vote: None};
	report_vote(&anoter_voter_card);
}