use std::{collections::BTreeMap, env, fs, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space0, space1},
    combinator::map,
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, InputTakeAtPosition,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    ChangeDirectoryToRoot,
    ChangeDirectoryUpOne,
    ChangeDirectoryInto(Rc<str>),
    List,
}

#[derive(Debug, Clone)]
enum DirEntry {
    File(Rc<str>, u64),
    Directory(Rc<str>),
}

#[derive(Debug, Clone)]
struct CommandAndOutput {
    cmd: Command,
    output: Vec<DirEntry>,
}

fn cd_command(input: &str) -> IResult<&str, CommandAndOutput> {
    delimited(
        terminated(tag("cd"), space1),
        map(
            alt((
                map(tag("/"), |_| Command::ChangeDirectoryToRoot),
                map(tag(".."), |_| Command::ChangeDirectoryUpOne),
                map(alphanumeric1, |name: &str| {
                    Command::ChangeDirectoryInto(Rc::from(name))
                }),
            )),
            |cmd| CommandAndOutput {
                cmd,
                output: vec![],
            },
        ),
        terminated(space0, newline),
    )(input)
}

fn filename(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(
        |item| !(item.is_alphanumeric() || item == '.' || item == '_'),
        ErrorKind::AlphaNumeric,
    )
}

fn dir_entry(input: &str) -> IResult<&str, DirEntry> {
    terminated(
        alt((
            preceded(
                terminated(tag("dir"), space1),
                map(filename, |name: &str| DirEntry::Directory(Rc::from(name))),
            ),
            map(
                separated_pair(
                    nom::character::complete::u64::<&str, nom::error::Error<&str>>,
                    space1,
                    filename,
                ),
                |(size, name)| DirEntry::File(Rc::from(name), size),
            ),
        )),
        terminated(space0, newline),
    )(input)
}

fn ls_command(input: &str) -> IResult<&str, CommandAndOutput> {
    preceded(
        terminated(tag("ls"), terminated(space0, newline)),
        map(many0(dir_entry), |output| CommandAndOutput {
            cmd: Command::List,
            output,
        }),
    )(input)
}

fn executed_command(input: &str) -> IResult<&str, CommandAndOutput> {
    preceded(terminated(tag("$"), space1), alt((cd_command, ls_command)))(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reversed_args: Vec<_> = args.iter().map(|x| x.as_str()).rev().collect();

    reversed_args
        .pop()
        .expect("Expected the executable name to be the first argument, but was missing");

    let part = reversed_args.pop().expect("part number");
    let input_file = reversed_args.pop().expect("input file");
    let content = fs::read_to_string(input_file).unwrap();

    let (leftovers, input_data) =
        many1(executed_command)(content.as_str()).expect("failed to parse");
    assert!(leftovers.is_empty(), "{leftovers}");

    match part {
        "1" => {
            let result = solve_part1(&input_data);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(&input_data);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Path(Vec<Rc<str>>);

impl Path {
    fn dir(&self, dir_name: &Rc<str>) -> Self {
        let mut components = self.0.clone();
        components.push(dir_name.clone());
        Self(components)
    }
}

fn construct_filesystem(data: &[CommandAndOutput]) -> BTreeMap<Path, Vec<DirEntry>> {
    let root_dir: Rc<str> = Rc::from("/");
    let mut fs: BTreeMap<Path, Vec<DirEntry>> = Default::default();

    assert_eq!(
        data.first().expect("empty data").cmd,
        Command::ChangeDirectoryToRoot
    );
    let mut current_path = vec![root_dir.clone()];

    for cmd_and_output in data {
        match &cmd_and_output.cmd {
            Command::ChangeDirectoryToRoot => current_path = vec![root_dir.clone()],
            Command::ChangeDirectoryUpOne => {
                current_path.pop();
            }
            Command::ChangeDirectoryInto(dir) => current_path.push(dir.clone()),
            Command::List => {
                fs.insert(Path(current_path.clone()), cmd_and_output.output.clone());
            }
        }
    }

    fs
}

fn compute_directory_size(
    path: &Path,
    fs: &BTreeMap<Path, Vec<DirEntry>>,
    size_cache: &mut BTreeMap<Path, u64>,
) -> u64 {
    if let Some(size) = size_cache.get(path) {
        return *size;
    }

    let mut size = 0;
    if let Some(entries) = fs.get(path) {
        for entry in entries {
            match entry {
                DirEntry::File(_, file_size) => size += *file_size,
                DirEntry::Directory(dir_name) => {
                    size += compute_directory_size(&path.dir(dir_name), fs, size_cache);
                }
            }
        }
    }

    size_cache.insert(path.clone(), size);

    size
}

fn solve_part1(data: &[CommandAndOutput]) -> u64 {
    let fs = construct_filesystem(data);
    let mut size_cache = Default::default();
    let fs_root = Path(vec![Rc::from("/")]);

    compute_directory_size(&fs_root, &fs, &mut size_cache);

    let max_size = 100000;
    size_cache
        .values()
        .filter_map(|size| (size <= &max_size).then_some(*size))
        .sum()
}

fn solve_part2(data: &[CommandAndOutput]) -> u64 {
    let fs = construct_filesystem(data);
    let mut size_cache = Default::default();
    let fs_root = Path(vec![Rc::from("/")]);

    compute_directory_size(&fs_root, &fs, &mut size_cache);

    let required_free_space = 30000000;

    let full_space = 70000000;
    let total_size = size_cache[&fs_root] as i64;
    let free_space = full_space - total_size;
    assert!(free_space >= 0, "the total size of the filesystem {total_size} is larger than the available capacity {full_space}");

    let size_to_free_up = required_free_space - free_space;
    assert!(
        size_to_free_up >= 0,
        "sufficient free space is already available"
    );

    size_cache
        .values()
        .filter_map(|size| (size >= &(size_to_free_up as u64)).then_some(*size))
        .min()
        .expect("no directory is big enough to free up the needed space")
}
