extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate aus_senate;
extern crate handlebars_iron as hbs;
#[macro_use] extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use hbs::{Template, HandlebarsEngine, DirectorySource};

fn index(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", ())).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct SetupPageData {
    state_name: String,
}

fn election_setup_page(req: &mut Request) -> IronResult<Response> {
    let state_name = req.extensions.get::<Router>().unwrap().find("state").unwrap().to_uppercase();

    let mut resp = Response::new();
    resp.set_mut(Template::new("setup", SetupPageData { state_name })).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Debug)]
struct ResultPageData {
    winner: String,
}

fn election_result_page(req: &mut Request) -> IronResult<Response> {
    let form_data = req.get_ref::<UrlEncodedBody>().expect("missing form body");

    println!("Here's your form data, biatch: {:?}", form_data);

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
