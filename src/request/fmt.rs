use json::{
    parse
};

pub fn pick_data(data : &str) -> String {
       parse(data).unwrap()["data"]["info"].dump()
}
