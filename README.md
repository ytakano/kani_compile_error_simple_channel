# Version

```
$ rustc --version
rustc 1.71.0-nightly (c4190f2d3 2023-05-07)
$ cargo kani --version
cargo-kani 0.27.0
```

# Error log

```
$  RUST_BACKTRACE=1 cargo kani
   Compiling kani_async v0.1.0 (/home/ytakano/kani_async)
warning: field `waker_receiver` is never read
 --> src/unbounded.rs:8:5
  |
6 | struct Channel<T> {
  |        ------- field in this struct
7 |     queue: VecDeque<T>,
8 |     waker_receiver: Option<Waker>,
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

thread 'rustc' panicked at 'Error in struct_expr; value type does not match field type.
        StructTag("tag-_7931425818576372193::DirectFields")
        [Field { name: "generator_field_1", typ: CInteger(SizeT) }, Field { name: "generator_field_0", typ: Pointer { typ: StructTag("tag-_2670222561860564476") } }, Field { name: "case", typ: Unsignedbv { width: 8 } }]
        [Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_1::self" }, typ: Pointer { typ: StructTag("tag-_2670222561860564476") }, location: None, size_of_annotation: None }, Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_2::data" }, typ: CInteger(SizeT), location: None, size_of_annotation: None }, Expr { value: IntConstant(0), typ: Unsignedbv { width: 8 }, location: None, size_of_annotation: None }]', cprover_bindings/src/goto_program/expr.rs:809:9
stack backtrace:
error: internal compiler error: Kani unexpectedly panicked at panicked at 'Error in struct_expr; value type does not match field type.
                                    StructTag("tag-_7931425818576372193::DirectFields")
                                    [Field { name: "generator_field_1", typ: CInteger(SizeT) }, Field { name: "generator_field_0", typ: Pointer { typ: StructTag("tag-_2670222561860564476") } }, Field { name: "case", typ: Unsignedbv { width: 8 } }]
                                    [Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_1::self" }, typ: Pointer { typ: StructTag("tag-_2670222561860564476") }, location: None, size_of_annotation: None }, Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_2::data" }, typ: CInteger(SizeT), location: None, size_of_annotation: None }, Expr { value: IntConstant(0), typ: Unsignedbv { width: 8 }, location: None, size_of_annotation: None }]', cprover_bindings/src/goto_program/expr.rs:809:9.

   0: rust_begin_unwind
             at /rustc/9aa5c24b7d763fb98d998819571128ff2eb8a3ca/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/9aa5c24b7d763fb98d998819571128ff2eb8a3ca/library/core/src/panicking.rs:64:14
   2: cprover_bindings::goto_program::expr::Expr::struct_expr_from_values
   3: kani_compiler::codegen_cprover_gotoc::codegen::rvalue::<impl kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_rvalue
   4: kani_compiler::codegen_cprover_gotoc::codegen::statement::<impl kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_statement
   5: kani_compiler::codegen_cprover_gotoc::codegen::function::<impl kani_compiler::codegen_cprover_gotoc::context::goto_ctx::GotocCtx>::codegen_function
   6: std::thread::local::LocalKey<T>::with
   7: <kani_compiler::codegen_cprover_gotoc::compiler_interface::GotocCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
   8: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
   9: rustc_interface::passes::start_codegen
  10: <rustc_middle::ty::context::GlobalCtxt>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  11: <rustc_interface::queries::Queries>::ongoing_codegen
  12: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  13: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  14: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

Kani unexpectedly panicked during compilation.
Please file an issue here: https://github.com/model-checking/kani/issues/new?labels=bug&template=bug_report.md

[Kani] current codegen item: codegen_function: unbounded::Sender::<usize>::send
_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_
[Kani] current codegen location: Loc { file: "/home/ytakano/kani_async/src/unbounded.rs", function: None, start_line: 26, start_col: Some(5), end_line: 26, end_col: Some(66) }
error: could not compile `kani_async`; 2 warnings emitted
error: Failed to compile `kani_async` due to an internal compiler error.: error: internal compiler error: Kani unexpectedly panicked at panicked at 'Error in struct_expr; value type does not match field type.
                                    StructTag("tag-_7931425818576372193::DirectFields")
                                    [Field { name: "generator_field_1", typ: CInteger(SizeT) }, Field { name: "generator_field_0", typ: Pointer { typ: StructTag("tag-_2670222561860564476") } }, Field { name: "case", typ: Unsignedbv { width: 8 } }]
                                    [Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_1::self" }, typ: Pointer { typ: StructTag("tag-_2670222561860564476") }, location: None, size_of_annotation: None }, Expr { value: Symbol { identifier: "_RNvMs_NtCs4eKqxUCtJjz_10kani_async9unboundedINtB4_6SenderjE4sendB6_::1::var_2::data" }, typ: CInteger(SizeT), location: None, size_of_annotation: None }, Expr { value: IntConstant(0), typ: Unsignedbv { width: 8 }, location: None, size_of_annotation: None }]', cprover_bindings/src/goto_program/expr.rs:809:9.
```
