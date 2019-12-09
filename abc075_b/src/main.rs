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
    "3 5\n.....\n.#.#.\n.....\n" => "11211\n1#2#1\n11211\n",
    "3 5\n#####\n#####\n#####\n" => "#####\n#####\n#####\n",
    "6 6\n#####.\n#.#.##\n####.#\n.#..#.\n#.##..\n#.#...\n" => "#####3\n#8#7##\n####5#\n4#65#2\n#5##21\n#4#310\n",
}

fn main() {
    println!("{}", solve(&stdin!()));
}

fn solve(src: &str) -> String {
    input! {
        source = src,
        h: isize,
        w: isize,
        v: [chars; h],
    };

    let mut v = v;

    let dy = [-1, -1, -1, 0, 0,  1, 1, 1];
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..h {
        for j in 0..w {
            if v[i as usize][j as usize] == '#' {
                continue;
            }

            let mut count = 0;

            for d in 0..8 {
                let ni = i + dy[d];
                let nj = j + dx[d];

                if ni < 0 || h <= ni {
                    continue;
                }
                
                if nj < 0 || w <= nj {
                    continue
                };

                if v[ni as usize][nj as usize] == '#' {
                    count += 1;
                }
            }

            v[i as usize][j as usize] = std::char::from_digit(count as u32, 10).unwrap();
        }
    }

    let mut ans = String::new();
    for i in 0..h {
        let s = format!("{}\n",v[i as usize].iter().cloned().collect::<String>());
        ans.push_str(&s);
    }

    ans
}