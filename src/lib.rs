//! _("kiam" is "when" in Esperanto)_
//!
//! This crate introduces [`when!`] macro which provides better syntax for
//! `if`/`else if`/`else` chains. The syntax is similar to `match`.
//!
//! (idea is borrowed from [kotlin][kt-when-expr])
//!
//! [kt-when-expr]: https://kotlinlang.org/docs/reference/control-flow.html#when-expression
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs, broken_intra_doc_links)]

/// Better syntax for `if`/`else if`/`else` similar to `match` syntax
///
/// ## Usage
///
/// Usage is similar to the usage of `match`, but instead of patterns, branches are guarded by boolean expression:
///
/// ```rust
/// kiam::when! {
///     false => (),
///     true => (),
///     // ...
/// }
/// ```
///
/// `_` can be used as a default branch (it's also required to use `when!` in expression position):
///
/// ```rust
/// let x = kiam::when! {
///     false => 0,
///     _ => 1,
/// };
///
/// assert_eq!(x, 1);
/// ```
///
/// You can also use `let <pat> =` to match a pattern, but in difference with `match` you'll have to provide an expression for every pattern:
///
/// ```rust
/// let a = None;
/// let b = Some(17);
/// let fallible = || Err(());
///
/// let x = kiam::when! {
///     let Some(x) = a => x,
///     let Ok(x) = fallible() => x,
///     let Some(x) = b => (x as u32) + 1,
///     _ => 1,
/// };
///
/// assert_eq!(x, 18);
/// ```
///
/// Last notes:
/// - You can also compare structure literals without brackets (you can't do this with `if`/`else if`/`else` chain)
/// - You can mixup boolean-branches with pattern matching
/// - Only one branch is executed (not to be confused with `switch` in C-like languages)
///
/// ```rust
/// let mut x = 0;
///
/// kiam::when! {
///     let Ok(_) = Err::<(), _>(()) => x = 1,
///     true => x = 2,
///     true => x = 3,
///     let Some(n) = Some(42) => x = n,
/// };
///
/// assert_eq!(x, 2);
/// ```
///
/// ```compile_fail
/// #[derive(PartialEq)]
/// struct Struct { a: i32 }
///
/// // This does not compile because of the ambiguity
/// if Struct { a: 0 } == Struct { a: 0 } {
///     // ...
/// }
/// ```
///
/// ```rust
/// #[derive(PartialEq)]
/// struct Struct { a: i32 }
///
/// kiam::when! {
///     // This, on the other hand, compiles fine
///     Struct { a: 0 } == Struct { a: 0 } => {
///         // ...
///     },
/// }
/// ```
///
/// ## Grammar
///
/// ```text
/// grammar:
///                   ╭───────────────>────────────────╮  ╭────>────╮
///                   │                                │  │         │
/// │├──╭── line ──╮──╯── "," ── "_" ── "=>" ── expr ──╰──╯── "," ──╰──┤│
///     │          │
///     ╰── "," ───╯
///
/// line:
///     ╭─────────────>─────────────╮
///     │                           │
/// │├──╯── "let"/i ── pat ── "=" ──╰── expr ── "=>" ── expr ──┤│
/// ```
#[macro_export]
macro_rules! when {
    (
        $(
            $(let $pat:pat = )? $cond:expr => $branch:expr
        ),+
        $(, _ => $def_branch:expr)?
        $(,)?
    ) => {
        $(
            if $(let $pat = )? $cond {
                $branch
            } else
        )+
        {
            $(
                $def_branch
            )?
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let r = when! {
            false => 0,
            true => 1,
            true => 2,
            _ => 42,
        };

        assert_eq!(r, 1);
    }

    #[test]
    fn pattern() {
        let r = when! {
            let Some(x) = None => x,
            let Some(y) = Some(12) => y + 1,
            _ => 0,
        };

        assert_eq!(r, 13);
    }

    #[test]
    fn mixed() {
        let r = when! {
            false => 0,
            let Some(_) = None::<bool> => 0,
            let Some(y) = Some(12) => y + 1,
            true => 1,
            _ => 0,
        };

        assert_eq!(r, 13);
    }

    #[test]
    fn r#struct() {
        #[derive(PartialEq)]
        struct Struct {
            x: i32,
        }

        // won't work with if/elseif/else
        #[allow(clippy::eq_op)]
        let r = when! {
            Struct { x: 0 } == Struct { x: 1 } => 0,
            Struct { x: 22 } == Struct { x: 22 } => 1,
            _ => 2,
        };

        assert_eq!(r, 1);
    }

    #[test]
    fn no_def() {
        let mut x = 0;

        when! {
            false => x = 18,
            true => x += 1,
        }

        assert_eq!(x, 1);
    }
}
