use libactionkv::ActionKV;
use std::str::FromStr;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
	akv_mem.exe FILE get KEY
	akv_mem.exe FILE delete KEY
	akv_mem.exe FILE insert KEY VALUE
	akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
	akv_mem FILE get KEY
	akv_mem FILE delete KEY
	akv_mem FILE insert KEY VALUE
	akv_mem FILE update KEY VALUE
";

#[derive(Debug, PartialEq)]
enum Action {
    GET,
    DELETE,
    INSERT,
    UPDATE,
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(Action::GET),
            "delete" => Ok(Action::DELETE),
            "insert" => Ok(Action::INSERT),
            "update" => Ok(Action::UPDATE),
            _ => Err("Invalid Action"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE);
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("Unable to open the file");
    store.load().expect("Unable to load data");

    match Action::from_str(action).unwrap() {
        Action::GET => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        Action::DELETE => store.delete(key).unwrap(),
        Action::UPDATE => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }

        Action::INSERT => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
    }
}
