use rustc_serialize::{json, Decodable};

#[derive(RustcDecodable, Debug)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(RustcDecodable, Debug)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
}
