#![allow(unused_variables)]
use std::{io::Write, path::{Path, PathBuf}, ffi::OsString};

use python_parser::ast::{CompoundStatement, Expression, Import, PyString, Statement, Funcdef, Argument, SetItem, Subscript, Bop, ComprehensionChunk, Try, Uop, AugAssignOp, Classdef};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut src = PathBuf::from(std::env::current_dir().unwrap());
    let mut dest = PathBuf::from(std::env::current_dir().unwrap());
    src.push("../execution-specs/src/ethereum/frontier/vm");
    dest.push("../rust-execution-specs/src/ethereum/frontier/vm");
    translate_dir(&src, &dest, 100)?;
    
    Ok(())
}

fn translate_dir(src: &Path, dest: &Path, max_depth: usize) -> Result<(), Error> {
    std::fs::DirBuilder::new().recursive(true).create(&dest).unwrap();
    let py = OsString::from("py");
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        // println!("entry={:?}", entry);
        let ft = entry.file_type()?;
        let fname = entry.file_name();
        let mut dest = dest.to_owned();
        dest.push(fname.clone());
        if ft.is_dir() && max_depth > 1 {
            std::fs::DirBuilder::new().recursive(true).create(&dest).unwrap();
            translate_dir(&entry.path(), &dest, max_depth - 1)?;
        } else if ft.is_file() && dest.extension() == Some(&py) {
            let file_name = if fname == "__init__.py" {
                "mod.rs"
            } else {
                fname.to_str().unwrap()
            };
            translate_file(&entry.path(), &dest.with_file_name(file_name).with_extension("rs"))?;
        }
    }
    Ok(())
}

fn translate_file(src: &Path, dest: &Path) -> Result<(), Error> {
    if dest.exists() {
        return Ok(());
    }
    eprintln!("{:?} -> {:?}", src, dest);
    let src = std::fs::read_to_string(src)?;

    let mut writer = std::fs::File::create(dest)?;
    let span = python_parser::make_strspan(&src);
    if let Ok((_span, ast)) = python_parser::file_input(span) {
        let statements = emit_doc_strings(&mut writer, "", &ast)?;

        emit_statements(&mut writer, "", statements)?;
    }

    Ok(())
}


