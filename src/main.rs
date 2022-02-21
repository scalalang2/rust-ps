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
    input!(words, n = u64, k = u64);

    let mut num = 0;
    for i in 1..n+1 {
        if n % i == 0 {
            num += 1;
            if num == k {
                println!("{}", i);
                return;
            }
        }
    }

    println!("0")
}