use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::path::Path;
use std::path::PathBuf;

use std::env;
use std::fmt;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate askama;

use askama::Template;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;


use std::cmp::Ordering;


error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];

    }
}

// fn get_pkg_helper_regex(text: &String) -> Option<String> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^pkg\.sh\.(.*)$").unwrap();
//     }
//     for cap in RE.captures_iter(text) {
//         return Some(cap[1].to_string());
//     }
//     return None;
// }

fn get_backend_execute_type(directory: &Path) -> Result<BackendExecute> {
    let entries = fs::read_dir(directory)?;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(x) = path.file_name() {
                if x == "BUILD_ONLY" {
                    return Ok(BackendExecute::Build);
                }
            }
        }
    }
    Ok(BackendExecute::Test)
}

fn get_backends(directory: &Path) -> Result<Vec<Backend>> {
    Ok(
        fs::read_dir(directory)?
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let pathbuf: PathBuf = entry.path().into();
                pathbuf
            })
            .filter(|entry| entry.is_dir())
            .filter_map(|entry| if let Some(name) = entry.file_name() {
                let name = name.to_str().expect("invalid unicode");
                get_backend_execute_type(&entry)
                    .ok()
                    .and_then(|xt| Some((name, xt).into()))
                    .or(None)
            } else {
                None
            })
            .collect(),
    )
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
                    _ => Ordering::Greater,
                    TestEnvType::Linux(ref x) => z.cmp(x),
                    TestEnvType::Darwin(_) => Ordering::Less,
                    TestEnvType::Windows(_) => Ordering::Less,
                }
            }
            TestEnvType::Darwin(ref z) => {
                match *other {
                    _ => Ordering::Equal,
                    TestEnvType::Linux(_) => Ordering::Greater,
                    TestEnvType::Darwin(ref x) => z.cmp(x),
                    TestEnvType::Windows(_) => Ordering::Less,
                }
            }
            TestEnvType::Windows(ref z) => {
                match *other {
                    _ => Ordering::Less,
                    TestEnvType::Linux(_) => Ordering::Greater,
                    TestEnvType::Darwin(_) => Ordering::Greater,
                    TestEnvType::Windows(ref x) => z.cmp(x),
                }
            }
            _ => {
                match *other {
                    _ => Ordering::Equal,
                    TestEnvType::Linux(_) => Ordering::Less,
                    TestEnvType::Windows(_) => Ordering::Less,
                    TestEnvType::Darwin(_) => Ordering::Less,
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
            _ => &y,
        };
        write!(f, "{}", echo)
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BackendExecute {
    Build,
    Test,
}

impl fmt::Display for BackendExecute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let echo = match *self {
            BackendExecute::Build => "build",
            BackendExecute::Test => "test",
        };
        write!(f, "{}", echo)
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Backend {
    name: String,
    execute: BackendExecute,
}

impl Backend {
    /// required since askama has no idea how to compare enums I guess
    pub fn is_build_only(&self) -> bool {
        self.execute == BackendExecute::Build
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl<T> From<(T, BackendExecute)> for Backend
where
    T: Into<String>,
{
    fn from(tuple: (T, BackendExecute)) -> Self {
        Self {
            name: tuple.0.into(),
            execute: tuple.1,
        }
    }
}

#[derive(Debug)]
struct TestEnv {
    name: TestEnvType,
    backends: Vec<Backend>,
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
    pub fn new(name: String, backends: Vec<Backend>) -> Self {
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



fn run() -> Result<()> {
    let cwd: PathBuf = env::current_dir().unwrap_or(PathBuf::from("HOME").join("spearow").join(
        "continuous-integration",
    ));

    let base: PathBuf = env::var("JUICE_CONTAINERS")
        .unwrap_or(cwd.join("container").to_string_lossy().to_string())
        .into();

    println!("Using {} as base container dir", base.display());



    let mut testenvs: Vec<TestEnv> = fs::read_dir(base)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            let pathbuf: PathBuf = entry.path().into();
            pathbuf
        })
        .filter_map(|entry| if let Some(x) = entry.file_name() {

            if let Ok(mut backends) = get_backends(&entry) {
                backends.sort();
                Some(TestEnv::new(String::from(x.to_string_lossy()), backends))
            } else {
                None
            }
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
        .open("juice.yml")?;
    f_juice.write_all(
        juice.render().unwrap().as_str().as_bytes(),
    )?;

    let mut f_containers = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("juice-containers.yml")?;
    f_containers.write_all(
        containers.render().unwrap().as_str().as_bytes(),
    )?;
    Ok(())
}


fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError;

        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}
