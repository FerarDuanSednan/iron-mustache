extern crate iron;
extern crate env_logger;
extern crate iron_mustache as irm;
extern crate rustc_serialize;

use std::collections::BTreeMap;

use iron::prelude::*;
use iron::{status};
use irm::{Template, MustacheEngine};
use rustc_serialize::json::{ToJson, Json};

struct Team {
    name: String,
    pts: u16
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("pts".to_string(), self.pts.to_json());
        m.to_json()
    }
}

fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
    pts: 43u16 },
    Team { name: "Beijing Guoan".to_string(),
    pts: 27u16 },
    Team { name: "Guangzhou Evergrand".to_string(),
    pts: 22u16 },
    Team { name: "Shandong Luneng".to_string(),
    pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}

/// the handler
fn hello_world(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    // open http://localhost:3000/
    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();

    let mut chain = Chain::new(hello_world);
    let muse = MustacheEngine::new("./examples");

    chain.link_after(muse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

