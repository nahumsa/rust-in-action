use std::{collections::HashMap, fs::File};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub Value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    pub file: File,
    pub index: HashMap<ByteString, u64>,
}
