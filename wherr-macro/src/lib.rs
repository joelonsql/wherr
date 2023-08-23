//! `wherr-macro` crate provides a procedural macro to enhance Rust errors with file and line number information.
//!
//! When using the provided `wherr` macro attribute on a function, any error returned by the `?` operator within that function
//! is automatically wrapped to include the file and line number where the error occurred.

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, visit_mut::VisitMut, Expr};

/// Procedural macro attribute that processes a function to automatically wrap errors using the `?` operator
/// with file and line number details.
#[proc_macro_attribute]
pub fn wherr(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as syn::ItemFn);

    let mut visitor = WherrVisitor;
    visitor.visit_item_fn_mut(&mut function);

    TokenStream::from(quote! { #function })
}

/// Visitor used by the `wherr` procedural macro to traverse and mutate the Abstract Syntax Tree (AST) of the function.
///
/// This visitor specifically looks for expressions using the `?` operator and wraps them with additional
/// file and line information.
struct WherrVisitor;

impl VisitMut for WherrVisitor {
    /// Visit expressions in the AST.
    ///
    /// Specifically, it targets the use of the `?` operator to wrap the error with file and line number details.
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // Check if the expression is a `?` usage. If it is, wrap it with our `wherrapper` function.
        if let Expr::Try(expr_try) = expr {
            let span = expr_try.question_token.span();
            let inner_expr = &expr_try.expr;
            let new_expr = syn::parse2(quote_spanned! { span=>
                wherr::wherrapper(#inner_expr, file!(), line!())?
            })
            .expect("Failed to create wherr wrapped expression");

            *expr = new_expr;
        } else {
            // Only continue visiting child expressions if it's not a Try expression
            syn::visit_mut::visit_expr_mut(self, expr);
        }
    }
}
