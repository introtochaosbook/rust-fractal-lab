extern crate core;

use std::fmt::{Display, Formatter, Write};

use bitvec::vec::BitVec;
use clap::Parser;
use rand::Rng;
use regex::Regex;

const STAR_CHAR: char = '*';

#[derive(Parser)]
struct Args {
    input: String,
}

// A state is just a vector of bits (true or false, 1 or 0)
struct State(BitVec);

impl State {
    /// Parse state from input string containing stars, spaces, and/or repeat directives
    pub fn parse<T: AsRef<str>>(input: T) -> Self {
        let input = input.as_ref();
        let mut rng = rand::thread_rng();

        let regex = Regex::new(r"(?P<count>\d+)\[(?P<char>[ *?])]|(?P<single>[ *?])").unwrap();

        let mut ret = BitVec::new();
        let mut start = 0;

        for m in regex.captures_iter(input) {
            if let Some(count) = m.name("count") {
                if count.start() != start {
                    eprintln!("{} != {}", count.start(), start);
                    panic!("unparsed input at pos {}: '{}'", start, &input[start..count.start()]);
                }

                let repetition_count: usize = count.as_str().parse().unwrap();
                let char = m.name("char").unwrap();
                // Account for closing square bracket
                start = char.end() + 1;
                let char = char.as_str();
                assert_eq!(char.len(), 1);
                let repetition_char = char.chars().nth(0).unwrap();
                let repeater = || match repetition_char {
                    STAR_CHAR => true,
                    ' ' => false,
                    '?' => rng.gen_bool(0.5),
                    c => panic!("unexpected char '{}' in repetition '{}[{}]'", c, repetition_count, c),
                };
                ret.extend(std::iter::repeat_with(repeater).take(repetition_count));
            } else if let Some(single) = m.name("single") {
                if single.start() != start {
                    panic!("unparsed input at pos {}: '{}'", start, &input[start..single.start()]);
                }

                start = single.end();
                let c = single.as_str().chars().nth(0).unwrap();
                match c {
                    STAR_CHAR => ret.push(true),
                    ' ' => ret.push(false),
                    '?' => ret.push(rng.gen_bool(0.5)),
                    c => panic!("unexpected char: '{}'", c),
                }
            } else {
                unreachable!();
            }
        }

        if start != input.len() {
            panic!("unparsed input at pos {}: '{}'", start, &input[start..]);
        }

        Self(ret)
    }

    //
    fn next(&self) -> State {
        let mut ret = BitVec::repeat(false, self.0.len());

        for (i, slice) in self.0.windows(3).enumerate() {
            let a_neg1 = slice[0];
            let a_0 = slice[1];
            let a_1 = slice[2];

            ret.set(
                i + 1,
                (a_neg1 && !a_0 && !a_1) || (!a_neg1 && a_1) || (a_0 && a_1),
            );
        }

        State(ret)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            if *c.as_ref() {
                f.write_char(STAR_CHAR)?;
            } else {
                f.write_char(' ')?;
            }
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let mut input = State::parse(args.input);
    eprintln!("{}", input);
    for _ in 0..25 {
        input = input.next();
        eprintln!("{}", input);
    }
}

#[cfg(test)]
mod test {
    use crate::State;
    use bitvec::bitvec;
    use bitvec::prelude::Lsb0;

    #[test]
    fn parse() {
        let input = "     ";
        let state = State::parse(input);
        assert_eq!(state.0, bitvec![0, 0, 0, 0, 0]);
        assert_eq!(format!("{}", state), String::from("     "));
    }
}
