# iron-mustache
Bring mustache template to Iron : https://github.com/iron/iron

Use rustache : https://github.com/FerarDuanSednan/rustache.git

Inspired by https://github.com/sunng87/handlebars-iron.git

*Use this at your own risk.*

#Example
```rust
/// the handler
fn hello_world(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    // open http://localhost:3000/
    let data :BTreeMap<String, Json> = make_data();
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
```
