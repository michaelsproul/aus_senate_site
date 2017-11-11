use std::io;
use std::fs::File;
use std::path::Path;
use std::error::Error;

use aus_senate::group::*;
use aus_senate::candidate::*;
use aus_senate::voting::*;
use aus_senate::ballot_parse::*;
use aus_senate::parse::candidates2016;
use aus_senate::senate_result::Senate;

use state::State;

// FIXME: move most of this stuff into `aus_senate` crate.

fn candidates_file(data_dir: &Path) -> io::Result<File> {
    File::open(data_dir.join("candidate_ordering.csv"))
}

fn preferences_file(data_dir: &Path, state: State) -> io::Result<File> {
    File::open(data_dir.join(state.to_str().to_owned() + ".csv"))
}

pub struct CandidateData {
    candidates: CandidateMap,
    candidate_list: Vec<CandidateId>,
    group_list: Vec<Group>,
}

pub fn load_candidate_data(data_dir: &Path, state: State) -> Result<CandidateData, Box<Error>> {
    let all_candidates = candidates2016::parse(candidates_file(data_dir)?)?;
    let candidates = get_state_candidates(&all_candidates, state.to_str());
    let candidate_list = get_candidate_id_list(&all_candidates, state.to_str());
    let group_list = get_group_list(&all_candidates, state.to_str());

    Ok(CandidateData {
        candidates,
        candidate_list,
        group_list,
    })
}

pub fn run_election<'a>(
    data_dir: &Path,
    state: State,
    candidates: &'a CandidateData,
    disqualified_candidates: &[CandidateId],
) -> Result<Senate<'a>, Box<Error>>
{
    let prefs_file = preferences_file(data_dir, state)?;

    let constraints = Constraints::official();

    let mut csv_reader = ::csv::ReaderBuilder::new()
        .comment(Some('-' as u8))
        .from_reader(prefs_file);
    let ballots_iter = parse_preferences_file!(
        csv_reader, &candidates.group_list, &candidates.candidate_list, &constraints
    );

    let num_senators = state.num_senators();

    decide_election(&candidates.candidates, disqualified_candidates, ballots_iter, num_senators)
}
