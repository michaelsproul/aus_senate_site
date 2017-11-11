extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate aus_senate;
extern crate handlebars_iron as hbs;
#[macro_use] extern crate serde_derive;
extern crate csv;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use hbs::{Template, HandlebarsEngine, DirectorySource};

use calc::{load_candidate_data, run_election};
use state::State;

mod calc;
mod state;

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", ())).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct SetupPageData {
    state_name: String,
}

fn get_query_state(req: &mut Request) -> IronResult<State> {
    // FIXME: all these unwraps are pretty bad
    let state_name = req.extensions.get::<Router>().unwrap().find("state").unwrap();
    Ok(State::from_str(state_name).unwrap())
}

fn election_setup_page(req: &mut Request) -> IronResult<Response> {
    let state = get_query_state(req)?;

    let mut resp = Response::new();
    resp.set_mut(Template::new("setup", SetupPageData { state_name: state.to_str().to_owned() })).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct ResultPageData {
    winner: String,
}

fn election_result_page(req: &mut Request) -> IronResult<Response> {
    let state = get_query_state(req)?;
    let form_data = req.get_ref::<UrlEncodedBody>().expect("missing form body");

    println!("Here's your form data, biatch: {:?}", form_data);

    let data_dir = Path::new("data");

    let candidate_data = load_candidate_data(data_dir, state).unwrap();
    let result = run_election(data_dir, state, &candidate_data, &[]).unwrap();

    println!("Stuff: {:?}", result);

    let mut resp = Response::new();
    resp.set_mut(Template::new("result", ResultPageData { winner: format!("neoliberalism") }))
        .set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut hbse = HandlebarsEngine::new();

    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    hbse.reload().expect("some templates are fukt");

    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/:state", election_setup_page, "setup");
    router.post("/:state", election_result_page, "result");

    let mut chain = Chain::new(router);
    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3001").unwrap();
}
