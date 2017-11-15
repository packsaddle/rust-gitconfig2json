use std::error::Error;
use std::str;
use std::iter::FromIterator;

pub use gitconfig::map::Map;
use gitconfig::map::Entry;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Object(Map<String, Value>),
}

impl Value {
    /// Returns true if the `Value` is an Object. Returns false otherwise.
    ///
    /// For any Value on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the `Value` is an Object, returns the associated Map. Returns None
    /// otherwise.
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is an Object, returns the associated mutable Map.
    /// Returns None otherwise.
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Object(ref mut map) => Some(map),
            _ => None,
        }
    }
}

impl str::FromStr for Value {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Value, Box<Error>> {
        let git_configs = Vec::from_iter(s.split("\0").map(String::from));
        let mut map = Map::new();

        for git_config in &git_configs {
            if git_config.is_empty() {
                continue;
            }
            let (keys, value) = split_once(git_config);
            if keys.len() == 0 {
                continue;
            }
            let split_keys = Vec::from_iter(keys.split(".").map(String::from));
            match split_keys.len() {
                1 => {
                    map.insert(split_keys[0].to_owned(), Value::String(value.to_owned()));
                    ()
                }
                2 => {
                    // TODO: split_keys[0].clone() why clone??
                    match map.entry(split_keys[0].clone()) {
                        Entry::Occupied(mut occupied) => {
                            occupied.get_mut().as_object_mut().unwrap().insert(
                                split_keys[1]
                                    .to_owned(),
                                Value::String(
                                    value.to_owned(),
                                ),
                            );
                            ()
                        }
                        Entry::Vacant(vacant) => {
                            let mut internal = Map::new();
                            internal.insert(
                                split_keys[1].to_owned(),
                                Value::String(value.to_owned()),
                            );
                            vacant.insert(Value::Object(internal));
                            ()
                        }
                    }
                }
                n if n >= 3 => {
                    // TODO: split_keys[0].clone() why clone??
                    match map.entry(split_keys[0].clone()) {
                        Entry::Occupied(mut occupied) => {
                            match occupied.get_mut().as_object_mut().unwrap().entry(
                                split_keys
                                    [1..n - 1]
                                    .join("."),
                            ) {
                                Entry::Occupied(mut occupied2) => {
                                    occupied2.get_mut().as_object_mut().unwrap().insert(
                                        split_keys[n - 1]
                                            .to_owned(),
                                        Value::String(
                                            value.to_owned(),
                                        ),
                                    );
                                    ()
                                }
                                Entry::Vacant(vacant2) => {
                                    let mut internal = Map::new();
                                    internal.insert(
                                        split_keys[n - 1].to_owned(),
                                        Value::String(value.to_owned()),
                                    );
                                    vacant2.insert(Value::Object(internal));
                                    ()
                                }
                            }
                        }
                        Entry::Vacant(vacant) => {
                            let mut internal = Map::new();
                            internal.insert(
                                split_keys[n - 1].to_owned(),
                                Value::String(value.to_owned()),
                            );
                            let mut external = Map::new();
                            external.insert(
                                split_keys[1..n - 1].join("."),
                                Value::Object(internal),
                            );
                            vacant.insert(Value::Object(external));
                            ()
                        }
                    }
                }
                _ => return Err(From::from("unexpected something happens.".to_owned())),
            }
        }

        Ok(Value::Object(map))
    }
}

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, "\n");
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}
