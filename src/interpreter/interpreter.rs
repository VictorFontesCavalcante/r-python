use std::collections::HashMap;

use crate::ir::ast::Expression;
use crate::ir::ast::Name;
use crate::ir::ast::Statement;

type ErrorMessage = String;

#[derive(Debug, Clone, PartialEq)]
pub enum EnvValue {
    CInt(i32),
    CReal(f32),
    Bool(bool),
    List(Vec<EvalResult>),
    Func(
        Box<EvalResult>,
        Option<HashMap<Name, Box<EvalResult>>>,
        Option<Box<Statement>>,
        Box<Expression>,
    ),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    CInt(i32),
    CReal(f32),
    Bool(bool),
    List(Vec<EvalResult>),
    None,
}

type Environment = HashMap<Name, EnvValue>;

pub fn eval(exp: &Expression, env: &Environment) -> Result<EvalResult, ErrorMessage> {
    match exp {
        Expression::CInt(v) => Ok(EvalResult::CInt(*v)),
        Expression::CReal(v) => Ok(EvalResult::CReal(*v)),
        Expression::Bool(v) => Ok(EvalResult::Bool(*v)),
        Expression::None => Ok(EvalResult::None),
        Expression::List(items) => {
            let mut list_vec: Vec<EvalResult> = Vec::new();
            let list_env = env.clone();

            if items.len() < 1 {
                return Err(String::from(
                    "List initialization must have at least one element",
                ));
            } else {
                let first_item = eval(&items[0], &list_env)?;
                for item in items {
                    let value = eval(&item, &list_env)?;
                    match (&first_item, &value) {
                        (EvalResult::CInt(_), EvalResult::CInt(_)) => list_vec.push(value),
                        (EvalResult::CReal(_), EvalResult::CReal(_)) => list_vec.push(value),
                        (EvalResult::Bool(_), EvalResult::Bool(_)) => list_vec.push(value),
                        (EvalResult::List(_), EvalResult::List(_)) => list_vec.push(value),
                        _ => return Err(String::from("List must be homogeneous")),
                    }
                }
            }
            Ok(EvalResult::List(list_vec))
        }
        Expression::Add(lhs, rhs) => {
            let lhs_value = eval(lhs, env)?;
            let rhs_value = eval(rhs, env)?;
            match (lhs_value, rhs_value) {
                (EvalResult::CInt(lhs), EvalResult::CInt(rhs)) => Ok(EvalResult::CInt(lhs + rhs)),
                (EvalResult::CReal(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs + rhs))
                }
                (EvalResult::CInt(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs as f32 + rhs))
                }
                (EvalResult::CReal(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CReal(lhs + rhs as f32))
                }
                (EvalResult::CInt(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs + rhs as i32))
                }
                (EvalResult::CReal(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CReal(lhs + (rhs as i32) as f32))
                }
                (EvalResult::Bool(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 + rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal((lhs as i32) as f32 + rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 + rhs as i32))
                }
                (EvalResult::List(lhs), EvalResult::List(rhs)) => {
                    let mut result_list = lhs.clone();
                    result_list.extend(rhs);
                    Ok(EvalResult::List(result_list))
                }
                (EvalResult::List(_), _) => Err(String::from("Can only concatenate list to list")),
                (_, EvalResult::List(_)) => Err(String::from("Can only concatenate list to list")),
                (EvalResult::None, _) => Err(String::from("Add is not supported for 'None'")),
                (_, EvalResult::None) => Err(String::from("Add is not supported for 'None'")),
            }
        }
        Expression::Sub(lhs, rhs) => {
            let lhs_value = eval(lhs, env)?;
            let rhs_value = eval(rhs, env)?;
            match (lhs_value, rhs_value) {
                (EvalResult::CInt(lhs), EvalResult::CInt(rhs)) => Ok(EvalResult::CInt(lhs - rhs)),
                (EvalResult::CReal(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs - rhs))
                }
                (EvalResult::CInt(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs as f32 - rhs))
                }
                (EvalResult::CReal(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CReal(lhs - rhs as f32))
                }
                (EvalResult::CInt(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs - rhs as i32))
                }
                (EvalResult::CReal(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CReal(lhs - (rhs as i32) as f32))
                }
                (EvalResult::Bool(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 - rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal((lhs as i32) as f32 - rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 - rhs as i32))
                }
                (EvalResult::List(_), _) => Err(String::from("Sub not supported for list")),
                (_, EvalResult::List(_)) => Err(String::from("Sub not supported for list")),
                (EvalResult::None, _) => Err(String::from("Sub is not supported for 'None'")),
                (_, EvalResult::None) => Err(String::from("Sub is not supported for 'None'")),
            }
        }
        Expression::Mul(lhs, rhs) => {
            let lhs_value = eval(lhs, env)?;
            let rhs_value = eval(rhs, env)?;
            match (lhs_value, rhs_value) {
                (EvalResult::CInt(lhs), EvalResult::CInt(rhs)) => Ok(EvalResult::CInt(lhs * rhs)),
                (EvalResult::CReal(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs * rhs))
                }
                (EvalResult::CInt(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal(lhs as f32 * rhs))
                }
                (EvalResult::CReal(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CReal(lhs * rhs as f32))
                }
                (EvalResult::CInt(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs * rhs as i32))
                }
                (EvalResult::CReal(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CReal(lhs * (rhs as i32) as f32))
                }
                (EvalResult::Bool(lhs), EvalResult::CInt(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 * rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::CReal(rhs)) => {
                    Ok(EvalResult::CReal((lhs as i32) as f32 * rhs))
                }
                (EvalResult::Bool(lhs), EvalResult::Bool(rhs)) => {
                    Ok(EvalResult::CInt(lhs as i32 * rhs as i32))
                }
                (EvalResult::List(lhs), EvalResult::CInt(rhs)) => {
                    let mut result_list = Vec::new();
                    for _i in 0..rhs {
                        result_list.extend(lhs.clone());
                    }
                    Ok(EvalResult::List(result_list))
                }
                (EvalResult::CInt(lhs), EvalResult::List(rhs)) => {
                    let mut result_list = Vec::new();
                    for _i in 0..lhs {
                        result_list.extend(rhs.clone());
                    }
                    Ok(EvalResult::List(result_list))
                }
                (EvalResult::List(lhs), EvalResult::Bool(rhs)) => {
                    let mut result_list = Vec::new();
                    for _i in 0..rhs as i32 {
                        result_list.extend(lhs.clone());
                    }
                    Ok(EvalResult::List(result_list))
                }
                (EvalResult::Bool(lhs), EvalResult::List(rhs)) => {
                    let mut result_list = Vec::new();
                    for _i in 0..lhs as i32 {
                        result_list.extend(rhs.clone());
                    }
                    Ok(EvalResult::List(result_list))
                }
                (EvalResult::List(_), _) => {
                    Err(String::from("Cannot multiply list by non-integer value"))
                }
                (_, EvalResult::List(_)) => {
                    Err(String::from("Cannot multiply list by non-integer value"))
                }
                (EvalResult::None, _) => Err(String::from("Mul is not supported for 'None'")),
                (_, EvalResult::None) => Err(String::from("Mul is not supported for 'None'")),
            }
        }
        Expression::Div(lhs, rhs) => {
            let lhs_value = eval(lhs, env)?;
            let rhs_value = eval(rhs, env)?;
            match (lhs_value, rhs_value) {
                (EvalResult::CInt(lhs), EvalResult::CInt(rhs)) => match rhs {
                    0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CInt(lhs / rhs)),
                },
                (EvalResult::CReal(lhs), EvalResult::CReal(rhs)) => match rhs {
                    0.0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CReal(lhs / rhs)),
                },
                (EvalResult::CInt(lhs), EvalResult::CReal(rhs)) => match rhs {
                    0.0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CReal(lhs as f32 / rhs)),
                },
                (EvalResult::CReal(lhs), EvalResult::CInt(rhs)) => match rhs {
                    0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CReal(lhs / rhs as f32)),
                },
                (EvalResult::CInt(lhs), EvalResult::Bool(rhs)) => match rhs {
                    false => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CInt(lhs / rhs as i32)),
                },
                (EvalResult::CReal(lhs), EvalResult::Bool(rhs)) => match rhs {
                    false => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CReal(lhs / (rhs as i32) as f32)),
                },
                (EvalResult::Bool(lhs), EvalResult::CInt(rhs)) => match rhs {
                    0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CInt(lhs as i32 / rhs)),
                },
                (EvalResult::Bool(lhs), EvalResult::CReal(rhs)) => match rhs {
                    0.0 => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CReal((lhs as i32) as f32 / rhs)),
                },
                (EvalResult::Bool(lhs), EvalResult::Bool(rhs)) => match rhs {
                    false => Err(String::from("Division by zero")),
                    _ => Ok(EvalResult::CInt(lhs as i32 / rhs as i32)),
                },
                (EvalResult::List(_), _) => Err(String::from("Div not supported for list")),
                (_, EvalResult::List(_)) => Err(String::from("Div not supported for list")),
                (EvalResult::None, _) => Err(String::from("Div is not supported for 'None'")),
                (_, EvalResult::None) => Err(String::from("Div is not supported for 'None'")),
            }
        }
        Expression::Var(name) => match env.get(name) {
            Some(EnvValue::CInt(value)) => Ok(EvalResult::CInt(*value)),
            Some(EnvValue::CReal(value)) => Ok(EvalResult::CReal(*value)),
            Some(EnvValue::Bool(value)) => Ok(EvalResult::Bool(*value)),
            Some(EnvValue::List(value)) => Ok(EvalResult::List(value.clone())),
            Some(EnvValue::None) => Ok(EvalResult::None),
            _ => Err(format!("Variable {} not found", name)),
        },
        Expression::FuncCall(name, args) => match env.get(name) {
            Some(EnvValue::Func(kind, params, stmt, retrn)) => {
                let mut func_env = env.clone();

                let new_params: HashMap<String, Box<EvalResult>> = match params {
                    None => HashMap::new(),
                    Some(s) => s.clone(),
                };

                let new_args: Vec<Expression> = match args {
                    None => Vec::new(),
                    Some(s) => s.clone(),
                };

                if new_args.len() != new_params.len() {
                    return Err(format!(
                        "{} requires {} arguments, got {}",
                        name,
                        new_params.len(),
                        new_args.len()
                    ));
                }

                for (param, arg) in new_params.iter().zip(new_args.iter()) {
                    let value = eval(arg, env)?;

                    match (*param.1.clone(), value) {
                        (EvalResult::CInt(_), EvalResult::CInt(v)) => {
                            func_env.insert(param.0.clone(), EnvValue::CInt(v));
                        }
                        (EvalResult::CReal(_), EvalResult::CReal(v)) => {
                            func_env.insert(param.0.clone(), EnvValue::CReal(v));
                        }
                        (EvalResult::Bool(_), EvalResult::Bool(v)) => {
                            func_env.insert(param.0.clone(), EnvValue::Bool(v));
                        }
                        (EvalResult::List(_), EvalResult::List(v)) => {
                            func_env.insert(param.0.clone(), EnvValue::List(v));
                        }
                        _ => return Err(format!("Mismatched types for {:?}", param.1)),
                    }
                }

                if let Some(body_stmt) = stmt {
                    match execute(body_stmt, func_env.clone()) {
                        Ok(result_env) => {
                            let result = eval(&retrn, &result_env)?;
                            let kind_type = *kind.clone();
                            match (kind_type, result) {
                                (EvalResult::CInt(_), EvalResult::CInt(v)) => {
                                    Ok(EvalResult::CInt(v))
                                }
                                (EvalResult::CReal(_), EvalResult::CReal(v)) => {
                                    Ok(EvalResult::CReal(v))
                                }
                                (EvalResult::Bool(_), EvalResult::Bool(v)) => {
                                    Ok(EvalResult::Bool(v))
                                }
                                (EvalResult::List(_), EvalResult::List(v)) => {
                                    Ok(EvalResult::List(v))
                                }
                                (EvalResult::None, EvalResult::None) => Ok(EvalResult::None),
                                _ => Err(format!(
                                    "{} returned a value different from specified type",
                                    name
                                )),
                            }
                        }
                        Err(err) => Err(format!("{} generated an error: {}", name, err)),
                    }
                } else {
                    let result = eval(&retrn, &func_env)?;
                    let kind_type = *kind.clone();
                    match (kind_type, result) {
                        (EvalResult::CInt(_), EvalResult::CInt(v)) => Ok(EvalResult::CInt(v)),
                        (EvalResult::CReal(_), EvalResult::CReal(v)) => Ok(EvalResult::CReal(v)),
                        (EvalResult::Bool(_), EvalResult::Bool(v)) => Ok(EvalResult::Bool(v)),
                        (EvalResult::List(_), EvalResult::List(v)) => Ok(EvalResult::List(v)),
                        (EvalResult::None, EvalResult::None) => Ok(EvalResult::None),
                        _ => Err(format!(
                            "{} returned a value different from specified type",
                            name
                        )),
                    }
                }
            }
            _ => Err(format!("{} is not defined", name)),
        },
        Expression::Range(exp1, exp2, exp3) => {
            let new_env = env.clone();
            let end_value = eval(exp2, &new_env)?;

            let mut srt_value = eval(&Expression::CInt(0), &new_env)?;
            let mut incr_value = eval(&Expression::CInt(1), &new_env)?;

            match (exp1, exp3) {
                (None, None) => (),
                (None, Some(incr_stp)) => {
                    incr_value = eval(&incr_stp, &new_env)?;
                }
                (Some(srt_step), None) => {
                    srt_value = eval(&srt_step, &new_env)?;
                }
                (Some(srt_step), Some(incr_step)) => {
                    srt_value = eval(&srt_step, &new_env)?;
                    incr_value = eval(&incr_step, &new_env)?;
                }
            }

            let srt_int: i32;
            let end_int: i32;
            let incr_int: i32;

            match (srt_value, end_value, incr_value) {
                (EvalResult::CInt(i), EvalResult::CInt(j), EvalResult::CInt(k)) => {
                    srt_int = i;
                    end_int = j;
                    incr_int = k;
                }
                (EvalResult::CInt(i), EvalResult::CInt(j), EvalResult::Bool(k)) => {
                    srt_int = i;
                    end_int = j;
                    incr_int = k as i32;
                }
                (EvalResult::CInt(i), EvalResult::Bool(j), EvalResult::CInt(k)) => {
                    srt_int = i;
                    end_int = j as i32;
                    incr_int = k;
                }
                (EvalResult::CInt(i), EvalResult::Bool(j), EvalResult::Bool(k)) => {
                    srt_int = i;
                    end_int = j as i32;
                    incr_int = k as i32;
                }
                (EvalResult::Bool(i), EvalResult::CInt(j), EvalResult::CInt(k)) => {
                    srt_int = i as i32;
                    end_int = j;
                    incr_int = k;
                }
                (EvalResult::Bool(i), EvalResult::CInt(j), EvalResult::Bool(k)) => {
                    srt_int = i as i32;
                    end_int = j;
                    incr_int = k as i32;
                }
                (EvalResult::Bool(i), EvalResult::Bool(j), EvalResult::CInt(k)) => {
                    srt_int = i as i32;
                    end_int = j as i32;
                    incr_int = k;
                }
                (EvalResult::Bool(i), EvalResult::Bool(j), EvalResult::Bool(k)) => {
                    srt_int = i as i32;
                    end_int = j as i32;
                    incr_int = k as i32;
                }
                _ => return Err(String::from("Parameters cannot be converted to integer")),
            }

            let mut range_vec: Vec<EvalResult> = Vec::new();

            match incr_int.signum() {
                0 => Err(String::from("Increment cannot be zero")),
                -1 => {
                    for i in (end_int + incr_int.abs()..=srt_int)
                        .rev()
                        .step_by(incr_int.abs() as usize)
                    {
                        range_vec.push(EvalResult::CInt(i))
                    }
                    Ok(EvalResult::List(range_vec))
                }
                1 => {
                    for i in (srt_int..end_int).step_by(incr_int as usize) {
                        range_vec.push(EvalResult::CInt(i));
                    }
                    Ok(EvalResult::List(range_vec))
                }
                _ => Ok(EvalResult::List(range_vec)),
            }
        }
    }
}