fn emit_import<W : Write>(writer: &mut W, indent: &str, i: &Import) -> Result<(), Error> {
    match i {
        #![allow(unused_variables)]
        Import::ImportFrom {
            leading_dots,
            path,
            names,
        } => {
            let supers = (0..*leading_dots)
                .map(|_| "super".to_string())
                .collect::<Vec<_>>()
                .join("::");
            let path = path.join("::");
            let names = names
                .iter()
                .map(|s| s.0.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(writer, "{}use {}::{}::{{{}}};", indent, supers, path, names)?;
        }
        Import::ImportStarFrom { leading_dots, path } => {
            let supers = (0..*leading_dots)
                .map(|_| "super".to_string())
                .collect::<Vec<_>>()
                .join("::");
            let path = path.join("::");
            writeln!(writer, "{}use {}::{}::*;", indent, supers, path)?;
        }
        Import::Import { names } => {
            writeln!(writer, "{}// NOTE: Import::Import unsupported", indent)?;
        }
    }
    Ok(())
}

fn emit_assignment<W : Write>(writer: &mut W, indent: &str, exprs: &[Expression], rhs: &[Vec<Expression>]) -> Result<(), Error> {
    let exprs = tuple_or_var(exprs);
    if rhs.is_empty() {
        writeln!(writer, "{}{};", indent, exprs)?;
    } else {
        let rhs = rhs.iter().map(|e| e.iter().map(expr)).flatten().collect::<Vec<_>>().join(" = ");
        writeln!(writer, "{}{} = {};", indent, exprs, rhs)?;
    }
    Ok(())
}

fn emit_compound<W : Write>(writer: &mut W, indent: &str, compound: &CompoundStatement) -> Result<(), Error> {
    use CompoundStatement::*;
    match compound {
        If(a, b) => {
            let mut iff = "if";
            for (e, s) in a {
                writeln!(writer, "{}{} {} {{", indent, iff, expr(e))?;
                emit_statements(writer, &format!("{}    ", indent), s)?;
                iff = "} else if";
            }
            if let Some(s) = b {
                writeln!(writer, "{}}} else {{", indent)?;
                emit_statements(writer, &format!("{}    ", indent), &s)?;
            }
            writeln!(writer, "{}}}", indent)?;
        }
        For {
            r#async: bool,
            item,
            iterator,
            for_block,
            else_block,
        } => {
            writeln!(writer, "{}for {} in {} {{", indent, tuple_or_var(item), expr(iterator.first().unwrap()))?;
            emit_statements(writer, &format!("{}    ", indent), for_block)?;
            writeln!(writer, "{}}}", indent)?;
            if let Some(else_block) = else_block {
                writeln!(writer, "{}}} ELSE {{", indent)?;
                emit_statements(writer, &format!("{}    ", indent), else_block)?;
                writeln!(writer, "{}}}", indent)?;
            }
        },
        While(a, s, c) => {
            writeln!(writer, "{}while {} {{", indent, expr(a))?;
            emit_statements(writer, &format!("{}    ", indent), s)?;
            if let Some(else_block) = c {
                writeln!(writer, "{}}} ELSE {{", indent)?;
                emit_statements(writer, &format!("{}    ", indent), else_block)?;
                writeln!(writer, "{}}}", indent)?;
            }
            writeln!(writer, "{}}}", indent)?;

        }
        With(exprs, statements) => {
            let exprs = exprs.iter().map(|(a, b)| expr(a)).collect::<Vec<_>>().join(", ");
            writeln!(writer, "{}// with {}", indent, exprs)?;
            emit_statements(writer, &format!("{}    ", indent), &statements)?;
        }
        Funcdef(funcdef) => {
            emit_funcdef(writer, indent, funcdef)?;
            writeln!(writer, "\n")?;
        }
        Classdef(classdef) => {
            emit_classdef(writer, indent, classdef)?;
        }
        Try(t) => emit_try(writer, indent, t)?,
    }
    Ok(())
}

fn emit_funcdef<W : Write>(writer: &mut W, indent: &str, funcdef: &Funcdef) -> Result<(), Error> {
    if funcdef.name == "test_mainnet_genesis_config" {
        println!("HERE: {:?}", funcdef.code);
    }
    let statements = emit_doc_strings(writer, indent, &funcdef.code)?;
    let a = if funcdef.r#async { "async "} else {""};
    if !funcdef.decorators.is_empty() {
        writeln!(writer, "{}// NOTE: function has decorators", indent)?;
    }
    if !funcdef.parameters.posonly_args.is_empty() {
        writeln!(writer, "{}// NOTE: function has untyped args", indent)?;
    }
    let ret_type = if let Some(ret_type) = &funcdef.return_type {
        format!(" -> Result<{}, Error>", expr(ret_type))
    } else {
        " -> Result<(), Error>".to_string()
    };
    let params = funcdef.parameters.args.iter().map(|p| {
        if let (name, Some(e), _x) = p {
            format!("{}: {}", name, expr(e))
        } else {
            p.0.clone()
        }
    }).collect::<Vec<_>>().join(", ");
    writeln!(writer, "{}pub {}fn {}({}){} {{", indent, a, funcdef.name, params, ret_type)?;
    emit_statements(writer, &format!("{}    ", indent), statements)?;
    writeln!(writer, "{}}}", indent)?;
    Ok(())
}

fn emit_try<W : Write>(writer: &mut W, indent: &str, t: &Try) -> Result<(), Error> {
    // Try {
    //     try_block: todo!(),
    //     except_clauses: todo!(),
    //     last_except: todo!(),
    //     else_block: todo!(),
    //     finally_block: todo!(),
    // }
    writeln!(writer, "{}// Try ", indent)?;
    emit_statements(writer, &format!("{}    ", indent), &t.try_block)?;
    // writeln!(writer, "{}// Except ", indent)?;
    // emit_statements(writer, &format!("{}    ", indent), &t.except_clauses)?;
    Ok(())    
}

fn expr(e: &Expression) -> String {
    match e {
        Expression::Ellipsis => "...".to_string(),
        Expression::None => "()".to_string(),
        Expression::True => "true".to_string(),
        Expression::False => "false".to_string(),
        Expression::Name(n) => n.to_string(),
        Expression::Int(i) => i.to_string(),
        Expression::ImaginaryInt(_) => "/* ImaginaryInt unsupported */".to_string(),
        Expression::Float(f) => f.to_string(),
        Expression::ImaginaryFloat(_) => "/* ImaginaryFloat unsupported */".to_string(),
        Expression::String(s) => format!("{:?}", s.first().unwrap().content),
        Expression::Bytes(b) => format!("{:?}", b.as_slice()),
        Expression::DictLiteral(_) => "/* DictLiteral unsupported */".to_string(),
        Expression::SetLiteral(_) => "/* SetLiteral unsupported */".to_string(),
        Expression::ListLiteral(t) => {
            let args = t.iter().map(set_item).collect::<Vec<_>>().join(", ");
            format!("[{}]", args)
        }
        Expression::TupleLiteral(t) => {
            let args = t.iter().map(set_item).collect::<Vec<_>>().join(", ");
            format!("({})", args)
        }
        Expression::DictComp(_, _) => "/* DictComp unsupported */".to_string(),
        Expression::SetComp(_, _) => "/* SetComp unsupported */".to_string(),
        Expression::ListComp(_, _) => "/* ListComp unsupported */".to_string(),
        Expression::Generator(a, b) => generator(a, b),
        Expression::Await(_) => "/* Await unsupported */".to_string(),
        Expression::Call(a, b) => {
            let a = expr(a);
            let args = b.iter().map(|b| arg(b)).collect::<Vec<_>>().join(", ");
            format!("{}({})?", a, args)
        }
        Expression::Subscript(a, b) => format!("{}{}", expr(a), subscripts(b)),
        Expression::Attribute(a, b) => format!("{}.{}", expr(a), b),
        Expression::Uop(op, e) => uop(op, e),
        Expression::Bop(a, b, c) => bop(a, expr(b), c),
        Expression::MultiBop(a, b) => multibop(a, b),
        Expression::Ternary(_, _, _) => "/* Ternary unsupported */".to_string(),
        Expression::Yield(_) => "/* Yield unsupported */".to_string(),
        Expression::YieldFrom(_) => "/* YieldFrom unsupported */".to_string(),
        Expression::Star(_) => "/* Star unsupported */".to_string(),
        Expression::Lambdef(_, _) => "/* Lambdef unsupported */".to_string(),
        Expression::Named(_, _) => "/* Named unsupported */".to_string(),
    }
}

fn emit_statements<W : Write>(writer: &mut W, indent: &str, statements: &[Statement]) -> Result<(), Error> {
    for (i, statement) in statements.iter().enumerate() {
        // writeln!(writer, "{:?}", statement)?;
        let is_last = i == statements.len() - 1;
        match statement {
            #![allow(unused_variables)]
            python_parser::ast::Statement::Pass => writeln!(writer, "{}// pass;", indent)?,
            python_parser::ast::Statement::Del(x) => writeln!(writer, "{}// del {:?};", indent, x)?,
            python_parser::ast::Statement::Break => writeln!(writer, "{}break;", indent)?,
            python_parser::ast::Statement::Continue => writeln!(writer, "{}continue;", indent)?,
            python_parser::ast::Statement::Return(e) => {
                let args = e.iter().map(expr).collect::<Vec<_>>().join(", ");
                if is_last {
                    if e.len() == 1 {
                        writeln!(writer, "{}return Ok({});", indent, args)?;
                    } else {
                        writeln!(writer, "{}return Ok(({}));", indent, args)?;
                    }
                } else {
                    if e.len() == 1 {
                        writeln!(writer, "{}Ok({})", indent, args)?;
                    } else {
                        writeln!(writer, "{}Ok(({}))", indent, args)?;
                    }
                }
            }
            python_parser::ast::Statement::RaiseExcFrom(_, _) => writeln!(writer, "{}// RaiseExcFrom unsupported", indent)?,
            python_parser::ast::Statement::RaiseExc(e) => writeln!(writer, "{}return Err(Error::{});", indent, expr(e))?,
            python_parser::ast::Statement::Raise => writeln!(writer, "{}// Raise unsupported", indent)?,
            python_parser::ast::Statement::Global(_) => writeln!(writer, "{}// Global unsupported", indent)?,
            python_parser::ast::Statement::Nonlocal(_) => writeln!(writer, "{}// Nonlocal unsupported", indent)?,
            python_parser::ast::Statement::Assert(a, b) => {
                if let Some(b) = b {
                    writeln!(writer, "{}assert!({}, {});", indent, expr(a), expr(b))?;
                } else {
                    writeln!(writer, "{}assert!({});", indent, expr(a))?;
                }
            }
            python_parser::ast::Statement::Import(i) => emit_import(writer, indent, i)?,
            python_parser::ast::Statement::Expressions(_) => writeln!(writer, "{}// Expressions unsupported", indent)?,
            python_parser::ast::Statement::Assignment(exprs, rhs) => {
                emit_assignment(writer, indent, exprs, rhs)?;
            }
            python_parser::ast::Statement::TypeAnnotation(a, b) => {
                // for a in a {
                //     writeln!(writer, "{}{}: {},", indent, expr(a), expr(b))?;
                // }
            }
            python_parser::ast::Statement::TypedAssignment(_, _, _) => writeln!(writer, "{}// TypedAssignment unsupported", indent)?,
            python_parser::ast::Statement::AugmentedAssignment(lhs, op, rhs) => {
                emit_augmented_assignment(writer, indent, lhs, op, rhs)?;
            }
            python_parser::ast::Statement::Compound(c) => {
                emit_compound(writer, indent, &c)?;
            }
        }
    }
    Ok(())
}

fn emit_doc_string<W : Write>(writer: &mut W, indent: &str, doc_strings: &[PyString]) -> Result<(), Error> {
    for s in doc_strings {
        if let Some(s) = s.content.as_str() {
            let lines = s.split("\n").collect::<Vec<_>>();
            for l in lines {
                writeln!(writer, "{}/// {}", indent, l)?;
            }
        }
    }
    Ok(())
}

/// Convert the first statement to comments.
fn emit_doc_strings<'a, W : Write>(writer: &mut W, indent: &str, statements: &'a [Statement]) -> Result<&'a [Statement], Error> {
    Ok(if let Some(Statement::Assignment(comment, _)) = statements.first() {
        match &comment.as_slice() {
            &[Expression::String(doc_strings)] => {
                emit_doc_string(writer, indent, &doc_strings)?;
                // writeln!(writer, "doc_strings: {:?}", emit_doc_string(doc_strings)?)
                &statements[1..]
            }
            _ => statements,
        }
    } else {
        statements
    })
}

