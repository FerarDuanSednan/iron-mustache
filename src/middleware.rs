use std::io::Read;

use rustc_serialize::json::{Json, ToJson};

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use iron::headers::ContentType;

use plugin::Plugin as PluginFor;

use rustache::{self, Render};

#[derive(Clone)]
pub struct Template {
    name: String,
    value: Json
}

impl Template {

    pub fn new<T: ToJson>(name: &str, value: T) -> Template {
        Template {
            name: name.to_string(),
            value: value.to_json()
        }
    }
}

#[derive(Clone)]
pub struct MustacheEngine {
    source: String
}

impl typemap::Key for MustacheEngine {
    type Value = Template;
}

impl Modifier<Response> for Template {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<MustacheEngine>(self);
    }
}

impl PluginFor<Response> for MustacheEngine {
    type Error = ();

    fn eval(resp: &mut Response) -> Result<Template, ()> {
        match resp.extensions.get::<MustacheEngine>(){
            Some(t) => Ok(t.clone()),
            None => Err(())
        }
    }
}

impl MustacheEngine {

    pub fn new(src: &str) -> MustacheEngine {
        MustacheEngine { source: src.to_string() }
    }

    pub fn render(&self, filename: &ToString, data: &Json) -> Result<String, rustache::RustacheError> {
        let mut w = String::new();

        match rustache::render_file(&*format!("{}/{}.mustache", self.source, filename.to_string()), data.clone()) {
            Ok(ref mut r) => {
                r.read_to_string(&mut w).unwrap();
                Ok(w)
            },
            Err(e) => Err(e)
        }
    }
}

impl AfterMiddleware for MustacheEngine {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let page_wrapper = resp.extensions.get::<MustacheEngine>().as_ref()
            .and_then(|h| {
                Some(self.render(&h.name, &h.value))
            });

        match page_wrapper {
            Some(page_result) => {
                match page_result {
                    Ok(page) => {
                        if !resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType::html());
                        }
                        resp.set_mut(page);
                        Ok(resp)
                    }
                    Err(e) => {
                        info!("{}", e);
                        Err(IronError::new(e, status::InternalServerError))
                    }
                }
            }
            None => {
                Ok(resp)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use iron::prelude::*;
    use middleware::*;

    fn hello_world() -> IronResult<Response> {
        let resp = Response::new();

        let mut data = HashMap::new();
        data.insert("title".to_string(), "Mustache on Iron".to_string());

        Ok(resp.set(Template::new("index", data)))
    }

    #[test]
    fn test_resp_set() {
        let mut resp = hello_world().ok().expect("response expected");

        // use response plugin to retrieve a cloned template for testing
        match resp.get::<MustacheEngine>() {
            Ok(h) => {
                assert_eq!(h.name, "index".to_string());
                assert_eq!(h.value.as_object().unwrap()
                           .get(&"title".to_string()).unwrap()
                           .as_string().unwrap(),
                           "Mustache on Iron");
            },
            _ => panic!("template expected")
        }
    }

}
