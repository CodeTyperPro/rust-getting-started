use std::fmt;
use std::env;
use std::fs;
use std::collections::HashMap;
use serde::Deserialize;
use reqwest::Error;


#[derive(Clone)]
#[derive(Debug)]
#[derive(Deserialize)] // Derive Deserialize trait for Candidate
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

fn announce_election(state: &mut ElectionState) {
    *state = ElectionState::Announced;
}

fn add_candidate(candidate: Candidate, hash_candidates: &mut HashMap<u64, Candidate>) {
    hash_candidates.insert(candidate.code, candidate.clone());
}

fn add_voter(voter: Voter, state: &mut ElectionState, hash_voters: &mut HashMap<String, u64>, array_voters: &mut Vec<Voter>) {
    array_voters.push(voter.clone());
    hash_voters.insert(voter.identifier.clone(), voter.choice_code);
    *state = ElectionState::Happening;
}

fn start_election(state: &mut ElectionState) {
    *state = ElectionState::Started;
}

fn end_election(state: &mut ElectionState) {
    *state = ElectionState::Ended;
}

fn give_right_to_vote() {
    // Implementation goes here
}

fn place_vote(voter: &Voter, hash_voters: &HashMap<String, u64>, hash_candidates: &mut HashMap<u64, Candidate>) {
    assert!(!has_voted(voter), "Voter already voted.");
    assert!(exists_voter(voter, hash_voters), "Voter does not exist.");
    if let Some(candidate) = hash_candidates.get_mut(&voter.choice_code) {
        candidate.num_votes += 1;
        // Mark as already voted
    }
}

fn winning_candidate(hash_candidates: &HashMap<u64, Candidate>) -> Option<Candidate> {
    let mut winner_candidate: Option<Candidate> = None;
    for (_, val) in hash_candidates.iter() {
        match winner_candidate {
            None => winner_candidate = Some(val.clone()),
            Some(ref x) => {
                if val.num_votes > x.num_votes {
                    winner_candidate = Some(val.clone());
                }
            }
        }
    }
    winner_candidate
}

fn exists_voter(voter: &Voter, hash_voters: &HashMap<String, u64>) -> bool {
    hash_voters.contains_key(&voter.identifier)
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


fn parse_candidate(line: &str) -> Option<Candidate> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 3 {
        let name = String::from(parts[0].trim());
        let num_votes = parts[1].trim().parse::<u64>().ok()?;
        let code = parts[2].trim().parse::<u64>().ok()?;
        Some(create_candidate(name, num_votes, code))
    } else {
        None
    }
}

fn parse_voters(line: &str) -> Option<Voter> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() == 2 {
        let identifier = String::from(parts[0].trim());
        let choice_code = parts[1].trim().parse::<u64>().ok()?;
        Some(create_voter(identifier, choice_code))
    } else {
        None
    }
}

fn reading_file_candidates(file_path: &String, hash_candidates: &mut HashMap<u64, Candidate>) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let candidates: Vec<Candidate> = contents
        .lines()
        .filter_map(|line| parse_candidate(line))
        .collect();

    for candidate in &candidates {
        hash_candidates.insert(candidate.code, candidate.clone());
    }

    Ok(())
}

fn reading_file_voters(file_path: &String, hash_voters: &mut HashMap<String, u64>) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let voters: Vec<Voter> = contents
        .lines()
        .filter_map(|line| parse_voters(line))
        .collect();

    for voter in &voters {
        hash_voters.insert(voter.identifier.clone(), voter.choice_code);
    }

    Ok(())
}

fn display_voters(hash_voters: &HashMap<String, u64>) {
    println!("Voters: ");
    for (key, val) in hash_voters.iter() {
        println!("{}: {}", key, val);
    }
}

fn display_candidates(hash_candidates: &HashMap<u64, Candidate>) {
    println!("Candidates: ");
    for (_, val) in hash_candidates.iter() {
        println!("{}", val);
    }
}

async fn get_json_file_from_url() -> Result<Vec<Candidate>, reqwest::Error> {
    let file_url = "https://example-files.online-convert.com/document/txt/example.json";
    let response = reqwest::get(file_url).await?;

    if !response.status().is_success() {
        println!("Failed to fetch data: {}", response.status());
    }

    let candidates: Vec<Candidate> = response.json().await?;
    println!("{:?}", candidates);

    Ok(candidates)
}

fn load_data() {
    let file_url = "test-immutable-file.txt";
    let file_git = "https://github.com/CodeTyperPro/rust-getting-started/blob/main/smart-contract/file_candidates.txt";
    use rusty_leveldb::{DB, DBIterator, LdbIterator, Options};
    
    let mut opt = Options::default();
    opt.create_if_missing = true;

    let mut db = DB::open(file_url, opt).unwrap();

    db.put(b"Hello", b"World").unwrap();
    assert_eq!(b"World", db.get(b"Hello").unwrap().as_slice());

    let mut iter = db.new_iter().unwrap();
    // Note: For efficiency reasons, it's recommended to use advance() and current() instead of
    // next() when iterating over many elements.
    assert_eq!((b"Hello".to_vec(), b"World".to_vec()), iter.next().unwrap());

    // db.delete(b"Hello").unwrap();
    db.flush().unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <query> <file_candidates> <file_voters>", args[0]);
        return;
    }

    let query = &args[1];
    let file_candidates = &args[2];
    let file_voters = &args[3];

    println!("Searching for {}", query);
    println!("In the file {}", file_candidates);
    println!("In the file {}", file_voters);

    // test();
    
    let mut state: ElectionState = ElectionState::Announced;
    let mut hash_voters: HashMap<String, u64> = HashMap::new();
    let mut array_voters: Vec<Voter> = Vec::new();
    let mut hash_candidates: HashMap<u64, Candidate> = HashMap::new();
    
/*    reading_file_candidates(file_candidates, &mut hash_candidates).expect("Error reading candidates file");
    reading_file_voters(file_voters, &mut hash_voters).expect("Error reading voters file");

    winning_candidate(&hash_candidates);
    display_voters(&hash_voters);
    display_candidates(&hash_candidates);*/

    get_json_file_from_url().await;
}