fn arg(arg: &Argument) -> String {
    match arg {
        Argument::Positional(p) => expr(p),
        Argument::Starargs(e) => format!("*{}", expr(e)),
        Argument::Keyword(n, t) => format!("{} = {}", n, expr(t)),
        Argument::Kwargs(_) => "/* kwargs? */".to_string(),
    }
}

fn set_item(item: &SetItem) -> String {
    match item {
        SetItem::Star(e) => format!("Star({})", expr(e)),
        SetItem::Unique(e) => expr(e),
    }
}

fn subscripts(s: &[Subscript]) -> String {
    s.iter().map(|s| {
        match s {
            Subscript::Simple(e) => format!("[{}]", expr(e)),
            Subscript::Double(a, b) => {
                let a = a.as_ref().map(expr).unwrap_or_default();
                let b = b.as_ref().map(expr).unwrap_or_default();
                format!("[{}..{}]", a, b)
            },
            Subscript::Triple(_, _, _) => "[/* Subscript::Triple */]".to_string(),
        }
    }).collect::<String>()
}

fn bop(bop: &Bop, l: String, r: &Expression) -> String {
    let opname = match bop {
        Bop::Add => "+",
        Bop::Sub => "-",
        Bop::Mult => "*",
        Bop::Matmult => "*",
        Bop::Mod => "%",
        Bop::Floordiv => return format!("({}).floordiv({})", l, expr(r)),
        Bop::Div => "/",
        Bop::Power => return format!("({}).pow({})", l, expr(r)),
        Bop::Lshift => "<<",
        Bop::Rshift => ">>",
        Bop::BitAnd => "&",
        Bop::BitXor => "^",
        Bop::BitOr => "|",
        Bop::Lt => "<",
        Bop::Gt => ">",
        Bop::Eq => "==",
        Bop::Leq => "<=",
        Bop::Geq => ">=",
        Bop::Neq => "!=",
        Bop::In => return format!("({}).contains({})", l, expr(r)),
        Bop::NotIn => return format!("!({}).contains({})", l, expr(r)),
        Bop::Is => return format!("({}).is({})", l, expr(r)),
        Bop::IsNot => return format!("!({}).is({})", l, expr(r)),
        Bop::And => "&&",
        Bop::Or => "||",
    };
    format!("{} {} {}", l, opname, expr(r))
}