pub fn execute(stmt: &Statement, env: Environment) -> Result<Environment, ErrorMessage> {
    match stmt {
        Statement::Assignment(name, exp) => {
            let value = eval(exp, &env)?;
            let mut new_env = env;
            match value {
                EvalResult::CInt(val) => {
                    new_env.insert(*name.clone(), EnvValue::CInt(val));
                }
                EvalResult::CReal(val) => {
                    new_env.insert(*name.clone(), EnvValue::CReal(val));
                }
                EvalResult::Bool(val) => {
                    new_env.insert(*name.clone(), EnvValue::Bool(val));
                }
                EvalResult::List(val) => {
                    new_env.insert(*name.clone(), EnvValue::List(val));
                }
                EvalResult::None => {
                    new_env.insert(*name.clone(), EnvValue::None);
                }
            }
            Ok(new_env)
        }
        Statement::IfThenElse(cond, stmt_then, stmt_else) => {
            let value = match eval(cond, &env) {
                Ok(EvalResult::CInt(v)) => v != 0,
                Ok(EvalResult::CReal(v)) => v != 0.0,
                Ok(EvalResult::Bool(v)) => v,
                Ok(EvalResult::List(v)) => !v.is_empty(),
                Ok(EvalResult::None) => false,
                Err(s) => return Err(format!("Condition resulted in an error: {}", s)),
            };

            if value {
                execute(stmt_then, env)
            } else {
                execute(stmt_else, env)
            }
        }
        Statement::While(cond, stmt) => {
            let mut new_env = env.clone();
            loop {
                let value = match eval(cond, &new_env) {
                    Ok(EvalResult::CInt(v)) => v != 0,
                    Ok(EvalResult::CReal(v)) => v != 0.0,
                    Ok(EvalResult::Bool(v)) => v,
                    Ok(EvalResult::List(v)) => !v.is_empty(),
                    Ok(EvalResult::None) => false,
                    Err(s) => return Err(format!("Condition resulted in an error: {}", s)),
                };

                if value {
                    new_env = execute(stmt, new_env)?;
                } else {
                    break;
                }
            }
            Ok(new_env)
        }
        Statement::Func(name, kind, params, stmt, retrn) => {
            let mut new_env = env.clone();

            new_env.insert(
                *name.clone(),
                EnvValue::Func(kind.clone(), params.clone(), stmt.clone(), retrn.clone()),
            );
            Ok(new_env)
        }
        Statement::For(var, exp, stmt) => {
            let mut new_env = env;
            let exp_value = eval(&exp, &new_env)?;
            match exp_value {
                EvalResult::List(vec) => {
                    for item in vec {
                        match item {
                            EvalResult::CInt(v) => {
                                new_env.insert(*var.clone(), EnvValue::CInt(v));
                            }
                            EvalResult::CReal(v) => {
                                new_env.insert(*var.clone(), EnvValue::CReal(v));
                            }
                            EvalResult::Bool(v) => {
                                new_env.insert(*var.clone(), EnvValue::Bool(v));
                            }
                            EvalResult::List(v) => {
                                new_env.insert(*var.clone(), EnvValue::List(v));
                            }
                            EvalResult::None => {
                                new_env.insert(*var.clone(), EnvValue::None);
                            }
                        }
                        new_env = execute(stmt, new_env)?;
                    }
                }
                _ => return Err(String::from("Expression must be an iterable object")),
            }
            new_env.remove(&var as &str);
            Ok(new_env)
        }
        Statement::Sequence(s1, s2) => execute(s1, env).and_then(|new_env| execute(s2, new_env)),
        _ => Err(String::from("not implemented yet")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_constant_integer() {
        let env = HashMap::new();
        let c10 = Expression::CInt(10);
        let c20 = Expression::CInt(20);

        assert_eq!(eval(&c10, &env), Ok(EvalResult::CInt(10)));
        assert_eq!(eval(&c20, &env), Ok(EvalResult::CInt(20)));
    }

    #[test]
    fn eval_constant_real() {
        let env = HashMap::new();
        let c10_5 = Expression::CReal(10.5);
        let c20_3 = Expression::CReal(20.3);

        assert_eq!(eval(&c10_5, &env), Ok(EvalResult::CReal(10.5)));
        assert_eq!(eval(&c20_3, &env), Ok(EvalResult::CReal(20.3)))
    }

    #[test]
    fn eval_constant_bool() {
        let env = HashMap::new();
        let ctrue = Expression::Bool(true);
        let cfalse = Expression::Bool(false);

        assert_eq!(eval(&ctrue, &env), Ok(EvalResult::Bool(true)));
        assert_eq!(eval(&cfalse, &env), Ok(EvalResult::Bool(false)))
    }

    #[test]
    fn eval_constant_list() {
        let env = HashMap::new();
        let cl1 = Expression::List(vec![Expression::CInt(1), Expression::CInt(2)]);
        let cl2 = Expression::List(vec![Expression::CReal(23.3), Expression::CReal(0.00)]);

        assert_eq!(
            eval(&cl1, &env),
            Ok(EvalResult::List(vec![
                EvalResult::CInt(1),
                EvalResult::CInt(2)
            ]))
        );
        assert_eq!(
            eval(&cl2, &env),
            Ok(EvalResult::List(vec![
                EvalResult::CReal(23.3),
                EvalResult::CReal(0.00)
            ]))
        );
    }

    #[test]
    fn eval_list_of_list() {
        let env = HashMap::new();
        let cl1 = Expression::List(vec![Expression::List(vec![Expression::CInt(1)])]);

        assert_eq!(
            eval(&cl1, &env),
            Ok(EvalResult::List(vec![EvalResult::List(vec![
                EvalResult::CInt(1)
            ])]))
        );
    }

    #[test]
    fn eval_add_integers_1() {
        let env = HashMap::new();
        let c10 = Expression::CInt(10);
        let c20 = Expression::CInt(20);
        let add1 = Expression::Add(Box::new(c10), Box::new(c20));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CInt(30)));
    }

    #[test]
    fn eval_add_integers_2() {
        let env = HashMap::new();
        let c10 = Expression::CInt(10);
        let c20 = Expression::CInt(20);
        let c30 = Expression::CInt(30);
        let add1 = Expression::Add(Box::new(c10), Box::new(c20));
        let add2 = Expression::Add(Box::new(add1), Box::new(c30));
        assert_eq!(eval(&add2, &env), Ok(EvalResult::CInt(60)));
    }

    #[test]
    fn eval_add_reals_1() {
        let env = HashMap::new();
        let c10_5 = Expression::CReal(10.5);
        let c20_3 = Expression::CReal(20.3);
        let add1 = Expression::Add(Box::new(c10_5), Box::new(c20_3));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CReal(30.8)));
    }

    #[test]
    fn eval_add_reals_2() {
        let env = HashMap::new();
        let c10_5 = Expression::CReal(10.5);
        let c20_3 = Expression::CReal(20.3);
        let c30_1 = Expression::CReal(30.1);
        let add1 = Expression::Add(Box::new(c10_5), Box::new(c20_3));
        let add2 = Expression::Add(Box::new(add1), Box::new(c30_1));
        assert_eq!(eval(&add2, &env), Ok(EvalResult::CReal(60.9)));
    }

    #[test]
    fn eval_add_integer_real() {
        let env = HashMap::new();
        let c10 = Expression::CInt(10);
        let c20_3 = Expression::CReal(20.3);
        let add1 = Expression::Add(Box::new(c10), Box::new(c20_3));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CReal(30.3)));
    }

    #[test]
    fn eval_add_bools_1() {
        let env = HashMap::new();
        let ctrue = Expression::Bool(true);
        let cfalse = Expression::Bool(false);
        let add1 = Expression::Add(Box::new(ctrue), Box::new(cfalse));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CInt(1)));
    }

    #[test]
    fn eval_add_bools_2() {
        let env = HashMap::new();
        let ctrue1 = Expression::Bool(true);
        let ctrue2 = Expression::Bool(true);
        let add1 = Expression::Add(Box::new(ctrue1), Box::new(ctrue2));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CInt(2)));
    }

    #[test]
    fn eval_add_num_bool() {
        let env = HashMap::new();
        let c10 = Expression::CInt(10);
        let ctrue2 = Expression::Bool(true);
        let add1 = Expression::Add(Box::new(c10), Box::new(ctrue2));
        assert_eq!(eval(&add1, &env), Ok(EvalResult::CInt(11)));
    }

    #[test]
    fn eval_add_lists() {
        let env = HashMap::new();
        let l1 = Expression::List(vec![Expression::CInt(0), Expression::CInt(1)]);
        let l2 = Expression::List(vec![Expression::CInt(2), Expression::CInt(3)]);
        let add = Expression::Add(Box::new(l1), Box::new(l2));
        assert_eq!(
            eval(&add, &env),
            Ok(EvalResult::List(vec![
                EvalResult::CInt(0),
                EvalResult::CInt(1),
                EvalResult::CInt(2),
                EvalResult::CInt(3)
            ]))
        );
    }

    #[test]
    fn eval_multiply_list() {
        let env = HashMap::new();
        let l1 = Expression::List(vec![Expression::CInt(0), Expression::CInt(1)]);
        let l2 = Expression::List(vec![Expression::CInt(0), Expression::CInt(1)]);
        let mul1 = Expression::Mul(Box::new(l1), Box::new(Expression::CInt(2)));
        let mul2 = Expression::Mul(Box::new(l2), Box::new(Expression::CInt(0)));
        assert_eq!(
            eval(&mul1, &env),
            Ok(EvalResult::List(vec![
                EvalResult::CInt(0),
                EvalResult::CInt(1),
                EvalResult::CInt(0),
                EvalResult::CInt(1)
            ]))
        );
        assert_eq!(eval(&mul2, &env), Ok(EvalResult::List(vec![])));
    }

    #[test]
    fn eval_variable() {
        let env = HashMap::from([
            (String::from("w"), EnvValue::CInt(10)),
            (String::from("x"), EnvValue::CReal(20.7)),
            (String::from("y"), EnvValue::Bool(true)),
            (
                String::from("z"),
                EnvValue::List(vec![EvalResult::CInt(1), EvalResult::CInt(2)]),
            ),
        ]);
        let v1 = Expression::Var(String::from("w"));
        let v2 = Expression::Var(String::from("x"));
        let v3 = Expression::Var(String::from("y"));
        let v4 = Expression::Var(String::from("z"));
        assert_eq!(eval(&v1, &env), Ok(EvalResult::CInt(10)));
        assert_eq!(eval(&v2, &env), Ok(EvalResult::CReal(20.7)));
        assert_eq!(eval(&v3, &env), Ok(EvalResult::Bool(true)));
        assert_eq!(
            eval(&v4, &env),
            Ok(EvalResult::List(vec![
                EvalResult::CInt(1),
                EvalResult::CInt(2)
            ]))
        );
    }

    #[test]
    fn execute_assignment_same_variable() {
        let env = HashMap::new();
        let a1 = Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(1)));
        let a2 = Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(2)));
        let seq = Statement::Sequence(Box::new(a1), Box::new(a2));

        match execute(&seq, env) {
            Ok(new_env) => match new_env.get("x") {
                Some(EnvValue::CInt(2)) => {}
                Some(value) => assert!(false, "Expected 2, got {:?}", value),
                None => assert!(false, "Variable x not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn execute_assignment() {
        let env = HashMap::new();
        let assign_stmt =
            Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(42)));

        match execute(&assign_stmt, env) {
            Ok(new_env) => match new_env.get("x") {
                Some(EnvValue::CInt(42)) => {}
                Some(value) => assert!(false, "Expected 42, got {:?}", value),
                None => assert!(false, "Variable x not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_expression_with_variables() {
        let env = HashMap::from([
            (String::from("a"), EnvValue::CInt(5)),
            (String::from("b"), EnvValue::CInt(3)),
        ]);
        let expr = Expression::Mul(
            Box::new(Expression::Var(String::from("a"))),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("b"))),
                Box::new(Expression::CInt(2)),
            )),
        );
        assert_eq!(eval(&expr, &env), Ok(EvalResult::CInt(25)));
    }

    #[test]
    fn eval_nested_expressions() {
        let env = HashMap::new();
        let expr = Expression::Add(
            Box::new(Expression::Mul(
                Box::new(Expression::CInt(2)),
                Box::new(Expression::CInt(3)),
            )),
            Box::new(Expression::Sub(
                Box::new(Expression::CInt(10)),
                Box::new(Expression::CInt(4)),
            )),
        );
        assert_eq!(eval(&expr, &env), Ok(EvalResult::CInt(12)));
    }

    #[test]
    fn eval_variable_not_found() {
        let env = HashMap::new();
        let var_expr = Expression::Var(String::from("z"));

        assert_eq!(
            eval(&var_expr, &env),
            Err(String::from("Variable z not found"))
        );
    }

    #[test]
    fn eval_summation() {
        /*
         * (a test case for the following program)
         *
         * > x = 10
         * > y = 0
         * > while x:
         * >   y = y + x
         * >   x = x - 1
         *
         * After executing this program, 'x' must be zero and
         * 'y' must be 55.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(10)));
        let a2 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let a3 = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("x"))),
            )),
        );
        let a4 = Statement::Assignment(
            Box::new(String::from("x")),
            Box::new(Expression::Sub(
                Box::new(Expression::Var(String::from("x"))),
                Box::new(Expression::CInt(1)),
            )),
        );

        let seq1 = Statement::Sequence(Box::new(a3), Box::new(a4));

        let while_statement =
            Statement::While(Box::new(Expression::Var(String::from("x"))), Box::new(seq1));

        let seq2 = Statement::Sequence(Box::new(a2), Box::new(while_statement));
        let program = Statement::Sequence(Box::new(a1), Box::new(seq2));

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("y") {
                    Some(EnvValue::CInt(55)) => {}
                    Some(val) => assert!(false, "Expected 55, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("x") {
                    Some(EnvValue::CInt(0)) => {}
                    Some(val) => assert!(false, "Expected 0, got {:?}", val),
                    None => assert!(false, "Variable x not found"),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_simple_if_then_else() {
        /*
         * Test for simple if-then-else statement
         *
         * > x = 10
         * > if x > 5:
         * >   y = 1
         * > else:
         * >   y = 0
         *
         * After executing, 'y' should be 1.
         */
        let env = HashMap::new();

        let condition = Expression::Var(String::from("x"));
        let then_stmt =
            Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(1)));
        let else_stmt =
            Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));

        let if_statement = Statement::IfThenElse(
            Box::new(condition),
            Box::new(then_stmt),
            Box::new(else_stmt),
        );

        let setup_stmt =
            Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(10)));
        let program = Statement::Sequence(Box::new(setup_stmt), Box::new(if_statement));

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("y") {
                Some(EnvValue::CInt(1)) => {}
                Some(val) => assert!(false, "Expected 1, got {:?}", val),
                None => assert!(false, "Variable y not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_while_loop_decrement() {
        /*
         * Test for while loop that decrements a variable
         *
         * > x = 3
         * > y = 10
         * > while x:
         * >   y = y - 1
         * >   x = x - 1
         *
         * After executing, 'y' should be 7 and 'x' should be 0.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(3)));
        let a2 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(10)));
        let a3 = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Sub(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::CInt(1)),
            )),
        );
        let a4 = Statement::Assignment(
            Box::new(String::from("x")),
            Box::new(Expression::Sub(
                Box::new(Expression::Var(String::from("x"))),
                Box::new(Expression::CInt(1)),
            )),
        );

        let seq1 = Statement::Sequence(Box::new(a3), Box::new(a4));
        let while_statement =
            Statement::While(Box::new(Expression::Var(String::from("x"))), Box::new(seq1));
        let program = Statement::Sequence(
            Box::new(a1),
            Box::new(Statement::Sequence(Box::new(a2), Box::new(while_statement))),
        );

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("y") {
                    Some(EnvValue::CInt(7)) => {}
                    Some(val) => assert!(false, "Expected 7, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("x") {
                    Some(EnvValue::CInt(0)) => {}
                    Some(val) => assert!(false, "Expected 0, got {:?}", val),
                    None => assert!(false, "Variable x not found"),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_for_loop_increment() {
        /*
         * For loop test for variable increment
         *
         * > y = 0
         *
         * > for i in range(0, 5, 2):
         * >    y = y + i
         *
         * After executing, 'y' should be 6 and 'i' should not be accessible.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let for_exec = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("i"))),
            )),
        );

        let range = Expression::Range(
            Some(Box::new(Expression::CInt(0))),
            Box::new(Expression::CInt(5)),
            Some(Box::new(Expression::CInt(2))),
        );

        let for_stmt = Statement::For(
            Box::new(String::from("i")),
            Box::new(range),
            Box::new(for_exec),
        );

        let program = Statement::Sequence(Box::new(a1), Box::new(for_stmt));

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("y") {
                    Some(EnvValue::CInt(6)) => {}
                    Some(val) => assert!(false, "Expected 6, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("i") {
                    None => {}
                    Some(val) => assert!(false, "Expected None, got {:?}", val),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_for_loop_decrement() {
        /*
         * For loop test for variable decrement
         *
         * > y = 0
         *
         * > for i in range(10, 3, -1):
         * >    y = y + i
         *
         * After executing, 'y' should be 49 and 'i' should not be accessible.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let for_exec = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("i"))),
            )),
        );

        let range = Expression::Range(
            Some(Box::new(Expression::CInt(10))),
            Box::new(Expression::CInt(3)),
            Some(Box::new(Expression::CInt(-1))),
        );

        let for_stmt = Statement::For(
            Box::new(String::from("i")),
            Box::new(range),
            Box::new(for_exec),
        );

        let program = Statement::Sequence(Box::new(a1), Box::new(for_stmt));

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("y") {
                    Some(EnvValue::CInt(49)) => {}
                    Some(val) => assert!(false, "Expected 49, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("i") {
                    None => {}
                    Some(val) => assert!(false, "Expected None, got {:?}", val),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_for_loop_no_values() {
        /*
         * For loop test for a loop specified by stop only
         *
         * > y = 0
         *
         * > for i in range(5):
         * >    y = y + i
         *
         * After executing, 'y' should be 10 and 'i' should not be accessible.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let for_exec = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("i"))),
            )),
        );

        let range = Expression::Range(None, Box::new(Expression::CInt(5)), None);

        let for_stmt = Statement::For(
            Box::new(String::from("i")),
            Box::new(range),
            Box::new(for_exec),
        );

        let program = Statement::Sequence(Box::new(a1), Box::new(for_stmt));

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("y") {
                    Some(EnvValue::CInt(10)) => {}
                    Some(val) => assert!(false, "Expected 10, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("i") {
                    None => {}
                    Some(val) => assert!(false, "Expected None, got {:?}", val),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_for_loop_no_range() {
        /*
         * For loop test for condition never reached
         *
         * > y = 0
         *
         * > for i in range(0, 1, -1):
         * >    y = y + i
         *
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let for_exec = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("i"))),
            )),
        );

        let range = Expression::Range(
            Some(Box::new(Expression::CInt(0))),
            Box::new(Expression::CInt(1)),
            Some(Box::new(Expression::CInt(-1))),
        );

        let for_stmt = Statement::For(
            Box::new(String::from("i")),
            Box::new(range),
            Box::new(for_exec),
        );

        let program = Statement::Sequence(Box::new(a1), Box::new(for_stmt));

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("y") {
                Some(EnvValue::CInt(0)) => (),
                Some(val) => assert!(false, "Expected 0, got {:?}", val),
                None => assert!(false, "Variable y not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_for_loop_list() {
        /*
         * For loop test for a list of objects
         *
         * > y = 0
         *
         * > for i in [1, 3, 5]:
         * >    y = y + i
         *
         * After executing, 'y' should be 9  and 'i' should not be accessible.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));

        let for_exec = Statement::Assignment(
            Box::new(String::from("y")),
            Box::new(Expression::Add(
                Box::new(Expression::Var(String::from("y"))),
                Box::new(Expression::Var(String::from("i"))),
            )),
        );

        let l1 = Expression::List(vec![
            Expression::CInt(1),
            Expression::CInt(3),
            Expression::CInt(5),
        ]);

        let for_stmt = Statement::For(
            Box::new(String::from("i")),
            Box::new(l1),
            Box::new(for_exec),
        );

        let program = Statement::Sequence(Box::new(a1), Box::new(for_stmt));

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("y") {
                Some(EnvValue::CInt(9)) => (),
                Some(val) => assert!(false, "Expected 9, got {:?}", val),
                None => assert!(false, "Variable y not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_nested_if_statements() {
        /*
         * Test for nested if-then-else statements
         *
         * > x = 10
         * > if x > 5:
         * >   if x > 8:
         * >     y = 1
         * >   else:
         * >     y = 2
         * > else:
         * >   y = 0
         *
         * After executing, 'y' should be 1.
         */
        let env = HashMap::new();

        let inner_then_stmt =
            Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(1)));
        let inner_else_stmt =
            Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(2)));
        let inner_if_statement = Statement::IfThenElse(
            Box::new(Expression::Var(String::from("x"))),
            Box::new(inner_then_stmt),
            Box::new(inner_else_stmt),
        );

        let outer_else_stmt =
            Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let outer_if_statement = Statement::IfThenElse(
            Box::new(Expression::Var(String::from("x"))),
            Box::new(inner_if_statement),
            Box::new(outer_else_stmt),
        );

        let setup_stmt =
            Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(10)));
        let program = Statement::Sequence(Box::new(setup_stmt), Box::new(outer_if_statement));

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("y") {
                Some(EnvValue::CInt(1)) => {}
                Some(val) => assert!(false, "Expected 1, got {:?}", val),
                None => assert!(false, "Variable y not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn eval_complex_sequence() {
        /*
         * Sequence with multiple assignments and expressions
         *
         * > x = 5
         * > y = 0
         * > z = 2 * x + 3
         *
         * After executing, 'x' should be 5, 'y' should be 0, and 'z' should be 13.
         */
        let env = HashMap::new();

        let a1 = Statement::Assignment(Box::new(String::from("x")), Box::new(Expression::CInt(5)));
        let a2 = Statement::Assignment(Box::new(String::from("y")), Box::new(Expression::CInt(0)));
        let a3 = Statement::Assignment(
            Box::new(String::from("z")),
            Box::new(Expression::Add(
                Box::new(Expression::Mul(
                    Box::new(Expression::CInt(2)),
                    Box::new(Expression::Var(String::from("x"))),
                )),
                Box::new(Expression::CInt(3)),
            )),
        );

        let program = Statement::Sequence(
            Box::new(a1),
            Box::new(Statement::Sequence(Box::new(a2), Box::new(a3))),
        );

        match execute(&program, env) {
            Ok(new_env) => {
                match new_env.get("x") {
                    Some(EnvValue::CInt(5)) => {}
                    Some(val) => assert!(false, "Expected 5, got {:?}", val),
                    None => assert!(false, "Variable x not found"),
                }
                match new_env.get("y") {
                    Some(EnvValue::CInt(0)) => {}
                    Some(val) => assert!(false, "Expected 0, got {:?}", val),
                    None => assert!(false, "Variable y not found"),
                }
                match new_env.get("z") {
                    Some(EnvValue::CInt(13)) => {}
                    Some(val) => assert!(false, "Expected 13, got {:?}", val),
                    None => assert!(false, "Variable z not found"),
                }
            }
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn func_decl_call() {
        /*
         * Test for declaration and call of a function
         *
         * > def add(a: CInt, b: CInt) -> CInt:
         * >    t = a + b
         * >    return t
         * >
         * > sum = add(5, 7)
         *
         * After executing, 'sum' should be 12.
         */
        let env = Environment::new();

        let mut args = HashMap::new();
        args.insert(String::from("a"), Box::new(EvalResult::CInt(0)));
        args.insert(String::from("b"), Box::new(EvalResult::CInt(0)));

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("add")),
                Box::new(EvalResult::CInt(0)),
                Some(args),
                Some(Box::new(Statement::Assignment(
                    Box::new(String::from("t")),
                    Box::new(Expression::Add(
                        Box::new(Expression::Var(String::from("a"))),
                        Box::new(Expression::Var(String::from("b"))),
                    )),
                ))),
                Box::new(Expression::Var(String::from("t"))),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("sum")),
                Box::new(Expression::FuncCall(
                    String::from("add"),
                    Some(vec![Expression::CInt(5), Expression::CInt(7)]),
                )),
            )),
        );

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("sum") {
                Some(EnvValue::CInt(12)) => {}
                Some(val) => assert!(false, "Expected 12, got {:?}", val),
                None => assert!(false, "Variable sum not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn func_decl_call_without_stmt() {
        /*
         * Test for declaration and call of a function with no statement
         *
         * > def add(a: CInt, b: CInt) -> CInt:
         * >    return a + b
         * >
         * > sum = add(1, 2)
         *
         * After executing, 'sum' should be 3.
         */
        let env = Environment::new();

        let mut args = HashMap::new();
        args.insert(String::from("a"), Box::new(EvalResult::CInt(0)));
        args.insert(String::from("b"), Box::new(EvalResult::CInt(0)));

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("add")),
                Box::new(EvalResult::CInt(0)),
                Some(args),
                None,
                Box::new(Expression::Add(
                    Box::new(Expression::Var(String::from("a"))),
                    Box::new(Expression::Var(String::from("b"))),
                )),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("sum")),
                Box::new(Expression::FuncCall(
                    String::from("add"),
                    Some(vec![Expression::CInt(1), Expression::CInt(2)]),
                )),
            )),
        );

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("sum") {
                Some(EnvValue::CInt(3)) => {}
                Some(val) => assert!(false, "Expected 3, got {:?}", val),
                None => assert!(false, "Variable sum not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn func_decl_call_without_args() {
        /*
         * Test for declaration and call of a function with no arguments
         *
         * > def two_plus_two() -> CInt:
         * >    return 4
         * >
         * > value = two_plus_two()
         *
         * After executing, 'sum' should be 4.
         */
        let env = Environment::new();

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("two_plus_two")),
                Box::new(EvalResult::CInt(0)),
                None,
                None,
                Box::new(Expression::CInt(4)),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("value")),
                Box::new(Expression::FuncCall(String::from("two_plus_two"), None)),
            )),
        );

        match execute(&program, env) {
            Ok(new_env) => match new_env.get("value") {
                Some(EnvValue::CInt(4)) => {}
                Some(val) => assert!(false, "Expected 4, got {:?}", val),
                None => assert!(false, "Variable value not found"),
            },
            Err(s) => assert!(false, "{}", s),
        }
    }

    #[test]
    fn num_arguments_error_func_call() {
        /*
         * Test for declaration and call of a function where the passed
         * arguments don't match the functions definition
         *
         * > def add(a: CInt, b: CInt) -> CInt:
         * >    return a + b
         * >
         * > sum = add(1, 2, 3)
         *
         */
        let env = Environment::new();

        let mut args = HashMap::new();
        args.insert(String::from("a"), Box::new(EvalResult::CInt(0)));
        args.insert(String::from("b"), Box::new(EvalResult::CInt(0)));

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("add")),
                Box::new(EvalResult::CInt(0)),
                Some(args),
                None,
                Box::new(Expression::Add(
                    Box::new(Expression::Var(String::from("a"))),
                    Box::new(Expression::Var(String::from("b"))),
                )),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("sum")),
                Box::new(Expression::FuncCall(
                    String::from("add"),
                    Some(vec![
                        Expression::CInt(1),
                        Expression::CInt(2),
                        Expression::CInt(3),
                    ]),
                )),
            )),
        );

        match execute(&program, env) {
            Ok(_) => assert!(false, "Function should generate an error"),
            Err(s) => assert_eq!(s, "add requires 2 arguments, got 3"),
        }
    }

    #[test]
    fn arguments_type_error_func_call() {
        /*
         * Test for declaration and call of a function where the passed
         * arguments don't match their defined types on the function
         *
         * > def add(a: CInt, b: CReal) -> CReal:
         * >    return a + b
         * >
         * > sum = add(1, 2)
         *
         */
        let env = Environment::new();

        let mut args = HashMap::new();
        args.insert(String::from("a"), Box::new(EvalResult::CInt(0)));
        args.insert(String::from("b"), Box::new(EvalResult::CReal(0.0)));

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("add")),
                Box::new(EvalResult::CReal(0.0)),
                Some(args),
                None,
                Box::new(Expression::Add(
                    Box::new(Expression::Var(String::from("a"))),
                    Box::new(Expression::Var(String::from("b"))),
                )),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("sum")),
                Box::new(Expression::FuncCall(
                    String::from("add"),
                    Some(vec![Expression::CInt(1), Expression::CInt(2)]),
                )),
            )),
        );

        match execute(&program, env) {
            Ok(_) => assert!(false, "Function should generate an error"),
            Err(s) => assert_eq!(s, "Mismatched types for CReal(0.0)"),
        }
    }

    #[test]
    fn func_return_type_error() {
        /*
         * Test for declaration and call of a function where the return type
         * is different from the one defined by the function
         *
         * > def add(a: CReal, b: CReal) -> CInt:
         * >    return a + b
         * >
         * > sum = add(1.5, 2.5)
         *
         */
        let env = Environment::new();

        let mut args = HashMap::new();
        args.insert(String::from("a"), Box::new(EvalResult::CReal(1.5)));
        args.insert(String::from("b"), Box::new(EvalResult::CReal(2.5)));

        let program = Statement::Sequence(
            Box::new(Statement::Func(
                Box::new(String::from("add")),
                Box::new(EvalResult::CInt(0)),
                Some(args),
                None,
                Box::new(Expression::Add(
                    Box::new(Expression::Var(String::from("a"))),
                    Box::new(Expression::Var(String::from("b"))),
                )),
            )),
            Box::new(Statement::Assignment(
                Box::new(String::from("sum")),
                Box::new(Expression::FuncCall(
                    String::from("add"),
                    Some(vec![Expression::CReal(1.5), Expression::CReal(2.5)]),
                )),
            )),
        );

        match execute(&program, env) {
            Ok(_) => assert!(false, "Function should generate an error"),
            Err(s) => assert_eq!(s, "add returned a value different from specified type"),
        }
    }

    #[test]
    fn undefined_func_call() {
        let env = Environment::new();

        let program = Box::new(Statement::Assignment(
            Box::new(String::from("sum")),
            Box::new(Expression::FuncCall(
                String::from("add"),
                Some(vec![Expression::CInt(1), Expression::CInt(2)]),
            )),
        ));

        match execute(&program, env) {
            Ok(_) => assert!(false, "Function not supposed to execute"),
            Err(s) => assert_eq!(s, "add is not defined"),
        }
    }
}
