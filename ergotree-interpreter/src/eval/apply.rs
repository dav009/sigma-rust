use ergotree_ir::mir::apply::Apply;
use ergotree_ir::mir::val_def::ValId;
use ergotree_ir::mir::value::Value;

use crate::eval::env::Env;
use crate::eval::EvalContext;
use crate::eval::EvalError;
use crate::eval::Evaluable;

impl Evaluable for Apply {
    fn eval(&self, env: &Env, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let func_v = self.func.eval(env, ctx)?;
        let args_v_res: Result<Vec<Value>, EvalError> =
            self.args.iter().map(|arg| arg.eval(env, ctx)).collect();
        let args_v = args_v_res?;
        match func_v {
            Value::Lambda(fv) => {
                let arg_ids: Vec<ValId> = fv.args.iter().map(|a| a.idx).collect();
                let mut cur_env = env.clone();
                arg_ids.iter().zip(args_v).for_each(|(idx, arg_v)| {
                    cur_env.insert(*idx, arg_v);
                });
                fv.body.eval(&cur_env, ctx)
            }
            _ => Err(EvalError::UnexpectedValue(format!(
                "expected func_v to be Value::FuncValue got: {0:?}",
                func_v
            ))),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::rc::Rc;

    use ergotree_ir::mir::bin_op::BinOp;
    use ergotree_ir::mir::bin_op::RelationOp;
    use ergotree_ir::mir::block::BlockValue;
    use ergotree_ir::mir::expr::Expr;
    use ergotree_ir::mir::func_value::FuncArg;
    use ergotree_ir::mir::func_value::FuncValue;
    use ergotree_ir::mir::val_def::ValDef;
    use ergotree_ir::mir::val_use::ValUse;
    use ergotree_ir::types::stype::SType;
    use sigma_test_util::force_any_val;

    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;

    use super::*;

    #[test]
    fn eval_user_defined_func_call() {
        let arg = Expr::Const(1i32.into());
        let bin_op = Expr::BinOp(BinOp {
            kind: RelationOp::Eq.into(),
            left: Box::new(
                ValUse {
                    val_id: 1.into(),
                    tpe: SType::SInt,
                }
                .into(),
            ),
            right: Box::new(
                ValUse {
                    val_id: 2.into(),
                    tpe: SType::SInt,
                }
                .into(),
            ),
        });
        let body = Expr::BlockValue(BlockValue {
            items: vec![ValDef {
                id: 2.into(),
                rhs: Box::new(Expr::Const(1i32.into())),
            }
            .into()],
            result: Box::new(bin_op),
        });
        let apply: Expr = Apply::new(
            FuncValue::new(
                vec![FuncArg {
                    idx: 1.into(),
                    tpe: SType::SInt,
                }],
                body,
            )
            .into(),
            vec![arg],
        )
        .unwrap()
        .into();
        let ctx = Rc::new(force_any_val::<Context>());
        assert!(eval_out::<bool>(&apply, ctx));
    }
}
