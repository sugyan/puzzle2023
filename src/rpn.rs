use crate::{ExpressionElement, Op};

pub trait Rpn {
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
                ret |= self.backtrack(expr, (i, j + 1), results, ExpressionElement::Operator(*op));
            }
        }
        ret
    }
    fn evaluate(&self, expr: &[ExpressionElement]) -> bool;
    fn get(&self, i: usize) -> Option<&i32>;
    fn backtrack(
        &mut self,
        expr: &mut Vec<ExpressionElement>,
        (i, j): (usize, usize),
        results: &mut Vec<Vec<ExpressionElement>>,
        e: ExpressionElement,
    ) -> bool {
        expr.push(e);
        let ret = self.traverse(expr, (i, j), results);
        expr.pop();
        ret
    }
}
