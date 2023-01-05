use crate::{ExpressionElement, Fraction, Rpn};

pub struct DefaultSearcher {
    nums: Vec<i32>,
    target: i32,
}

impl DefaultSearcher {
    pub fn new(nums: Vec<i32>, target: i32) -> Self {
        Self { nums, target }
    }
}

impl Rpn for DefaultSearcher {
    fn evaluate(&self, expr: &[ExpressionElement]) -> bool {
        let mut stack = Vec::new();
        for e in expr {
            match e {
                ExpressionElement::Operand(n) => stack.push(Fraction(*n, 1)),
                ExpressionElement::Operator(op) => {
                    if let (Some(n0), Some(n1)) = (stack.pop(), stack.pop()) {
                        if let Some(n) = op.apply(&n1, &n0) {
                            stack.push(n);
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        stack
            .last()
            .map(|n| n.1 * self.target == n.0)
            .unwrap_or(false)
    }
    fn get(&self, i: usize) -> Option<&i32> {
        self.nums.get(i)
    }
}
