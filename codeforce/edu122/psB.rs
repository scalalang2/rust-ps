/**
 * - 해설
 * 가장 많이 제거할 수 있는 경우를 생각해보자
 * 서로 개수가 다른 경우에는 0의 개수와 1의 개수 중 적은 수를 제거하면 최적해가 된다.
 * 경합이 발생하는 경우는 0의 개수와 1의 개수가 서로 동일한 경우 밖에 없다.
 * 이 때는 개수를 절반으로 나누고 1을 빼주는 경우가 최적해가 된다.
 * 11110000 <- 1110000 이거나 1111000 의 경우 밖에 존재하지 않는다. 어떻게 섞여있어도 개수가 동일하다면 1 or 0의 개수 - 1이 최적해가 된다.
 */
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
    input!(words, t = u64);
    for _ in 0..t {
        input!(words, a = String);
        let mut ch = a.chars();
        let mut len = a.len();
        let mut zeros:usize = 0;
        let mut ones:usize = 0;

        while let Some(b) = ch.next() {
            if b == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        println!("{}", min(min(ones, zeros), (len - 1) / 2))
    }
}