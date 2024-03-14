use std::fmt;
use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Candidate {
    name: String,
    num_votes: u64,
    code: u64,
}

#[derive(Clone)]
struct Voter {
    identifier: String,
    choice_code: u64,
}

enum ElectionState {
    Announced,
    Started,
    Happening,
    Ended,
}

static mut state: ElectionState = ElectionState::Announced;
static mut hash_voters: HashMap<String, u64> = HashMap::new();
static mut array_voters: Vec<Voter> = Vec::new();
static mut hash_candidates: HashMap<u64, Candidate> = HashMap::new();

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.num_votes, self.code)
    }
}

impl fmt::Display for Voter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.identifier, self.choice_code)
    }
}

fn create_voter(identifier: String, choice_code: u64) -> Voter {
    Voter {
        identifier,
        choice_code,
    }
}

fn create_candidate(name: String, num_votes: u64, code: u64) -> Candidate {
    Candidate {
        name,
        num_votes,
        code,
    }
}

fn announce_election() {
    state = ElectionState::Announced;
}

fn add_candidate(candidate: Candidate) {
    hash_candidates.insert(candidate.code, candidate.clone());
}

fn add_voter(voter: Vote, hash_voters: HashMap<String, u64>r) {
    array_voters.push(voter.clone());
    hash_voters.insert(voter.identifier, voter.choice_code);
    state = ElectionState::Happening;
}

fn start_election() {
    state = ElectionState::Started;
}

fn end_election() {
    state = ElectionState::Ended;
}

fn give_right_to_vote() {

}

fn place_vote(voter: Voter) {
    assert!(has_voted(&voter), "Candidate already voted.");
    assert!(exists_voter(&voter), "Voter does not exist.");
    if let Some(candidate) = hash_candidates.get_mut(&voter.choice_code) {
        candidate.num_votes += 1;
        // mark already voted
    }
}

unsafe fn winning_candidate(hash_candidates: HashMap<u64, Candidate>) -> Option<Candidate> {
    let mut winner_candidate: Option<Candidate> = None;
    for (_, val) in hash_candidates {
        match winner_candidate {
            None => winner_candidate = Some(val),
            Some(ref x) => {
                if val.num_votes > x.num_votes {
                    winner_candidate = Some(val);
                }
            }
        }
    }
    winner_candidate
}

fn exists_voter(voter: &Voter) -> bool {
    unsafe {
        hash_voters.contains_key(&voter.identifier)
    }
}

fn has_voted(voter: &Voter) -> bool {
    voter.choice_code <= 0
}

fn test() {
    let voter1 = create_voter("1234".to_string(), 2);
    let candidate1 = create_candidate("Alfredo Martins".to_string(), 100, 3);

    println!("{}", voter1);
    println!("{}", candidate1);
}

fn reading_file(file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");
    println!("With text:\n{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <query> <file_path>", args[0]);
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In the file {}", file_path);

    reading_file(file_path);
    test();

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }


    winning_candidate();
}
