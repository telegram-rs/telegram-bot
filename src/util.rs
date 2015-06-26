use super::Result;
use rustc_serialize::{json, Encodable};

// Type for managing GET and POST parameter
pub struct Params<'a> {
    gets: Vec<(&'a str, String)>,
}

impl<'a> Params<'a> {
    pub fn new() -> Params<'a> {
        Params {
            gets: Vec::new(),
        }
    }

    pub fn add_get_opt<T: ToString>(&mut self, key: &'a str, value: Option<T>) {
        if let Some(d) = value {
            self.gets.push((key, d.to_string()));
        }
    }
    pub fn add_get<T: ToString>(&mut self, key: &'a str, value: T) {
        self.gets.push((key, value.to_string()));
    }

    pub fn add_get_json_opt<T: Encodable>(&mut self,
                            key: &'a str, value: Option<T>) -> Result<()> {
        if let Some(d) = value {
            self.gets.push((key, try!(json::encode(&d))));
        }
        Ok(())
    }

    pub fn get_params(&self) -> &Vec<(&str, String)> {
        &self.gets
    }
}
