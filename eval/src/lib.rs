use itertools::Itertools;
use num::{Rational32, Zero};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Op {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
}
fn prio(op: Op) -> u8 {
    match op {
        Op::Add => 0,
        Op::Sub => 0,
        Op::Mul => 1,
        Op::Div => 1,
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Elem {
    Num(u8),
    Op(Op),
}
fn into_rpn(nums: &[u8], ops: &[Op]) -> Vec<Elem> {
    assert_eq!(nums.len(), ops.len() + 1);

    let nums = nums.iter().map(|&x| Elem::Num(x));
    let ops = ops.iter().map(|&x| Elem::Op(x));
    let it = nums.interleave(ops);

    let mut rpn = vec![];
    let mut op_stack = vec![];
    for e in it {
        match e {
            Elem::Num(n) => rpn.push(Elem::Num(n)),
            Elem::Op(new_op) => {
                let new_prio = prio(new_op);
                while let Some(top) = op_stack.pop() {
                    let top_prio = prio(top);
                    if new_prio > top_prio {
                        op_stack.push(top);
                        break;
                    } else {
                        rpn.push(Elem::Op(top));
                    }
                }
                op_stack.push(new_op);
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        rpn.push(Elem::Op(op));
    }
    rpn
}

fn eval_rpn(rpn: &[Elem]) -> Option<u32> {
    let mut calc_stack: Vec<Rational32> = vec![];
    for e in rpn {
        match e {
            Elem::Op(op) => {
                use Op::*;
                let y = calc_stack.pop().unwrap();
                let x = calc_stack.pop().unwrap();
                // x op y
                let z = match op {
                    Add => x + y,
                    Sub => x - y,
                    Mul => x * y,
                    Div => {
                        if y.is_zero() {
                            return None;
                        }
                        x / y
                    }
                };
                calc_stack.push(z);
            }
            Elem::Num(n) => {
                calc_stack.push(Rational32::new(*n as i32, 1));
            }
        }
    }

    let ans = calc_stack.pop().unwrap();
    if ans.is_integer() {
        let n = ans.to_integer() as u32;
        Some(n)
    } else {
        None
    }
}

pub fn eval(nums: &[u8], ops: &[Op]) -> Option<u32> {
    let rpn = into_rpn(nums, ops);
    eval_rpn(&rpn)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rational32() {
        let n5 = Rational32::new(5, 1);
        let n4 = Rational32::new(4, 1);
        let n0 = Rational32::new(0, 1);

        dbg!(n5 / n4);
        assert!(!(n5 / n4).is_integer());
        assert!((n5 / n5).is_integer());
        assert!((n0 / n5).is_integer());
    }
    #[test]
    fn test_rpn_wiki() {
        use super::*;
        let nums = [1, 2, 3, 4];
        let ops = [Op::Add, Op::Mul, Op::Sub];
        let rpn = into_rpn(&nums, &ops);
        assert_eq!(
            rpn,
            vec![
                Elem::Num(1),
                Elem::Num(2),
                Elem::Num(3),
                Elem::Op(Op::Mul),
                Elem::Op(Op::Add),
                Elem::Num(4),
                Elem::Op(Op::Sub)
            ]
        );
    }
    #[test]
    fn test_eval_easy() {
        use Op::*;
        let nums = [1, 2, 3, 4, 5];
        let ops = [Add, Add, Add, Add];
        assert_eq!(eval(&nums, &ops), Some(15));
    }
    #[test]
    fn test_eval_29() {
        use Op::*;
        let nums = [6, 4, 5, 2, 1];
        let ops = [Add, Mul, Add, Add];
        assert_eq!(eval(&nums, &ops), Some(29));
    }
    #[test]
    fn test_eval_31() {
        use Op::*;
        let nums = [6, 4, 5, 2, 1];
        let ops = [Mul, Add, Add, Div];
        assert_eq!(eval(&nums, &ops), Some(31));
    }
}