fn multibop(e: &Expression, rhs: &[(Bop, Expression)]) -> String {
    let mut res = expr(e);
    for (b, r) in rhs {
        res = bop(b, res, r);
    }
    res
}

fn generator(s: &SetItem, g: &[ComprehensionChunk]) -> String {
    println!("{:?} {:?}", s, g);
    match &g[0] {
        ComprehensionChunk::If { cond } => "/* ComprehensionChunk::If unsupported */".to_string(),
        ComprehensionChunk::For { r#async, item, iterator } => {
            let items = item.iter().map(expr).collect::<Vec<_>>().join(", ");
            format!("{}.iter().map(|{}| {}).collect::<Vec<_>>()", expr(iterator), items, set_item(s))
        }
    }
}

fn tuple_or_var(e: &[Expression]) -> String {
    let args = e.iter().map(expr).collect::<Vec<_>>().join(", ");
    if e.len() == 1 {
        args
    } else {
        format!("({})", args)
    }
}

fn uop(op: &Uop, e: &Expression) -> String {
    match op {
        Uop::Plus => format!("+({})", expr(e)),
        Uop::Minus => format!("-({})", expr(e)),
        Uop::Invert => format!("!({})", expr(e)),
        Uop::Not => format!("!({})", expr(e)),
    }
}

