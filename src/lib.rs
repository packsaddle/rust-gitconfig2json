extern crate serde_json;
extern crate gitconfig;

use std::error::Error;
use gitconfig::{Value, Map};
use gitconfig::map::Entry;

pub fn run(message: &str) -> Result<String, Box<Error>> {
    match serde_json::to_string(&convert(message.parse()?)) {
        Ok(t) => Ok(t),
        Err(e) => Err(Box::new(e)),
    }
}

fn convert(git_config: gitconfig::Value) -> serde_json::Value {
    match git_config {
        gitconfig::Value::String(s) => serde_json::Value::String(s),
        gitconfig::Value::Object(map) => serde_json::Value::Object(
            map.into_iter().map(|(k, v)| (k, convert(v))).collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn parse() {
        let mut f = File::open("git-config-list-null.txt").expect("file not found");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect(
            "something went wrong reading the file",
        );
        println!("{}", buf);
        println!("----");
        println!("{:?}", run(buf.as_ref()).unwrap());
    }

    #[test]
    fn convert_empty() {
        let target = gitconfig::Map::new();
        let map = gitconfig::Value::Object(target);
        let converted = convert(map);
        println!("empty !! {}", serde_json::to_string(&converted).unwrap());
    }

    #[test]
    fn convert_one() {
        // {"key": "value"}
        let mut target = gitconfig::Map::new();
        target.insert(
            "key".to_owned(),
            gitconfig::Value::String("value".to_owned()),
        );
        let map = gitconfig::Value::Object(target);
        let converted = convert(map);
        println!("{}", serde_json::to_string(&converted).unwrap());
    }

    #[test]
    fn convert_one_another() {
        // {"key": "value"}
        let mut target = gitconfig::Map::new();
        match target.entry("key") {
            gitconfig::map::Entry::Occupied(mut occupied) => unimplemented!(),
            gitconfig::map::Entry::Vacant(vacant) => {
                vacant.insert(gitconfig::Value::String("value".to_owned()));
                ()
            }
        }
        let map = gitconfig::Value::Object(target);
        let converted = convert(map);
        println!("{}", serde_json::to_string(&converted).unwrap());
    }

    #[test]
    fn convert_two() {
        // {"key1": {"key2": "value2"}}
        let mut internal = gitconfig::Map::new();
        internal.insert(
            "key2".to_owned(),
            gitconfig::Value::String("value2".to_owned()),
        );
        let mut external = gitconfig::Map::new();
        external.insert("key1".to_owned(), gitconfig::Value::Object(internal));
        let map = gitconfig::Value::Object(external);
        let converted = convert(map);
        println!("{}", serde_json::to_string(&converted).unwrap());
    }
}
