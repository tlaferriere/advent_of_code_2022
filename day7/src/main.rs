use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::hash::Hash;

use derive_new::new;
use std::io::{BufReader, Read};
use std::path::PathBuf;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "fs.pest"]
pub struct FSParser;

#[derive(new)]
struct Dir {
    path: PathBuf,
    #[new(default)]
    dirs: Vec<String>,
    #[new(default)]
    files: Vec<u64>,
}

impl Dir {
    fn size(&self, fs: &HashMap<PathBuf, Self>, cache: &mut HashMap<PathBuf, u64>) -> u64 {
        if let Some(s) = cache.get(&self.path) {
            return *s;
        }
        let i = self
            .dirs
            .iter()
            .fold(self.files.iter().sum::<u64>(), |acc, dir| {
                let mut dir_path = self.path.clone();
                dir_path.push(dir);
                acc + fs
                    .get(&dir_path)
                    .expect(format!("Path not found: {}", dir_path.to_str().unwrap()).as_str())
                    .size(fs, cache)
            });
        cache.insert(self.path.clone(), i);
        i
    }
}

enum Cd {
    Up,
    Down(String),
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;

    let fs = parse_fs_navigation(f)?;

    // Find dirs of at most 100000 size
    const FS_SIZE: u64 = 70000000;
    const REQUIRED_FREE_SPACE: u64 = 30000000;

    let mut size_cache = HashMap::new();
    let root = PathBuf::from("/");
    let mut total_size = fs.get(&root).unwrap().size(&fs, &mut size_cache);
    let space_to_free = REQUIRED_FREE_SPACE - (FS_SIZE - total_size);

    let mut eligible_dirs: Vec<_> = size_cache
        .iter()
        .filter(|(_, size)| **size >= space_to_free)
        .collect();
    eligible_dirs.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    println!("{}", eligible_dirs.first().unwrap().1);
    Ok(())
}

fn parse_fs_navigation(f: File) -> std::io::Result<HashMap<PathBuf, Dir>> {
    let mut reader = BufReader::new(f);
    let mut unparsed = String::new();
    reader.read_to_string(&mut unparsed)?;
    let mut parsed = FSParser::parse(Rule::file, unparsed.as_str()).unwrap();
    // println!("{parsed:?}");

    // Build up the fs representation
    let mut fs: HashMap<PathBuf, Dir> = HashMap::new();
    let mut current_path = PathBuf::new();
    for pair in parsed {
        match pair.as_rule() {
            Rule::nav => {
                for name in pair.into_inner() {
                    match name.as_str() {
                        ".." => {
                            current_path.pop();
                        }
                        s => {
                            current_path.push(s);
                        }
                    }
                }
            }
            Rule::ls => {
                // Navigate to the target dir
                let mut current_dir = fs
                    .entry(current_path.clone())
                    .or_insert(Dir::new(current_path.clone()));
                for output in pair.into_inner() {
                    match output.as_rule() {
                        Rule::dir_name => current_dir.dirs.push(output.as_str().to_string()),
                        Rule::file_node => current_dir.files.push(
                            output
                                .into_inner()
                                .next() // extract the file_size token (ignoring the name)
                                .unwrap()
                                .as_str()
                                .parse()
                                .unwrap(),
                        ),
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => {
                println!("Reached end of input.")
            }
            _ => {
                unreachable!()
            }
        }
    }
    Ok(fs)
}
