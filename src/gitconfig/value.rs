pub use gitconfig::map::Map;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Map(Map<String, Value>),
}
