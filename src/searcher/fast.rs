use crate::{ExpressionElement, Op, Rpn};
use std::collections::HashSet;

type Key = (Vec<(i32, i32)>, (usize, usize));

pub struct FastSearcher {
    nums: Vec<i32>,
    target: i32,
    stack: Vec<(i32, i32)>,
    seen: HashSet<Key>,
}

impl FastSearcher {
    pub fn new(nums: Vec<i32>, target: i32) -> Self {
        let stack = Vec::with_capacity(nums.len());
        Self {
            nums,
            target,
            stack,
            seen: HashSet::new(),
        }
    }
}

impl Rpn for FastSearcher {
    fn evaluate(&self, _: &[ExpressionElement]) -> bool {
        self.stack[0].1 * self.target == self.stack[0].0
    }
    fn get(&self, i: usize) -> Option<&i32> {
        self.nums.get(i)
    }
    fn traverse(
        &mut self,
        expr: &mut Vec<ExpressionElement>,
        (i, j): (usize, usize),
        results: &mut Vec<Vec<ExpressionElement>>,
    ) -> bool {
        if i == j + 1 && self.get(i).is_none() {
            let found = self.evaluate(expr);
            if found {
                results.push(expr.clone());
            }
            return found;
        }
        let key = (self.stack.clone(), (i, j));
        if self.seen.contains(&key) {
            return false;
        }
        let mut ret = false;
        if let Some(&n) = self.get(i) {
            ret |= self.backtrack(expr, (i + 1, j), results, ExpressionElement::Operand(n));
        }
        if i > j + 1 {
            for op in &Op::ALL {
                ret |= self.backtrack(expr, (i, j + 1), results, ExpressionElement::Operator(*op));
            }
        }
        if !ret {
            self.seen.insert(key);
        }
        ret
    }
    fn backtrack(
        &mut self,
        expr: &mut Vec<ExpressionElement>,
        (i, j): (usize, usize),
        results: &mut Vec<Vec<ExpressionElement>>,
        e: ExpressionElement,
    ) -> bool {
        let mut ret = false;
        match e {
            ExpressionElement::Operand(n) => {
                self.stack.push((n, 1));
                expr.push(e);
                ret |= self.traverse(expr, (i, j), results);
                expr.pop();
                self.stack.pop();
            }
            ExpressionElement::Operator(op) => {
                if let (Some(n0), Some(n1)) = (self.stack.pop(), self.stack.pop()) {
                    if let Some(n) = op.apply(&n1, &n0) {
                        let gcd = super::gcd(n.0, n.1);
                        self.stack.push((n.0 / gcd, n.1 / gcd));
                        expr.push(e);
                        ret |= self.traverse(expr, (i, j), results);
                        expr.pop();
                        self.stack.pop();
                    }
                    self.stack.push(n1);
                    self.stack.push(n0);
                }
            }
        }
        ret
    }
}
