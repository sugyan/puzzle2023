use clap::{ArgAction, Parser, ValueEnum};
use std::{collections::BTreeSet, time::Instant};

use puzzle2023::{DefaultSearcher, Div8Searcher, ExpressionElement, FastSearcher, Op, Rpn};

/// Solvers for https://twitter.com/tkihira/status/1609313732034965506
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "2023")]
    target: i32,
    #[arg(short, long, default_value = "10")]
    nums: i32,
    #[arg(short, long, default_value = "true", action = ArgAction::Set)]
    rev: bool,
    #[arg(value_enum, short, long, default_value = "default")]
    searcher: Searcher,
    #[arg(long, default_value = "false")]
    normalize: bool,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone, ValueEnum)]
enum Searcher {
    Default,
    Fast,
    Div8,
}

fn solve(searcher: &mut impl Rpn, normalize: bool, verbose: bool) {
    let now = Instant::now();
    let mut results = Vec::new();
    searcher.traverse(&mut Vec::new(), (0, 0), &mut results);
    let mut bts = BTreeSet::new();
    for result in results {
        let mut stack = Vec::new();
        for e in &result {
            match e {
                ExpressionElement::Operand(n) => stack.push((n.to_string(), None)),
                ExpressionElement::Operator(op) => {
                    if let (Some((mut s0, o0)), Some((mut s1, o1))) = (stack.pop(), stack.pop()) {
                        if normalize {
                            if matches!((op, o1), (Op::Mul | Op::Div, Some(Op::Add | Op::Sub))) {
                                s1 = format!("({s1})");
                            }
                            if matches!(
                                (op, o0),
                                (Op::Sub | Op::Mul, Some(Op::Add | Op::Sub)) | (Op::Div, Some(_))
                            ) {
                                s0 = format!("({s0})");
                            }
                        } else {
                            if o1.is_some() {
                                s1 = format!("({s1})");
                            }
                            if o0.is_some() {
                                s0 = format!("({s0})");
                            }
                        }
                        stack.push((format!("{s1}{op}{s0}"), Some(*op)));
                    }
                }
            }
        }
        bts.insert(stack[0].0.clone());
    }
    let elapsed = now.elapsed();
    for s in &bts {
        println!("{s}");
    }
    if verbose {
        println!("Completed {} results in {elapsed:?}", bts.len());
    }
}

fn main() {
    let args = Args::parse();
    let nums = if args.rev {
        (1..=args.nums).rev().collect()
    } else {
        (1..=args.nums).collect()
    };
    match args.searcher {
        Searcher::Default => solve(
            &mut DefaultSearcher::new(nums, args.target),
            args.normalize,
            args.verbose,
        ),
        Searcher::Fast => solve(
            &mut FastSearcher::new(nums, args.target),
            args.normalize,
            args.verbose,
        ),
        Searcher::Div8 => solve(
            &mut Div8Searcher::new(nums, args.target),
            args.normalize,
            args.verbose,
        ),
    };
}
