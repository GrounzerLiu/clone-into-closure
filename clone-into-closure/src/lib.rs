extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Block, Expr, ExprBlock, ExprCall, ExprMethodCall, ExprPath, ItemFn, Local, LocalInit, Pat, PatIdent, PatTuple, Path, PathArguments, PathSegment, Stmt, Token};

#[proc_macro_attribute]
pub fn clone_into_closure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    input.block.stmts.iter_mut().for_each(|stmt| {
        iter_stmt(stmt);
    });
    input.into_token_stream().into()
}

fn iter_stmt(stmt: &mut Stmt) {
    match stmt {
        Stmt::Local(local) => {
            if let Some(init) = &mut local.init {
                iter_expr(&mut init.expr);
            }
        }
        Stmt::Item(_item) => {}
        Stmt::Expr(expr, _) => {
            iter_expr(expr);
        }
        Stmt::Macro(_mac) => {}
    }
}

fn iter_expr(expr: &mut Expr) {

    let mut stmts_and_closure = None;

    match expr {
        Expr::Array(array) => {
            array.elems.iter_mut().for_each(|e| iter_expr(e));
        }
        Expr::Assign(assign) => {
            iter_expr(&mut assign.left);
            iter_expr(&mut assign.right);
        }
        Expr::Async(async_expr) => {
            async_expr.block.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Await(await_expr) => {
            iter_expr(&mut await_expr.base);
        }
        Expr::Binary(binary) => {
            iter_expr(&mut binary.left);
            iter_expr(&mut binary.right);
        }
        Expr::Block(block) => {
            block.block.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Break(_break_expr) => {}
        Expr::Call(call) => {
            iter_expr(&mut call.func);
            call.args.iter_mut().for_each(|arg| {
                iter_expr(arg);
            });
        }
        Expr::Cast(cast) => {
            iter_expr(&mut cast.expr);
        }
        Expr::Closure(closure) => {
            let mut stmts = Vec::new();
            if let Some(Pat::Tuple(PatTuple{ elems, .. })) = closure.inputs.first_mut() {
                let mut clone_args: Vec<String> = Vec::new();
                for elem in elems.iter_mut() {
                    if let Pat::Ident(PatIdent { ident, .. }) = elem {
                        clone_args.push(ident.to_string());
                    } else {
                        clone_args.clear();
                        break;
                    }
                }

                if clone_args.len() > 0 {
                    let mut new_inputs: Punctuated<Pat, Token![,]> = Punctuated::new();
                    closure.inputs.iter().skip(1).for_each(|pat| {
                        new_inputs.push(pat.clone());
                    });
                    closure.inputs = new_inputs;


                    clone_args.iter().for_each(|arg| {
                        stmts.push(Stmt::Local(Local {
                            attrs: vec![],
                            let_token: Default::default(),
                            pat: Pat::Ident(PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: Some(Default::default()),
                                ident: Ident::new(&arg, Span::call_site()),
                                subpat: None,
                            }),
                            init: Some(LocalInit {
                                eq_token: Default::default(),
                                expr: Box::new(Expr::MethodCall(ExprMethodCall {
                                    attrs: vec![],
                                    receiver: Box::new(Expr::Path(ExprPath {
                                        attrs: vec![],
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut segments = Punctuated::new();
                                                segments.push(PathSegment {
                                                    ident: Ident::new(arg, Span::call_site()),
                                                    arguments: PathArguments::None,
                                                });
                                                segments
                                            },
                                        },
                                    })),
                                    dot_token: Default::default(),
                                    method: Ident::new("clone", Span::call_site()),
                                    turbofish: None,
                                    paren_token: Default::default(),
                                    args: Default::default(),
                                })),
                                diverge: None,
                            }),
                            semi_token: Default::default(),
                        }));
                    });
                }
            }

            if stmts.len() > 0 {
                stmts_and_closure = Some((stmts, closure.clone()));
            }else {
                iter_expr(&mut closure.body);
            }
        }
        Expr::Const(const_expr) => {
            const_expr.block.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Continue(_) => {}
        Expr::Field(field) => {
            iter_expr(&mut field.base);
        }
        Expr::ForLoop(for_loop) => {
            iter_expr(&mut for_loop.expr);
            for_loop.body.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Group(group) => {
            iter_expr(&mut group.expr);
        }
        Expr::If(if_expr) => {
            iter_expr(&mut if_expr.cond);
            if_expr.then_branch.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
            if let Some((_, else_branch)) = &mut if_expr.else_branch {
                iter_expr(else_branch);
            }
        }
        Expr::Index(index) => {
            iter_expr(&mut index.expr);
            iter_expr(&mut index.index);
        }
        Expr::Infer(_) => {}
        Expr::Let(let_expr) => {
            iter_expr(&mut let_expr.expr);
        }
        Expr::Lit(_) => {}
        Expr::Loop(loop_expr) => {
            loop_expr.body.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Macro(_) => {}
        Expr::Match(match_expr) => {
            iter_expr(&mut match_expr.expr);
            match_expr.arms.iter_mut().for_each(|arm| {
                if let Some((_, guard)) = &mut arm.guard {
                    iter_expr(guard);
                }
                iter_expr(&mut arm.body);
            });
        }
        Expr::MethodCall(method_call) => {
            iter_expr(&mut method_call.receiver);
            method_call.args.iter_mut().for_each(|arg| {
                iter_expr(arg);
            });
        }
        Expr::Paren(paren) => {
            iter_expr(&mut paren.expr);
        }
        Expr::Path(_) => {}
        Expr::Range(range) => {
            if let Some(start) = &mut range.start {
                iter_expr(start);
            }
            if let Some(end) = &mut range.end {
                iter_expr(end);
            }
        }
        Expr::RawAddr(raw_addr) => {
            iter_expr(&mut raw_addr.expr);
        }
        Expr::Reference(reference) => {
            iter_expr(&mut reference.expr);
        }
        Expr::Repeat(repeat) => {
            iter_expr(&mut repeat.expr);
        }
        Expr::Return(return_expr) => {
            if let Some(expr) = &mut return_expr.expr {
                iter_expr(expr);
            }
        }
        Expr::Struct(struct_expr) => {
            struct_expr.fields.iter_mut().for_each(|field| {
                iter_expr(&mut field.expr);
            });
            if let Some(rest) = &mut struct_expr.rest {
                iter_expr(rest);
            }
        }
        Expr::Try(try_expr) => {
            iter_expr(&mut try_expr.expr);
        }
        Expr::TryBlock(try_block) => {
            try_block.block.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Tuple(tuple) => {
            tuple.elems.iter_mut().for_each(|e| iter_expr(e));
        }
        Expr::Unary(unary) => {
            iter_expr(&mut unary.expr);
        }
        Expr::Unsafe(unsafe_expr) => {
            unsafe_expr.block.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Verbatim(_) => {}
        Expr::While(while_expr) => {
            iter_expr(&mut while_expr.cond);
            while_expr.body.stmts.iter_mut().for_each(|stmt| {
                iter_stmt(stmt);
            });
        }
        Expr::Yield(yield_expr) => {
            if let Some(expr) = &mut yield_expr.expr {
                iter_expr(expr);
            }
        }
        _ => {}
    }

    if let Some((mut stmts, closure)) = stmts_and_closure {
        stmts.push(
            Stmt::Expr(Expr::Closure(closure), Default::default())
        );
        let new_expr = Expr::Block(
            ExprBlock{
                attrs: vec![],
                label: None,
                block: Block {
                    brace_token: Default::default(),
                    stmts,
                },
            }
        );
        *expr = new_expr;
        if let Expr::Block(
            ExprBlock{
                block: Block {
                    stmts,
                    ..
                },
                ..
            }
        ) = expr {
            let last = stmts.last_mut().unwrap();
            if let Stmt::Expr(Expr::Closure(closure), _) = last {
                iter_expr(&mut closure.body);
            }
        }
    }
}