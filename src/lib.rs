//! This crate entroduces `when!` macro which provides better syntax for
//! `if`/`else if`/`else` chains. The syntax is similar to `match`.
//!
//! (idea is borrowed from [kotlin][kt-when-expr])
//!
//! [kt-when-expr]: https://kotlinlang.org/docs/reference/control-flow.html#when-expression
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

/// Better syntax for `if`/`else if`/`else` similar to `match` syntax
///
/// ## Examples
///
/// ```
/// use kiam::when;
///
/// #[derive(PartialEq)]
/// struct Struct {
///     x: i32,
/// }
///
/// fn cond0() -> bool { false }
///
/// let cond1 = true;
/// let zero = Struct { x: 0 };
/// let opt = None::<&'static str>;
///
/// let res = when! {
///     cond0() => 0,
///     // In difference with `if`, struct literals are allowed here
///     zero == Struct { x: 1 } => 1,
///     // Pattern matching
///     let Some(s) = opt => s.len() as _,
///     cond1 => {
///         /* blocks are allowed too */
///         17
///     },
///     // Default branch (`else`) (it's optional, unless `when!` is used in expression position)
///     _ => 42,
/// };
///
/// assert_eq!(res, 17);
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
        $(let $fpat:pat = )? $fcond:expr => $fbranch:expr
        $(
            , $(let $pat:pat = )? $cond:expr => $branch:expr
        )*
        $(, _ => $def_branch:expr)?
        $(,)?
    ) => {
        if $(let $fpat = )? $fcond {
            $fbranch
        }
        $(
            else if $(let $pat = )? $cond {
                $branch
            }
        )*
        $(
            else {
                $def_branch
            }
        )?
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
