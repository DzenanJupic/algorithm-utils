use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Formatter;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use crate::algorithms::load::Algorithm;
use crate::error::GeneralError;


#[cfg(target_os = "windows")]
const DYL_EXTENSION: &str = "dll";
#[cfg(any(target_os = "macos", target_os = "linux"))]
const DYL_EXTENSION: &str = "so";

#[derive(Default)]
pub struct Algorithms {
    algorithms: HashMap<&'static str, Algorithm>,
}

impl Algorithms {
    /// creates an empty instance of `Algorithms`
    pub fn empty() -> Self {
        Self {
            algorithms: HashMap::new()
        }
    }

    /// loads all algorithms of a directory
    ///
    /// This method provides a convenient way to search for all dynamically loaded library's in a
    /// directory. It searches for these library's in a specific pattern:
    ///     1. Dynamically loaded library's directly places inside the directory (./)
    ///     2. Dynamically loaded library's directly places inside a separate directory (./<library-dir>/)
    ///     3. Dynamically loaded library's in the release directory of a crate (./<crate>/target/release/)
    ///     4. Dynamically loaded library's in the debug directory of a crate (./<crate>/target/debug/)
    ///
    /// Duplicated algorithm names will lead to an error
    /// If a file is no dynamic library or a folder does not contains a dynamic library
    /// a warning message will be printed
    pub fn load_all<P: AsRef<Path>>(&mut self, path: &P) -> Result<(), GeneralError> {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(library) = find_dynamic_library_in_dir(&path) {
                    self.load(&library)?;
                    continue;
                }
                if let Some(library) = find_dynamic_library_in_crate(&path) {
                    self.load(&library)?;
                    continue;
                }
                println!("This directory does not contain any algorithms: {:?}", path);
            } else if path.is_file() {
                match path.extension() {
                    Some(extension) if extension == DYL_EXTENSION => self.load(&path)?,
                    _ => println!("This file is no algorithms: {:?}", path)
                }
            } else { unreachable!("Path is neither directory nor file") }
        }

        Ok(())
    }

    /// loads an algorithm by path
    /// for more information have a look at `Algorithm::load`
    pub fn load<P: AsRef<OsStr>>(&mut self, path: &P) -> Result<(), GeneralError> {
        let algorithm = Algorithm::load(path)?;

        if self.algorithms.contains_key(algorithm.name()) {
            println!(
                "An algorithm with the name `{}` already exists\n\
                consider removing one of both",
                algorithm.name()
            );
            return Err(GeneralError::LibLoading);
        }

        self.algorithms.insert(
            algorithm.name(),
            algorithm,
        );

        Ok(())
    }

    /// returns a reference to a `Algorithm`
    /// can be used to trade using this algorithm
    pub fn get(&self, algorithm: &str) -> Option<&Algorithm> {
        self.algorithms.get(algorithm)
    }

    /// returns a mutable reference to a `Algorithm`
    /// can be used to trade using this algorithm
    pub fn get_mut(&mut self, algorithm: &str) -> Option<&mut Algorithm> {
        self.algorithms.get_mut(algorithm)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.algorithms.contains_key(name)
    }
}

impl fmt::Display for Algorithms {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let algorithms = if self.algorithms.len() > 0 {
            self.algorithms
                .iter()
                .fold(
                    String::from("\t"),
                    |mut prev, (name, _)| {
                        prev.push_str("\n\t");
                        prev.push_str(name);
                        prev
                    },
                )
        } else { String::from("None") };

        write!(formatter, "ALGORITHMS:{}", algorithms)
    }
}

/// looks for dynamically loadable library's in a directory
///
/// Iterates over each element in a directory and returns the first dynamical library
/// it can find.
pub fn find_dynamic_library_in_dir<P: AsRef<Path>>(path: &P) -> Option<PathBuf> {
    if let Ok(dir) = read_dir(path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == DYL_EXTENSION {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

/// looks for dynamically loadable library's in a rust crate
///
/// First looks for a library in the release folder, then in the debug folder.
pub fn find_dynamic_library_in_crate<P: AsRef<Path>>(path: &P) -> Option<PathBuf> {
    if path.as_ref().is_dir() {
        if let Some(library) = find_dynamic_library_in_dir(&path.as_ref().join("target/release")) {
            return Some(library);
        }
        if let Some(library) = find_dynamic_library_in_dir(&path.as_ref().join("target/debug")) {
            return Some(library);
        }
    }

    None
}
