use std::collections::HashMap;

mod parse;

#[allow(dead_code)]
enum Value {
    Null,
    Chunks(Vec<Chunk<String, DataType>>),
}

#[allow(dead_code)]
enum DataType {
    Null,
    Bool(bool),
    Number(Number), // TODO
    String(String),
    Array(Vec<Value>),
    Chunk(Chunk<String, DataType>), // TODO
}

struct Number {} // TODO
type Chunk<T, U> = HashMap<T, U>;
