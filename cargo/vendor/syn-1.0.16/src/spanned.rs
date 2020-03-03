//! A trait that can provide the `Span` of the complete contents of a syntax
//! tree node.
//!
//! *This module is available if Syn is built with both the `"parsing"` and
//! `"printing"` features.*
//!
//! <br>
//!
//! # Example
//!
//! Suppose in a procedural macro we have a [`Type`] that we want to assert
//! implements the [`Sync`] trait. Maybe this is the type of one of the fields
//! of a struct for which we are deriving a trait implementation, and we need to
//! be able to pass a reference to one of those fields across threads.
//!
//! [`Type`]: ../enum.Type.html
//! [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
//!
//! If the field type does *not* implement `Sync` as required, we want the
//! compiler to report an error pointing out exactly which type it was.
//!
//! The following macro code takes a variable `ty` of type `Type` and produces a
//! static assertion that `Sync` is implemented for that type.
//!
//! ```
//! # extern crate proc_macro;
//! #
//! use proc_macro::TokenStream;
//! use proc_macro2::Span;
//! use quote::quote_spanned;
//! use syn::Type;
//! use syn::spanned::Spanned;
//!
//! # const IGNORE_TOKENS: &str = stringify! {
//! #[proc_macro_derive(MyMacro)]
//! # };
//! pub fn my_macro(input: TokenStream) -> TokenStream {
//!     # let ty = get_a_type();
//!     /* ... */
//!
//!     let assert_sync = quote_spanned! {ty.span()=>
//!         struct _AssertSync where #ty: Sync;
//!     };
//!
//!     /* ... */
//!     # input
//! }
//! #
//! # fn get_a_type() -> Type {
//! #     unimplemented!()
//! # }
//! ```
//!
//! By inserting this `assert_sync` fragment into the output code generated by
//! our macro, the user's code will fail to compile if `ty` does not implement
//! `Sync`. The errors they would see look like the following.
//!
//! ```text
//! error[E0277]: the trait bound `*const i32: std::marker::Sync` is not satisfied
//!   --> src/main.rs:10:21
//!    |
//! 10 |     bad_field: *const i32,
//!    |                ^^^^^^^^^^ `*const i32` cannot be shared between threads safely
//! ```
//!
//! In this technique, using the `Type`'s span for the error message makes the
//! error appear in the correct place underlining the right type.
//!
//! <br>
//!
//! # Limitations
//!
//! The underlying [`proc_macro::Span::join`] method is nightly-only. When
//! called from within a procedural macro in a nightly compiler, `Spanned` will
//! use `join` to produce the intended span. When not using a nightly compiler,
//! only the span of the *first token* of the syntax tree node is returned.
//!
//! In the common case of wanting to use the joined span as the span of a
//! `syn::Error`, consider instead using [`syn::Error::new_spanned`] which is
//! able to span the error correctly under the complete syntax tree node without
//! needing the unstable `join`.
//!
//! [`syn::Error::new_spanned`]: crate::Error::new_spanned

use proc_macro2::Span;
use quote::spanned::Spanned as ToTokens;

/// A trait that can provide the `Span` of the complete contents of a syntax
/// tree node.
///
/// This trait is automatically implemented for all types that implement
/// [`ToTokens`] from the `quote` crate, as well as for `Span` itself.
///
/// [`ToTokens`]: quote::ToTokens
///
/// See the [module documentation] for an example.
///
/// [module documentation]: self
///
/// *This trait is available if Syn is built with both the `"parsing"` and
/// `"printing"` features.*
pub trait Spanned {
    /// Returns a `Span` covering the complete contents of this syntax tree
    /// node, or [`Span::call_site()`] if this node is empty.
    ///
    /// [`Span::call_site()`]: proc_macro2::Span::call_site
    fn span(&self) -> Span;
}

impl<T: ?Sized + ToTokens> Spanned for T {
    fn span(&self) -> Span {
        self.__span()
    }
}