fn emit_augmented_assignment<W : Write>(writer: &mut W, indent: &str, lhs: &[Expression], op: &AugAssignOp, rhs: &[Expression]) -> Result<(), Error> {
    let op = match op {
        AugAssignOp::Add => "+=",
        AugAssignOp::Sub => "-=",
        AugAssignOp::Mult => "*=",
        AugAssignOp::MatMult => "*=",
        AugAssignOp::Div => "/=",
        AugAssignOp::Mod => "%=",
        AugAssignOp::BitAnd => "&=",
        AugAssignOp::BitOr => "|=",
        AugAssignOp::BitXor => "^=",
        AugAssignOp::Lshift => "<<=",
        AugAssignOp::Rshift => ">>=",
        AugAssignOp::Power => "pow",
        AugAssignOp::Floordiv => "floordiv",
    };
    writeln!(writer, "{}{} {} {};", indent, tuple_or_var(lhs), op, tuple_or_var(rhs))?;
    Ok(())
}

fn emit_classdef<W : Write>(writer: &mut W, indent: &str, classdef: &Classdef) -> Result<(), Error> {
    if classdef.name == "GenesisConfiguration" {
        println!("{:?}", classdef);
    }
    // Classdef {
    //     decorators: todo!(),
    //     name: todo!(),
    //     arguments: todo!(),
    //     code: todo!(),
    // }

    let new_indent = format!("{}    ", indent);

    let statements = emit_doc_strings(writer, "", &classdef.code)?;
    writeln!(writer, "{}struct {} {{", indent, classdef.name)?;
    for a in &classdef.arguments {
        writeln!(writer, "{}base: {},", new_indent, arg(a))?;
    }
    for s in statements {
        match s {
            python_parser::ast::Statement::TypeAnnotation(a, b) => {
                for a in a {
                    writeln!(writer, "{}{}: {},", new_indent, expr(a), expr(b))?;
                }
            }
            _ => {}
        }
    }
    writeln!(writer, "{}}}\n\n", indent)?;

    writeln!(writer, "{}impl {} {{", indent, classdef.name)?;
    emit_statements(writer, &new_indent, statements)?;
    writeln!(writer, "{}}}\n\n", indent)?;

    Ok(())
}
