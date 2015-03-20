#![feature(core)]
requires newline
requires end
requires word
requires space

extern crate regex;

use std::collections::HashSet;
use std::old_io;

use regex::Regex;
use parse::{Flags, FLAG_EMPTY, FLAG_NEGATE,};
use regex::native::{
	OneChar, CharClass, Any, Save, Jump, Split,
	Match, EmptyBegin, EmptyEnd, EmptyWordBoundary, Dynamic,
};

/* The "empty" match types cause the matching behavior of a state to depend on how you arrive
at that state. Thus, for every state in the NFA, there is really a number of "modified"
states, which we identify by this flag, indicating an additional constraint on how one can
exit that state. Ending only matches "match", etc. */
enum TransitionMod { Normal, Newline, Ending, Word, Space, Empty};

fn nextState(re: Vec<Inst>, state: Vec<(usize, TransitionMod)>) -> (HashMap<char, usize>, Vec<Vec<(usize, bool)>>) {
	let (start, _) = state[0];
	let mut current = 0;
	let mut queue = vec!{(start, Normal)};
	while current < queue.size() {
		let (cstate, m) = queue[current];
		let (a1, a2) = match re[cstate] {
			Jump(i) => (Some((i, m)), None),
			Split(i1, i2) => (Some((i1, m)), Some((i2, m))),
			Save(_) => (Some(cstate+1, m), None),
			EmptyBegin(flags) => {
				(Some(cstate+1, m), None)
			},
			EmptyEnd(flags) => {
				(Some(cstate+1, m), None)
			},
			EmptyWordBoundry(flags) => {
				nm = if	flags & FLAG_NEGATE == FLAGS_EMPTY {
					match m {
						Normal => Word,
						Newline => Empty,
						Ending => Empty,
						Word => Word,
						Space => Empty,
						Empty => Empty,
					}
				}
				else {
					match m {
						Normal => Space,
						Newline => Newline,
						Ending => Ending,
						Word => Empty,
						Space => Space,
						Empty => Empty,
					}
				};
				(Some(cstate+1, nm), None)
			},
			_ => ,
		};
	}
}

fn main() {
	loop {
		print!("\n> ");
		let input = old_io::stdin().read_line().ok().expect("Failed to read line");
		let r = Regex::new(&input[..input.len()-1]);
		match r {
			Err(e) => println!("Broken: {}", e),
			Ok(Regex::Dynamic(r)) => {
				let prog = r.prog.insts;
				for (line, command) in prog.iter().enumerate() {
			
					let r = match *command {
						Match => format!("Match"),
						OneChar(c,f) => format!("OneChar({},{})", c, f),
						CharClass(_,f) => format!("CharClass(?,{})",f),
						Any(_) => format!("Any"),
						EmptyBegin(_) => format!("Begin"),
						EmptyEnd(_) => format!("End"),
						EmptyWordBoundary(_) => format!("WordBoundry"),
						Save(i) => format!("Save({})", i),
						Jump(i) => format!("Jump({})", i),
						Split(i1, i2) => format!("Split({},{})", i1, i2),
					};
					println!("{}: {}", line, r);
				}
			}
			_ => println!("blarf"),
		}
	}
}
