use crate::Oopsie;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum CdArg {
    In(String),
    Out,
    Root,
}

impl FromStr for CdArg {
    type Err = Oopsie; // got lazy lmao

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "/" => CdArg::Root,
            ".." => CdArg::Out,
            s => CdArg::In(s.to_string()),
        })
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Command {
    Cd(CdArg),
    Ls(Vec<Result<String, (usize, String)>>),
}

impl FromStr for Command {
    type Err = Oopsie;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.trim().split_whitespace().collect();
        let (&cmd, v) = v.split_first().ok_or(Oopsie)?;
        match cmd {
            "cd" => {
                if v.len() != 1 {
                    Err(Oopsie)
                } else {
                    Ok(Command::Cd(v[0].parse()?))
                }
            }
            "ls" => Ok(Command::Ls(
                v.chunks_exact(2)
                    .map(|a| {
                        if a[0] == "dir" {
                            Ok(Ok(a[1].to_string()))
                        } else {
                            Ok(Err((a[0].parse().map_err(|_| Oopsie)?, a[1].to_string())))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            _ => Err(Oopsie),
        }
    }
}

impl Command {
    fn parse_file(s: &str) -> Option<Vec<Command>> {
        s.strip_prefix("$")?
            .trim()
            .split("$")
            .map(|a| a.parse().ok())
            .collect()
    }
}

#[derive(Clone, Debug, Default)]
struct Files {
    files: HashMap<String, Result<Files, usize>>,
    is_root: bool,
}

impl Files {
    fn root() -> Self {
        Self {
            files: HashMap::new(),
            is_root: true,
        }
    }

    // returns true if `cd /` command hit
    fn browse_step(&mut self, cmds: &mut &[Command]) -> Option<bool> {
        if let Some((cmd, next)) = cmds.split_first() {
            *cmds = next;
            match cmd {
                Command::Cd(CdArg::In(f)) => {
                    if let Some(Ok(sub)) = self.files.get_mut(f) {
                        if sub.browse_step(cmds)? && !self.is_root {
                            Some(true)
                        } else {
                            self.browse_step(cmds)
                        }
                    } else {
                        None
                    }
                }
                Command::Cd(CdArg::Out) => {
                    if self.is_root {
                        None
                    } else {
                        Some(false)
                    }
                }
                Command::Cd(CdArg::Root) => {
                    if self.is_root {
                        self.browse_step(cmds)
                    } else {
                        Some(true)
                    }
                }
                Command::Ls(f) => {
                    if self.files.is_empty() {
                        self.files = f
                            .iter()
                            .map(|a| match a {
                                Ok(dir) => (dir.to_string(), Ok(Files::default())),
                                Err((size, file)) => (file.to_string(), Err(*size)),
                            })
                            .collect();
                    }
                    self.browse_step(cmds)
                }
            }
        } else {
            Some(false)
        }
    }

    fn browse(&mut self, mut cmd: &[Command]) -> Option<()> {
        self.browse_step(&mut cmd).map(|_| ())
    }
}

#[derive(Clone, Debug)]
struct FileSize {
    size: usize,
    sub: Vec<FileSize>,
}

impl FileSize {
    fn add_sizes(f: &Files) -> Self {
        let sub: Vec<_> = f
            .files
            .values()
            .filter_map(|a| a.as_ref().ok())
            .map(|a| FileSize::add_sizes(a))
            .collect();
        let file_sum: usize = f
            .files
            .values()
            .filter_map(|a| a.as_ref().err().copied())
            .sum();
        FileSize {
            size: file_sum + sub.iter().map(|a| a.size).sum::<usize>(),
            sub,
        }
    }

    fn solve(&self) -> usize {
        self.sub.iter().map(|a| a.solve()).sum::<usize>()
            + if self.size <= 100000 { self.size } else { 0 }
    }

    fn flatten(&self) -> Vec<usize> {
        std::iter::once(self.size)
            .chain(self.sub.iter().map(|a| a.flatten()).flatten())
            .collect()
    }
}

pub fn solve(s: &str) -> Option<usize> {
    let cmd = Command::parse_file(s)?;
    let mut files = Files::root();
    files.browse(&cmd)?;
    let file_size = FileSize::add_sizes(&files);
    Some(file_size.solve())
}

pub fn more(s: &str) -> Option<usize> {
    let cmd = Command::parse_file(s)?;
    let mut files = Files::root();
    files.browse(&cmd)?;
    let file_size = FileSize::add_sizes(&files);
    let total = file_size.size;
    let size = file_size
        .flatten()
        .iter()
        .filter(|&&a| 40000000 + a >= total)
        .min()
        .copied()?;
    Some(size)
}
