use anyhow::anyhow;

use askama::Template;

use fs::OpenOptions;
use fs_err as fs;
use std::env;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

mod types;
use self::types::*;

// /// Legacy approach to obtain multiple `pkg.sh` variants depending on the backend.
// fn get_pkg_helper_regex(text: &String) -> Option<String> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^pkg\.sh\.(.*)$").unwrap();
//     }
//     for cap in RE.captures_iter(text) {
//         return Some(cap[1].to_string());
//     }
//     return None;
// }

/// Checks if there is a `BUILD_ONLY` file in the directory to determine
/// if this backend should only be built or built and run (default).
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

/// Read all sub dirs from a base dir.
///
/// Each dir that matches a backend is treated as such.
fn get_backends(directory: &Path) -> Result<Vec<Backend>> {
    Ok(fs::read_dir(directory)?
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let pathbuf: PathBuf = entry.path().into();
            pathbuf
        })
        .filter(|entry| entry.is_dir())
        .filter_map(|entry| {
            if let Some(name) = entry.file_name() {
                let name = name.to_str().expect("invalid unicode");
                get_backend_execute_type(&entry)
                    .ok()
                    .and_then(|xt| Some((name, xt).into()))
                    .or(None)
            } else {
                None
            }
        })
        .collect())
}

fn run() -> Result<()> {
    let cwd: PathBuf = env::current_dir().unwrap_or(
        PathBuf::from("HOME")
            .join("spearow")
            .join("continuous-integration"),
    );

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
        .filter(|entry| entry.file_name().is_some())
        .filter_map(|entry| {
            if let Some(filename) = entry.file_name() {
                if let Ok(bb) = get_backends(&entry) {
                    return Some((filename.to_owned(), bb));
                }
            }
            None
        })
        .filter(|(_, backends)| !backends.is_empty())
        .map(|(filename, mut backends)| {
            backends.sort();
            let filename = String::from(filename.to_string_lossy());
            TestEnv::new(filename, backends)
            // TestEnv::new(filename, vec![Backend::new("cuda", BackendExecute::Test)])
        })
        .collect();

    testenvs.sort();

    for testenv in &testenvs {
        println!("test envs: {:?}", testenv);
    }

    let juice = JuiceYml {
        testenvs: &testenvs,
        passive: false,
    };
    let containers = ContainerYml {
        testenvs: &testenvs,
        passive: false,
    };
    let crashtest = CrashTestYml {
        testenvs: &testenvs,
        passive: true,
    };

    fn dump<T: askama::Template, P: AsRef<Path>>(template: &T, dest: P) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(dest.as_ref())?;
        let content = template
            .render()
            .map_err(|e| anyhow!("Failed to render template").context(e))?;
        file.write_all(content.as_str().as_bytes())?;
        Ok(())
    }

    dump(&juice, "juice.yml")?;
    dump(&containers, "juice-containers.yml")?;
    dump(&crashtest, "juice-crashtest.yml")?;
    println!("=== YAML Generation Complete ===");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}
