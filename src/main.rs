#![allow(unused_macros)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::Ordering::{self, *};
use std::io::{prelude::*, stdin, stdout, BufWriter};
use std::os::raw::c_char;
use std::cmp::min;

/**
 * hm/dc <= hc/dm 이면 끗
 * 10억 번 정도의 연산은, +,- 정도의 연산만 하는 거라면 브루트 포스로 시도하자
 */

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
        input!(words, hc = i64, dc = i64);
        input!(words, hm = i64, dm = i64);
        input!(words, k = i64, w = i64, a = i64);

        let mut is_win = false;
        
        for j in 0..k+1 {
            let nhc = hc + j * a;
            let ndc = dc + (k-j) * w;

            // a/b -> (a + b -1) / b
            // 이 기법은 나눗셈을 ceil하는 방법이다.
            // 예시) 5/2 는 결과가 2로 나온다.
            // 이를 (5+2-1)/2로 바꾸면 결과가 3으로 나온다.
            if (hm + ndc - 1) / ndc <= (nhc + dm - 1)/dm {
                is_win = true;
                break;
            }
        }

        if is_win {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}