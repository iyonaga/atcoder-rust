// Input macro
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

// Test Code
macro_rules! test {
    ($($input:expr => $output:expr),* $(,)*) => {
        #[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input), $output);
            )*
        }
    };
}

test! {
    "9 45000" => "0 9 0",
    "20 196000" => "-1 -1 -1",
    "1000 1234000" => "2 54 944",
    "2000 20000000" => "2000 0 0"
}

fn main() {
    println!("{}", solve(&stdin!()));
}

fn solve(src: &str) -> String {
    input! {
        source = src,
        n: i32,
        y: i32
    }

    let mut ans = None;

    'outer: for i in 0..n  + 1{
        for j in 0..n - i + 1 {
            let k = n - i - j;

            if i * 10000 + j * 5000 + k * 1000 == y {
                ans = Some((i, j, k));
                break 'outer;
            }
        }
    }

    let (x, y, z) = ans.unwrap_or((-1, -1, -1));
    format!("{} {} {}", x, y, z)
}
