use crate::parser::ast::*;
use crate::utils::owo::owoify;
use std::collections::HashMap;

// === Runtime Structures ===

#[derive(Debug, Default)]
struct Environment {
    functions: HashMap<String, FunctionDeclarationNode>,
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(i8),
}

// === Entry Point ===

pub fn run(ast: &[ASTNode]) {
    let mut env = Environment::default();

    // Pass 1: Collect function declarations
    for node in ast {
        if let ASTNode::FunctionDeclaration(func) = node {
            env.functions.insert(func.name.clone(), func.clone());
        }
    }

    // Pass 2: Execute statements
    for node in ast {
        match node {
            ASTNode::OwO(_) | ASTNode::Print(_) | ASTNode::FunctionCall(_) | ASTNode::VariableDeclaration(_) | ASTNode::KindOf(_) => {
                execute(node, &mut env);
            }
            _ => {}
        }
    }
}

// === Evaluation ===

fn evaluate(node: &ASTNode, env: &Environment) -> Value {
    match node {
        ASTNode::StringLiteral(s) => Value::String(s.value.clone()),
        ASTNode::NumberLiteral(n) => Value::Number(n.value),
        ASTNode::BoolLiteral(b) => Value::Bool(b.value),

        ASTNode::VariableReference(var) => env
            .variables
            .get(&var.name)
            .cloned()
            .unwrap_or_else(|| panic!("Variable \"{}\" is not defined", var.name)),

        ASTNode::Print(p) => evaluate(&p.expression, env),
        ASTNode::OwO(p) => evaluate(&p.expression, env),
        
        ASTNode::KindOf(k) => evaluate_kind_of(k, env),

        ASTNode::BinaryExpression(expr) => {
            let left = evaluate(&expr.left, env);
            let right = evaluate(&expr.right, env);

            match expr.operator.as_str() {
                "+" => match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (Value::String(l), Value::String(r)) => Value::String(l.clone() + r),
                    (Value::String(l), Value::Number(r)) => {
                        Value::String(l.clone() + &r.to_string())
                    }
                    (Value::Number(l), Value::String(r)) => Value::String(l.to_string() + r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Number((l+r).into()),
                    (Value::Bool(l), Value::Number(r)) => Value::Number(*l as f64 + r),
                    (Value::Number(l), Value::Bool(r)) => Value::Number(l + *r as f64),
                    _ => panic!("Operator '+' does not support those operands"),

                },
                "-" => match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Number((l-r).into()),
                    (Value::Bool(l), Value::Number(r)) => Value::Number(*l as f64 - r),
                    (Value::Number(l), Value::Bool(r)) => Value::Number(l - *r as f64),
                    _ => panic!("Operator '-' does not support those operands"),
                },
                "*" => match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Number((l*r).into()),
                    (Value::Bool(l), Value::Number(r)) => Value::Number(*l as f64 * r),
                    (Value::Number(l), Value::Bool(r)) => Value::Number(l * *r as f64),
                    _ => panic!("Operator '*' does not support those operands"),
                },
                "/" => match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                    (Value::Bool(l), Value::Bool(r)) => Value::Number((l/r).into()),
                    (Value::Bool(l), Value::Number(r)) => Value::Number(*l as f64 / r),
                    (Value::Number(l), Value::Bool(r)) => Value::Number(l / *r as f64),
                    _ => panic!("Operator '/' does not support those operands"),
                },
                op => panic!("Unknown operator: {}", op),
            }
        }

        ASTNode::FunctionCall(call) => evaluate_function_call(call, env),

        _ => panic!("Cannot evaluate node of type: {:?}", node),
    }
}

fn evaluate_function_call(call: &FunctionCallNode, env: &Environment) -> Value {
    let func = env
        .functions
        .get(&call.name)
        .unwrap_or_else(|| panic!("Function \"{}\" is not defined", call.name));

    let mut local_env = Environment::default();

    if call.arguments.len() != func.params.len() {
        panic!(
            "Function \"{}\" expects {} arguments, but {} were provided",
            func.name,
            func.params.len(),
            call.arguments.len()
        );
    }

    for (param, arg) in func.params.iter().zip(&call.arguments) {
        let value = evaluate(arg, env);
        if let ASTNode::VariableDeclaration(var_decl) = param {
            local_env.variables.insert(var_decl.name.clone(), value);
        }
    }

    let mut return_value = None;
    for stmt in &func.body {
        return_value = execute(stmt, &mut local_env);
        if return_value.is_some() {
            break;
        }
    }

    return_value.unwrap_or(Value::String("No return value".to_string()))
}

fn evaluate_kind_of(k: &KindOfNode, env: &Environment) -> Value {
    let result = evaluate(&k.expression, env);
    match result {
        Value::String(_) => Value::String("string".to_string()),
        Value::Number(_) => Value::String("number".to_string()),
        Value::Bool(_) => Value::String("bool".to_string()),
    }
}

// === Execution ===

fn execute(node: &ASTNode, env: &mut Environment) -> Option<Value> {
    match node {
        ASTNode::Print(p) => {
            let result = evaluate(&p.expression, env);
            match result {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
                Value::Bool(b) => {
                    if b == 1 {
                        println!("twue");
                    } else {
                        println!("fawse");
                    }
                },
            }
            None
        }

        ASTNode::OwO(p) => {
            let result = evaluate(&p.expression, env); // Get the evaluated expression result
            match result {
                Value::String(s) => {
                    let owo_result = owoify(&s); // Apply owoify to the string
                    println!("{}", owo_result);  // Print the "owoified" result
                }
                Value::Number(n) => {
                    println!("{}", n);  // If the result is a number, print it as is
                }
                Value::Bool(b) => {
                    if b == 1 {
                        println!("twue");
                    } else {
                        println!("fawse");
                    }
                }
                // _ => panic!("OwO only supports strings or numbers."),
            }
            None
        }

        ASTNode::KindOf(k) => Some(evaluate_kind_of(k, env)),

        ASTNode::FunctionCall(call) => Some(evaluate_function_call(call, env)),

        ASTNode::Return(ret) => Some(evaluate(&ret.value, env)),

        ASTNode::VariableDeclaration(decl) => {
            let value = evaluate(&decl.value, env);
            env.variables.insert(decl.name.clone(), value);
            None
        }

        _ => None,
    }
}
