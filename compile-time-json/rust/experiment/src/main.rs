#[macro_use]
extern crate compile_time_json;

#[load_json(parsed_json, MyCoolStruct, "../simple.json")]

fn main() {
    println!("{:?}", parsed_json.key1);
}
