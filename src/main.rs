use std::fs::OpenOptions;
use std::io::prelude::*;

#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "juice.yml")]
struct JuiceYml<'a> {
    operatingsystems: &'a Vec<String>,
    backends: &'a Vec<String>
}

#[derive(Template)]
#[template(path = "juice-containers.yml")]
struct ContainerYml<'a> {
    operatingsystems: &'a Vec<String>,
    backends: &'a Vec<String>
}


fn main() {
    // TODO read this from a `matrix.yml` file using yml-rust
    let operatingsystems = &vec!["fedora".to_string(),"ubuntu".to_string(), "pureos".to_string()];
    let backends = &vec!["default".to_string(),"native".to_string(),"cuda".to_string(),"opencl".to_string()];

    let juice = JuiceYml {
         operatingsystems: operatingsystems,
         backends: backends,
         };
    let containers = ContainerYml {
         operatingsystems: operatingsystems,
         backends: backends,
         };

    let mut f_juice = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open("juice.yml").unwrap();
    f_juice.write_all(juice.render().unwrap().as_str().as_bytes()).unwrap();
    let mut f_containers = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open("juice-containers.yml").unwrap();
    f_containers.write_all(containers.render().unwrap().as_str().as_bytes()).unwrap();
}
