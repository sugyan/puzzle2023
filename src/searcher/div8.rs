use crate::{ExpressionElement, Fraction, Op, Rpn};

pub struct Div8Searcher {
    nums: Vec<i32>,
    target: i32,
    stack: Vec<Fraction>,
    eight_depth: i32,
}

impl Div8Searcher {
    pub fn new(nums: Vec<i32>, target: i32) -> Self {
        let stack = Vec::with_capacity(nums.len());
        Self {
            nums,
            target,
            stack,
            eight_depth: -1,
        }
    }
}

impl Rpn for Div8Searcher {
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
        let mut ret = false;
        if let Some(&n) = self.get(i) {
            ret |= self.backtrack(expr, (i + 1, j), results, ExpressionElement::Operand(n));
        }
        if i > j + 1 {
            for op in &Op::ALL {
                if self.eight_depth == 0 && op != &Op::Div {
                    continue;
                }
                ret |= self.backtrack(expr, (i, j + 1), results, ExpressionElement::Operator(*op));
            }
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
                let orig = self.eight_depth;
                if n == 8 {
                    self.eight_depth = 0;
                } else if self.eight_depth >= 0 {
                    self.eight_depth += 1;
                }
                self.stack.push(Fraction(n, 1));
                expr.push(e);
                ret |= self.traverse(expr, (i, j), results);
                expr.pop();
                self.stack.pop();
                self.eight_depth = orig;
            }
            ExpressionElement::Operator(op) => {
                if let (Some(n0), Some(n1)) = (self.stack.pop(), self.stack.pop()) {
                    if let Some(n) = op.apply(&n1, &n0) {
                        self.eight_depth -= 1;
                        self.stack.push(n);
                        expr.push(e);
                        ret |= self.traverse(expr, (i, j), results);
                        expr.pop();
                        self.stack.pop();
                        self.eight_depth += 1;
                    }
                    self.stack.push(n1);
                    self.stack.push(n0);
                }
            }
        }
        ret
    }
}
