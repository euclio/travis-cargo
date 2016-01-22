use std::path::Path;
use std::process::Command;
use std::str;

use rustc_serialize::json::Json;

#[derive(Debug, Clone)]
pub struct Target(Json);

impl Target {
    pub fn binary_name(&self) -> Option<String> {
        self.0
            .find("name")
            .and_then(Json::as_string)
            .map(|name| name.replace("-", "_"))
            .and_then(|name| {
                self.0
                    .find_path(&["metadata", "extra_filename"])
                    .and_then(Json::as_string)
                    .map(|file_name| name + file_name)
            })
    }
}

#[derive(Debug)]
pub struct Manifest(Json);

impl Manifest {
    pub fn new<P>(dir: P, version: &str) -> Self
        where P: AsRef<Path>
    {
        let path_file = dir.as_ref().join("Cargo.toml");
        let path_dir = dir;

        let output = Command::new("cargo")
                         .args(&["read-manifest", "--manifest-path", path_file.to_str().unwrap()])
                         .output()
                         .unwrap_or_else(|e| {
                             Command::new("cargo")
                                 .args(&["read-manifest",
                                         "--manifest-path",
                                         path_dir.as_ref().to_str().unwrap()])
                                 .output()
                                 .unwrap_or_else(|e2| {
                                     panic!("{:?}, {:?}", e, e2);
                                 })
                         });

        Manifest(Json::from_str(&str::from_utf8(&output.stdout).unwrap()).unwrap())
    }

    pub fn targets(&self) -> Option<Vec<Target>> {
        let target_json = self.0
                              .find("targets")
                              .and_then(Json::as_array)
                              .unwrap();

        Some(target_json.iter()
                        .map(|target_json| Target(target_json.to_owned()))
                        .collect())
    }

    pub fn lib_name(&self) -> Option<String> {
        println!("{:?}", self.targets());
        None
    }
}
