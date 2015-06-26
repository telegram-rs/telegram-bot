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

    pub fn get_params(&self) -> &Vec<(&str, String)> {
        &self.gets
    }
}
