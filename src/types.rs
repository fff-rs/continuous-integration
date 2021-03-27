use super::*;

use std::cmp::Ordering;
use std::fmt;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
use std::ops::Add;
#[derive(Debug, Clone)]
pub enum TestEnvType {
    #[allow(dead_code)]
    Darwin(String),

    Linux(String),

    #[allow(dead_code)]
    Windows(String),

    Unknown,
}

impl TestEnvType {
    pub fn as_str(&self) -> &str {
        match *self {
            TestEnvType::Linux(ref x) => x.as_str(),
            TestEnvType::Windows(ref x) => x.as_str(),
            TestEnvType::Darwin(ref x) => x.as_str(),
            _ => "Unknown",
        }
    }
}

impl Ord for TestEnvType {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            TestEnvType::Linux(ref z) => match *other {
                TestEnvType::Linux(ref x) => z.cmp(x),
                TestEnvType::Darwin(_) => Ordering::Less,
                TestEnvType::Windows(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            TestEnvType::Darwin(ref z) => match *other {
                TestEnvType::Linux(_) => Ordering::Greater,
                TestEnvType::Darwin(ref x) => z.cmp(x),
                TestEnvType::Windows(_) => Ordering::Less,
                _ => Ordering::Equal,
            },
            TestEnvType::Windows(ref z) => match *other {
                TestEnvType::Linux(_) => Ordering::Greater,
                TestEnvType::Darwin(_) => Ordering::Greater,
                TestEnvType::Windows(ref x) => z.cmp(x),
                _ => Ordering::Less,
            },
            _ => match *other {
                TestEnvType::Linux(_) => Ordering::Less,
                TestEnvType::Windows(_) => Ordering::Less,
                TestEnvType::Darwin(_) => Ordering::Less,
                _ => Ordering::Equal,
            },
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
        f.write_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendExecute {
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
pub struct Backend {
    name: String,
    execute: BackendExecute,
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

impl Backend {
    pub fn new(s: &'static str, execute: BackendExecute) -> Self {
        Self {
            name: s.to_owned(),
            execute,
        }
    }

    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug)]
pub struct TestEnv {
    pub(crate) name: TestEnvType,
    pub(crate) backends: Vec<Backend>,
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
            backends,
        }
    }

    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}


impl fmt::Display for TestEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
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

#[derive(askama::Template)]
#[template(path = "juice.yml", escape = "none")]
pub struct JuiceYml<'a> {
    pub(crate) passive: bool, // false
    pub(crate) testenvs: &'a Vec<TestEnv>,
}

#[derive(askama::Template)]
#[template(path = "juice-containers.yml", escape = "none")]
pub struct ContainerYml<'a> {
    #[allow(dead_code)]
    pub(crate) passive: bool, // false
    pub(crate) testenvs: &'a Vec<TestEnv>,
}

#[derive(askama::Template)]
#[template(path = "juice-crashtest.yml", escape = "none")]
pub struct CrashTestYml<'a> {
    // only external events trigger this
    pub(crate) passive: bool, // true
    pub(crate) testenvs: &'a Vec<TestEnv>,
}
