use std::fmt::{Display, Formatter, Write};

use bitvec::vec::BitVec;
use clap::Parser;
use rand::Rng;
use regex::Regex;

const STAR_CHAR: char = '*';

// A state is just a vector of bits (true or false, 1 or 0)
struct State(BitVec);

/// Represents a rule to be applied to the state
trait Rule {
    fn apply(a_neg1: bool, a_0: bool, a_1: bool) -> bool;
}

struct Rule1;
impl Rule for Rule1 {
    fn apply(a_neg1: bool, a_0: bool, a_1: bool) -> bool {
        (a_neg1 && !a_0 && !a_1) || (!a_neg1 && a_1) || (a_0 && a_1)
    }
}

struct Rule2;
impl Rule for Rule2 {
    fn apply(a_neg1: bool, a_0: bool, a_1: bool) -> bool {
        (a_neg1 && !a_0 && !a_1) || (!a_neg1 && a_1)
    }
}

impl State {
    /// Parse state from input string containing stars, spaces, and/or repeat directives.
    /// This is a bit complicated because of the handling for repeat directives.
    pub fn parse<T: AsRef<str>>(input: T) -> Self {
        let input = input.as_ref();
        let mut rng = rand::thread_rng();

        let regex = Regex::new(r"(?P<count>\d+)\[(?P<char>[ *?])]|(?P<single>[ *?])").unwrap();

        let mut ret = BitVec::new();

        // Keep track of the start of the last match to ensure we consume all of the input
        let mut start = 0;
        for m in regex.captures_iter(input) {
            // Handle repeat directive
            if let Some(count) = m.name("count") {
                if count.start() != start {
                    eprintln!("{} != {}", count.start(), start);
                    panic!(
                        "unparsed input at pos {}: '{}'",
                        start,
                        &input[start..count.start()]
                    );
                }

                let repetition_count: usize = count.as_str().parse().unwrap();
                let char = m.name("char").unwrap();

                // Account for closing square bracket
                start = char.end() + 1;

                let repetition_char = char.as_str().chars().next().unwrap();
                // This needs to be a closure, otherwise using '?' would result in repeating the
                // same character, which isn't very random.
                let repeater = || match repetition_char {
                    STAR_CHAR => true,
                    ' ' => false,
                    '?' => rng.gen_bool(0.5),
                    c => panic!(
                        "unexpected char '{}' in repetition '{}[{}]'",
                        c, repetition_count, c
                    ),
                };

                ret.extend(std::iter::repeat_with(repeater).take(repetition_count));
            } else if let Some(single) = m.name("single") {
                if single.start() != start {
                    panic!(
                        "unparsed input at pos {}: '{}'",
                        start,
                        &input[start..single.start()]
                    );
                }

                start = single.end();

                let c = single.as_str().chars().next().unwrap();
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

        // Ensure we consumed all of the input
        if start != input.len() {
            panic!("unparsed input at pos {}: '{}'", start, &input[start..]);
        }

        Self(ret)
    }

    /// Compute next state given `R`, a rule type
    fn next<R: Rule>(&self) -> State {
        let mut ret = BitVec::repeat(false, self.0.len());

        // The `Windows` iterator is very helpful here - it saves us from having to manually deal
        // with boundary conditions.
        for (i, slice) in self.0.windows(3).enumerate() {
            // Apply the rule
            ret.set(i + 1, R::apply(slice[0], slice[1], slice[2]));
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

#[derive(Parser)]
struct Args {
    input: String,

    #[clap(short, long, default_value_t = 25)]
    iterations: u32,
}

fn main() {
    let args = Args::parse();

    let mut state = State::parse(args.input);
    println!("{}", state);

    for _ in 0..args.iterations {
        state = state.next::<Rule2>();
        println!("{}", state);
    }
}

#[cfg(test)]
mod test {
    use bitvec::bitvec;
    use bitvec::prelude::Lsb0;

    use crate::State;

    #[test]
    fn parse() {
        let input = "     ";
        let state = State::parse(input);
        assert_eq!(state.0, bitvec![0, 0, 0, 0, 0]);
        assert_eq!(format!("{}", state), String::from("     "));
    }
}
