extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate mount;
extern crate staticfile;
extern crate persistent;
#[macro_use] extern crate aus_senate;
extern crate handlebars_iron as hbs;
#[macro_use] extern crate serde_derive;
extern crate csv;
extern crate env_logger;
extern crate futures_cpupool;
extern crate futures;
#[macro_use] extern crate derive_error;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use urlencoded::UrlEncodedBody;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use aus_senate::candidate::{Candidate, CandidateName, CandidateId, CandidateMap, find_candidates_with_names};
use futures_cpupool::CpuPool;
use futures::Future;

use calc::{load_candidate_data, run_election};
use state::State;
use error::Error::*;

mod calc;
mod state;
mod error;

#[derive(Serialize, Debug)]
struct IndexPageData {
    title: &'static str,
    states: Vec<&'static str>,
}

fn index(_: &mut Request) -> IronResult<Response> {
    let data = IndexPageData {
        title: "Australian Senate Simulator 3000",
        states: State::all_states().iter().map(State::to_str).collect(),
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("index", data))
        .set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct SetupPageData {
    title: String,
}

fn get_query_state(req: &mut Request) -> IronResult<State> {
    req.extensions.get::<Router>()
        .and_then(|qs| qs.find("state"))
        .and_then(|state_name| State::from_str(state_name))
        .ok_or(IronError::from(InvalidState))
}

fn election_setup_page(req: &mut Request) -> IronResult<Response> {
    let state = get_query_state(req)?;

    let page_data = SetupPageData {
        title: format!("{} Senate Election (2016)", state.to_str()),
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("setup", page_data))
        .set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct ElectedSenator {
    position: usize,
    candidate: Candidate,
    vote_tally: String,
}

#[derive(Serialize, Debug)]
struct ResultPageData {
    elected: Vec<ElectedSenator>,
    disqualified: Vec<CandidateName>,
    tied: bool,
}

fn parse_disqualified(
    disqualified_str: String,
    candidate_map: &CandidateMap,
) -> Result<(Vec<CandidateName>, Vec<CandidateId>), ()> {
    let full_names = disqualified_str.split("\r\n").filter(|s| !s.is_empty());
    let mut candidate_names = vec![];
    for name in full_names {
        let split_names: Vec<&str> = name.rsplitn(2, " ").collect();
        if split_names.len() != 2 {
            return Err(());
        }
        candidate_names.push(CandidateName {
            first: split_names[1].to_owned(),
            last: split_names[0].to_owned(),
        });
    }

    let ids = find_candidates_with_names(&candidate_names, candidate_map);

    if ids.len() == candidate_names.len() {
        Ok((candidate_names, ids))
    } else {
        Err(())
    }
}

fn election_result_page(req: &mut Request) -> IronResult<Response> {
    let state = get_query_state(req)?;

    let disqualified_str = req.get_ref::<UrlEncodedBody>()
        .ok()
        .and_then(|form_data| form_data.get("disqualified"))
        .and_then(|values| values.first())
        .cloned()
        .unwrap_or_else(String::new);

    println!("Processing request for {}, disqualified: {:?}", state.to_str(), disqualified_str);

    let cpu_pool = req.get::<persistent::Read<JobPool>>().map_err(|_| InternalErr)?;

    // Run the election computation in the single-threaded pool, so as not to OOM.
    let f = cpu_pool.spawn_fn(move || {
        let data_dir = Path::new("data");
        let candidate_data = load_candidate_data(data_dir, state).unwrap();
        let (disq_names, disq_ids) = parse_disqualified(disqualified_str, &candidate_data.candidates).unwrap();
        let res: Result<_, ()> = Ok((
            run_election(data_dir, state, &candidate_data, &disq_ids).unwrap(),
            disq_names
        ));
        res
    });

    let (result, disqualified) = f.wait().map_err(|_| InternalErr)?;

    println!("Result: {:?}", result);

    let elected = result.senators.into_iter().enumerate().map(|(i, (c, t))| {
        ElectedSenator {
            position: i + 1,
            candidate: c,
            vote_tally: format!("{}", t),
        }
    }).collect();

    let result_data = ResultPageData {
        elected,
        disqualified,
        tied: result.tied,
    };

    let mut resp = Response::new();
    resp.set_mut(Template::new("result", result_data))
        .set_mut(status::Ok);
    Ok(resp)
}

pub struct JobPool;
impl iron::typemap::Key for JobPool {
    type Value = CpuPool;
}

fn main() {
    env_logger::init().expect("env_logger init failed");

    let mut hbse = HandlebarsEngine::new();

    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    hbse.reload().expect("some templates are fukt");

    // Static file serving.
    let mut mount = Mount::new();
    mount.mount("/static/", Static::new(Path::new("./static/")));

    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/:state", election_setup_page, "setup");
    router.post("/:state", election_result_page, "result");
    router.get("/static/*", mount, "static");

    let mut chain = Chain::new(router);

    // Thread pool for running jobs on. Single-threaded to avoid OOM.
    let pool = CpuPool::new(1);
    chain.link_before(persistent::Read::<JobPool>::one(pool));

    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3001").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn empty_disqualified_str() {
        let map = HashMap::new();
        assert_eq!(parse_disqualified("".into(), &map), Ok(vec![]));
    }
}
