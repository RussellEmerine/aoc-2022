mod day1;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Stdin, Stdout, Write};

struct Io<Input, Output>
where
    Input: Read,
    Output: Write,
{
    input: BufReader<Input>,
    output: BufWriter<Output>,
    buffer: Vec<String>,
}

#[allow(dead_code)]
impl<Input, Output> Io<Input, Output>
where
    Input: Read,
    Output: Write,
{
    fn new(input: Input, output: Output) -> Self {
        Self {
            input: BufReader::new(input),
            output: BufWriter::new(output),
            buffer: vec![],
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.input.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn print(&mut self, value: impl Display) {
        writeln!(self, "{}", value).ok();
    }

    fn prints(&mut self, items: &[impl Display], sep: &str) {
        writeln!(
            self,
            "{}",
            items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(sep)
        )
        .ok();
    }
}

impl<Input, Output> Write for Io<Input, Output>
where
    Input: Read,
    Output: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.output.flush()
    }
}

impl Default for Io<Stdin, Stdout> {
    fn default() -> Self {
        Self::new(stdin(), stdout())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        println!("day1");
        let mut s = String::new();
        File::open("input/day1/input.txt")?.read_to_string(&mut s)?;
        let calories = s
            .split("\n\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|a| a.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        println!("{}", day1::solve(calories.clone()));
        println!("{}", day1::more(calories));
    }

    Ok(())
}
