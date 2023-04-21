use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(arith_expr) => IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr) => BoolValue(eval_bool_expr(bool_expr))
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr { left, right, op } => {
            let a = eval_arith_expr(*left);
            let b = eval_arith_expr(*right);
            match op {
                AddOp => a + b,
                SubOp => a - b,
                MulOp => a * b,
                IntDivOp => a / b
            }
        },
        IntLit(num) => num
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr { left, right, op } => {
            let p = eval_arith_expr(*left);
            let q = eval_arith_expr(*right);
            match op {
                LtOp => p < q,
                LteOp => p <= q,
                GtOp => p > q,
                GteOp => p >= q,
                ArithEqOp => p == q,
                ArithNeqOp => p != q
            }
        }
        BinBoolExpr { left, right, op } => {
            let x = eval_bool_expr(*left);
            let y = eval_bool_expr(*right);
            match op {
                AndOp => x && y,
                OrOp => x || y,
                BoolEqOp => x == y,
                BoolNeqOp => x != y
            }
        }
        NotExpr(bool_ex) => eval_bool_expr(*bool_ex),
        BoolLit(bool) => bool
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
        //TODO TESTSSS!!!!
    //...

    #[test]
    fn test_case_tree() {
        let tree = BoolExpr(
            ArithCmpExpr {
                left: Box::new(BinArithExpr { 
                    left: Box::new(IntLit(12)),
                    right: Box::new(IntLit(155)),
                    op: AddOp
                }),
                right: Box::new(IntLit(167)),
                op: LteOp
            }
        );

        assert_eq!(eval(tree), BoolValue(true));
    }

    #[test]
    fn test_1() {
        let tree = ArithExpr(
            BinArithExpr { 
                left: Box::new(BinArithExpr { 
                    left: Box::new(IntLit(50)),
                    right: Box::new(IntLit(20)),
                    op: SubOp }
                ),
                right: Box::new(BinArithExpr {
                    left: Box::new(IntLit(5)),
                    right: Box::new(IntLit(6)),
                    op: MulOp }
                ),
                op: IntDivOp
            }
        );

        assert_eq!(eval(tree), IntValue(1));
    }

    #[test]
    fn test_2() {
        let tree = BoolExpr(
            BinBoolExpr {
                left: Box::new(BinBoolExpr {
                    left: Box::new(ArithCmpExpr {
                        left: Box::new(IntLit(5)),
                        right: Box::new(IntLit(3)),
                        op: LtOp }
                    ),
                    right: Box::new(ArithCmpExpr {
                        left: Box::new(IntLit(4)),
                        right: Box::new(IntLit(2)),
                        op: GtOp }
                    ),
                    op: AndOp }
                ),
                right: Box::new(BinBoolExpr {
                    left: Box::new(BinBoolExpr {
                        left: Box::new(BoolLit(true)),
                        right: Box::new(BoolLit(false)),
                        op: BoolNeqOp }
                    ),
                    right: Box::new(NotExpr(Box::new(BinBoolExpr {
                        left: Box::new(BinBoolExpr {
                            left: Box::new(ArithCmpExpr {
                                left: Box::new(IntLit(5)),
                                right: Box::new(IntLit(5)),
                                op: ArithEqOp }
                            ),
                            right: Box::new(ArithCmpExpr {
                                left: Box::new(IntLit(0)),
                                right: Box::new(IntLit(4)),
                                op: ArithNeqOp }
                            ),
                            op: OrOp }
                        ),
                        right: Box::new(ArithCmpExpr {
                            left: Box::new(IntLit(5)),
                            right: Box::new(IntLit(9)),
                            op: GteOp }
                        ),
                        op: OrOp }))
                    ),
                    op: BoolEqOp }
                ),
                op: OrOp
            }
        );

        assert_eq!(eval(tree), BoolValue(true));
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}

