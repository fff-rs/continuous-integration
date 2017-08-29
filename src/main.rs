use std::fs::OpenOptions;
use std::io::prelude::*;

#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "concourse.yml")]
struct ConcourseYml<'a> {
    operatingsystems: &'a Vec<String>,
    backends: &'a Vec<String>
}


fn main() {
    // TODO read this from a `matrix.yml` file
    let operatingsystems = &vec!["fedora".to_string(),"ubuntu".to_string()];
    let backends = &vec!["default".to_string(),"native".to_string(),"cuda".to_string(),"opencl".to_string()];

    let config = ConcourseYml {
         operatingsystems: operatingsystems,
         backends: backends,
         };

    let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open("concourse.yml").unwrap();

    {
        println!("{}", config.render());
    }
    file.write_all(config.render().as_str().as_bytes()).unwrap();
}
