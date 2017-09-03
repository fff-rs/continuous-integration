use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::path::Path;
use std::path::PathBuf;

use std::env;
use std::fmt;

#[macro_use]
extern crate askama;

use askama::Template;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;


use std::cmp::Ordering;




fn get_pkg_helper_regex(text: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^pkg\.sh\.(.*)$").unwrap();
    }
    for cap in RE.captures_iter(text) {
        return Some(cap[1].to_string());
    }
    return None;
}


fn get_backends(directory: &Path) -> Vec<String> {
    fs::read_dir(directory)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let pathbuf: PathBuf = entry.path().into();
            pathbuf
        })
        .filter(|entry| entry.is_file())
        .filter_map(|entry| if let Some(x) = entry.file_name() {
            Some(String::from(x.to_string_lossy()))
        } else {
            None
        })
        .filter_map(|entry| get_pkg_helper_regex(&entry))
        .collect()
}



#[derive(Debug)]
enum TestEnvType {
    Darwin(String),
    Linux(String),
    Windows(String),
    Unknown,
}

impl Ord for TestEnvType {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            TestEnvType::Linux(ref z) => {
                match *other {
                    _ | TestEnvType::Unknown => Ordering::Greater,
                    TestEnvType::Linux(ref x) => z.cmp(x),
                    TestEnvType::Darwin(ref x) => Ordering::Less,
                    TestEnvType::Windows(ref x) => Ordering::Less,
                }
            }
            TestEnvType::Darwin(ref z) => {
                match *other {
                    _ | TestEnvType::Unknown => Ordering::Equal,
                    TestEnvType::Linux(ref x) => Ordering::Greater,
                    TestEnvType::Darwin(ref x) => z.cmp(x),
                    TestEnvType::Windows(ref x) => Ordering::Less,
                }
            }
            TestEnvType::Windows(ref z) => {
                match *other {
                    _ | TestEnvType::Unknown => Ordering::Less,
                    TestEnvType::Linux(ref x) => Ordering::Greater,
                    TestEnvType::Darwin(ref x) => Ordering::Greater,
                    TestEnvType::Windows(ref x) => z.cmp(x),
                }
            }
            _ | TestEnvType::Unknown => {
                match *other {
                    _ | TestEnvType::Unknown => Ordering::Equal,
                    TestEnvType::Linux(ref x) => Ordering::Less,
                    TestEnvType::Windows(ref x) => Ordering::Less,
                    TestEnvType::Darwin(ref x) => Ordering::Less,
                }
            }
        }
    }
}

impl PartialOrd for TestEnvType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TestEnvType {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for TestEnvType {}

impl fmt::Display for TestEnvType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let y = String::from("Unknown");
        let echo = match *self {
            TestEnvType::Linux(ref x) => x,
            TestEnvType::Windows(ref x) => x,
            TestEnvType::Darwin(ref x) => x,
            _ | TestEnvType::Unknown => &y,
        };
        write!(f, "{}", echo)
    }
}

#[derive(Debug)]
struct TestEnv {
    name: TestEnvType,
    backends: Vec<String>,
}



impl Ord for TestEnv {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for TestEnv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PartialEq for TestEnv {
    fn eq(&self, other: &Self) -> bool {
        self.name.cmp(&other.name) == Ordering::Equal
    }
}


impl Eq for TestEnv {}

impl TestEnv {
    /// TODO currently only Linux envs are supported by this idiotic implementation
    pub fn new(name: String, backends: Vec<String>) -> Self {
        Self {
            name: TestEnvType::Linux(name),
            backends: backends,
        }
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self {
            name: TestEnvType::Unknown,
            backends: vec![],
        }
    }
}


#[derive(Template)]
#[template(path = "juice.yml")]
struct JuiceYml<'a> {
    testenvs: &'a Vec<TestEnv>,
}

#[derive(Template)]
#[template(path = "juice-containers.yml")]
struct ContainerYml<'a> {
    testenvs: &'a Vec<TestEnv>,
}



fn main() {
    let cwd: PathBuf = env::current_dir().unwrap_or(PathBuf::from("HOME").join("spearow").join(
        "continuous-integration",
    ));

    let base: PathBuf = env::var("JUICE_CONTAINERS")
        .unwrap_or(cwd.join("container").to_string_lossy().to_string())
        .into();

    println!("Using {} as base container dir", base.display());



    let mut testenvs: Vec<TestEnv> = fs::read_dir(base)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            let pathbuf: PathBuf = entry.path().into();
            pathbuf
        })
        .filter_map(|entry| if let Some(x) = entry.file_name() {

            let mut backends = get_backends(&entry);
            backends.sort();
            Some(TestEnv::new(String::from(x.to_string_lossy()), backends))
        } else {
            None
        })
        .collect();

    testenvs.sort();

    for testenv in &testenvs {
        println!("test envs: {:?}", testenv);
    }

    let juice = JuiceYml { testenvs: &testenvs };
    let containers = ContainerYml { testenvs: &testenvs };

    let mut f_juice = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("juice.yml")
        .unwrap();
    f_juice
        .write_all(juice.render().unwrap().as_str().as_bytes())
        .unwrap();

    let mut f_containers = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("juice-containers.yml")
        .unwrap();
    f_containers
        .write_all(containers.render().unwrap().as_str().as_bytes())
        .unwrap();
}
