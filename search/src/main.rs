use eval::*;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const num_cand: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const N: usize = 5;
const op_cand: [Op; N - 1] = [Op::Add, Op::Sub, Op::Mul, Op::Div];

fn main() {
    let mut lst = vec![];
    for nums in itertools::iproduct!(num_cand, num_cand, num_cand, num_cand, num_cand) {
        let nums = [nums.0, nums.1, nums.2, nums.3, nums.4];

        // all number should be unique.
        // no? -> comment out
        let mut h = std::collections::HashSet::new();
        for n in nums {
            h.insert(n);
        }
        if h.len() != N {
            continue;
        }

        lst.push(nums);
    }
    let mut res: Vec<([u8; N], usize)> = lst
        .into_par_iter()
        .map(|nums| {
            let len = calc_one(nums);
            (nums, len)
        })
        .collect();
    res.sort_by_key(|x| x.1);
    if let Some((nums, len)) = res.last() {
        println!("{:?} -> {}", nums, len);
        for (v, ops) in op_search(&nums, *len).iter().enumerate() {
            let mut s = String::new();
            s.push_str(&nums[0].to_string());
            for i in 0..N - 1 {
                let op = match ops[i] {
                    Op::Add => "+",
                    Op::Sub => "-",
                    Op::Mul => "x",
                    Op::Div => "÷",
                };
                s.push_str(op);

                let num = nums[i + 1].to_string();
                s.push_str(&num);
            }
            s.push('=');
            let ans = v.to_string();
            s.push_str(&ans);
            println!("{s}");
        }
    } else {
        println!("* -> 0");
    }
}
fn calc_one(nums: [u8; N]) -> usize {
    let mut h = std::collections::HashSet::new();
    for ops in itertools::iproduct!(op_cand, op_cand, op_cand, op_cand) {
        let ops = [ops.0, ops.1, ops.2, ops.3];
        match eval::eval(&nums, &ops) {
            Some(n) => {
                h.insert(n);
            }
            None => {}
        }
    }
    let mut out = 0;
    for i in 0.. {
        if h.contains(&i) {
            out += 1;
        } else {
            break;
        }
    }
    out
}
fn op_search(nums: &[u8; N], len: usize) -> Vec<[Op; 4]> {
    let mut out = vec![Default::default(); len];
    for ops in itertools::iproduct!(op_cand, op_cand, op_cand, op_cand) {
        let ops = [ops.0, ops.1, ops.2, ops.3];
        match eval::eval(nums, &ops) {
            Some(n) => {
                let n = n as usize;
                if n < len {
                    out[n as usize] = ops;
                }
            }
            None => {}
        }
    }
    out
}
#[test]
fn test_calc_one_nf() {
    let inp = [6, 4, 5, 2, 1];
    dbg!(calc_one(inp));
}
