# kiam

[![CI status](https://github.com/WaffleLapkin/kiam/workflows/Continuous%20integration/badge.svg)](https://github.com/WaffleLapkin/kiam/actions)
[![crates.io](https://img.shields.io/crates/v/kiam)](https://crates.io/crates/kiam)
[![documentation (docs.rs)](https://docs.rs/kiam/badge.svg)](https://docs.rs/kiam)
[![documentation (master)](https://img.shields.io/badge/docs-master-blue)](https://kiam-rs.netlify.com)
[![LICENSE](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)


_("kiam" is "when" in Esperanto)_

This crate entroduces `when!` macro which provides better syntax for 
`if`/`else if`/`else` chains. The syntax is similar to `match`.

(idea is borrowed from [kotlin][kt-when-expr])

```toml
[dependencies] 
kiam = "0.1"
```

[kt-when-expr]: https://kotlinlang.org/docs/reference/control-flow.html#when-expression
