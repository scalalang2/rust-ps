#![allow(unused_macros)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::Ordering::{self, *};
use std::io::{prelude::*, stdin, stdout, BufWriter};
use std::os::raw::c_char;
use std::cmp::min;

macro_rules! input {
    () => {};
    ($iter:ident, $id:ident = &str ) => { let $id = $iter.next().unwrap(); };
    ($iter:ident, $id:pat = $t:ty ) => { let $id:$t = $iter.next().unwrap().parse().unwrap(); };
    ($iter:ident, $id:ident = &str, $($tail:tt)* ) => { let $id = $iter.next().unwrap(); input!($iter, $($tail)*) };
    ($iter:ident, $id:pat = $t:ty, $($tail:tt)* ) => { let $id:$t = $iter.next().unwrap().parse().unwrap(); input!($iter, $($tail)*) };
    ($iter:ident, ? $id:ident = &str ) => { let $id = match $iter.next() { Some(x) => x, _ => break }; };
    ($iter:ident, ? $id:pat = $t:ty ) => { let $id:$t = match $iter.next() { Some(x) => x, _ => break }.parse().unwrap(); };
    ($iter:ident, ? $id:ident = &str, $($tail:tt)* ) => { let $id = match $iter.next() { Some(x) => x, _ => break }; input!($iter, $($tail)*) };
    ($iter:ident, ? $id:pat = $t:ty, $($tail:tt)* ) => { let $id:$t = match $iter.next() { Some(x) => x, _ => break }.parse().unwrap(); input!($iter, $($tail)*) };
}

macro_rules! r {
    ($iter:ident, &str) => { $iter.next().unwrap() };
    ($iter:ident, $t:ty ) => { $iter.next().unwrap().parse::<$t>().unwrap() };
}

pub fn main() {
    let stdin = stdin();
    let stdin = &mut stdin.lock();
    let mut buf = String::with_capacity(1 << 20);
    stdin.read_to_string(&mut buf).unwrap();
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut words = buf.split_ascii_whitespace();
    macro_rules! print { ($($tt:tt)*) => { write!(stdout, $($tt)*).unwrap() }; }
    macro_rules! println { ($($tt:tt)*) => { writeln!(stdout, $($tt)*).unwrap() }; }

    // from here to end
    input!(words, f = i32, s = usize, g = usize, u = i32, d = i32);
    let mut dist = [-1; 1_000_001];
    dist[s] = 0;

    let mut queue:VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((s as i32, dist[s]));
    
    while !queue.is_empty() {
        let (curr, step) = queue.pop_front().unwrap();
        if curr + u <= f && dist[(curr + u) as usize] == -1{
            dist[(curr + u) as usize] = step + 1;
            queue.push_back((curr + u, step + 1));
        }

        if curr - d >= 1 && dist[(curr - d) as usize] == -1 {
            dist[(curr - d) as usize] = step + 1;
            queue.push_back((curr - d, step + 1));
        }
    }

    if dist[g] != -1 {
        println!("{}", dist[g]);
    } else {
        println!("use the stairs");
    }
}