// Not public API.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! punct_len {
    ($mode:ident, +)     => { 1 };
    ($mode:ident, +=)    => { 2 };
    ($mode:ident, &)     => { 1 };
    ($mode:ident, &&)    => { 2 };
    ($mode:ident, &=)    => { 2 };
    ($mode:ident, @)     => { 1 };
    ($mode:ident, !)     => { 1 };
    ($mode:ident, ^)     => { 1 };
    ($mode:ident, ^=)    => { 2 };
    ($mode:ident, :)     => { 1 };
    ($mode:ident, ::)    => { 2 };
    ($mode:ident, ,)     => { 1 };
    ($mode:ident, /)     => { 1 };
    ($mode:ident, /=)    => { 2 };
    ($mode:ident, .)     => { 1 };
    ($mode:ident, ..)    => { 2 };
    ($mode:ident, ...)   => { 3 };
    ($mode:ident, ..=)   => { 3 };
    ($mode:ident, =)     => { 1 };
    ($mode:ident, ==)    => { 2 };
    ($mode:ident, >=)    => { 2 };
    ($mode:ident, >)     => { 1 };
    ($mode:ident, <=)    => { 2 };
    ($mode:ident, <)     => { 1 };
    ($mode:ident, *=)    => { 2 };
    ($mode:ident, !=)    => { 2 };
    ($mode:ident, |)     => { 1 };
    ($mode:ident, |=)    => { 2 };
    ($mode:ident, ||)    => { 2 };
    ($mode:ident, #)     => { 1 };
    ($mode:ident, ?)     => { 1 };
    ($mode:ident, ->)    => { 2 };
    ($mode:ident, <-)    => { 2 };
    ($mode:ident, %)     => { 1 };
    ($mode:ident, %=)    => { 2 };
    ($mode:ident, =>)    => { 2 };
    ($mode:ident, ;)     => { 1 };
    ($mode:ident, <<)    => { 2 };
    ($mode:ident, <<=)   => { 3 };
    ($mode:ident, >>)    => { 2 };
    ($mode:ident, >>=)   => { 3 };
    ($mode:ident, *)     => { 1 };
    ($mode:ident, -)     => { 1 };
    ($mode:ident, -=)    => { 2 };
    ($mode:ident, ~)     => { 1 };
    (lenient, $tt:tt)    => { 0 };
    (strict, $tt:tt)     => {{ unexpected!($tt); 0 }};
    ($mode:ident, $head:tt $($tail:tt)+) => {
        punct_len!($mode, $head) $(+ punct_len!($mode, $tail))+
    };
}

// Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! unexpected {
    () => {};
}

// Not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! stringify_punct {
    ($($tt:tt)+) => {
        concat!($(stringify!($tt)),+)
    };
}

// Not public API.
// Without this, local_inner_macros breaks when looking for concat!
#[doc(hidden)]
#[macro_export]
macro_rules! my_concat {
    ($($tt:tt)*) => {
        concat!($($tt)*)
    };
}

/// Define a type that supports parsing and printing a multi-character symbol
/// as if it were a token.
///
/// # Usage
///
/// As a convention, it is recommended that this macro be invoked within a
/// module called `punct` or `punctuation` and that the resulting parser be invoked
/// with a `punct::` or `punctuation::` prefix.
///
/// ```edition2018
/// mod punct {
///     syn::custom_punctuation!(LeftRightArrow, <=>);
/// }
/// ```
///
/// The generated syntax tree node supports the following operations just like
/// any built-in punctuation token.
///
/// - [Peeking] — `input.peek(punct::LeftRightArrow)`
///
/// - [Parsing] — `input.parse::<punct::LeftRightArrow>()?`
///
/// - [Printing] — `quote!( ... #left_right_arrow ... )`
///
/// - Construction from a [`Span`] — `let left_right_arrow = punct::LeftRightArrow(sp)`
///
/// - Construction from multiple [`Span`] — `let left_right_arrow = punct::LeftRightArrow([sp, sp, sp])`
///
/// - Field access to its spans — `let spans = left_right_arrow.spans`
///
/// [Peeking]: parse/struct.ParseBuffer.html#method.peek
/// [Parsing]: parse/struct.ParseBuffer.html#method.parse
/// [Printing]: https://docs.rs/quote/0.6/quote/trait.ToTokens.html
/// [`Span`]: struct.Span.html
///
/// # Example
///
/// ```edition2018
/// use syn::{Expr, ExprParen, parenthesized};
/// use syn::punctuated::Punctuated;
/// use syn::parse::{Parse, ParseStream, Result};
/// use syn::token::Paren;
///
/// mod punct {
///     syn::custom_punctuation!(PathSeparator, </>);
/// }
///
/// // (expr) </> (expr) </> (expr) ...
/// struct PathSegments {
///     segments: Punctuated<Expr, punct::PathSeparator>,
/// }
///
/// impl Parse for PathSegments {
///     fn parse(input: ParseStream) -> Result<Self> {
///         let mut segments = Punctuated::new();
///
///         let la = input.lookahead1();
///         if la.peek(Paren) {
///             let content;
///             let paren_token = parenthesized!(content in input);
///             let expr = Box::new(content.parse()?);
///             segments.push_value(Expr::Paren(ExprParen { attrs: vec![], paren_token, expr }));
///         } else {
///             return Err(la.error());
///         }
///
///         while input.peek(punct::PathSeparator) {
///             segments.push_punct(input.parse()?);
///             let content;
///             let paren_token = parenthesized!(content in input);
///             let expr = Box::new(content.parse()?);
///             segments.push_value(Expr::Paren(ExprParen { attrs: vec![], paren_token, expr }));
///         }
///
///         Ok(PathSegments { segments })
///     }
/// }
///
/// let _: PathSegments = syn::parse_str(r#"("five") </> ("hundred")"#).unwrap();
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {
        pub struct $ident {
            pub spans: [$crate::export::Span; punct_len!(lenient, $($tt)+)],
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub fn $ident<__S: $crate::export::IntoSpans<[$crate::export::Span; punct_len!(lenient, $($tt)+)]>>(
            spans: __S,
        ) -> $ident {
            let _punct_len = punct_len!(strict, $($tt)+);
            $ident {
                spans: $crate::export::IntoSpans::into_spans(spans)
            }
        }

        impl $crate::export::Default for $ident {
            fn default() -> Self {
                $ident($crate::export::Span::call_site())
            }
        }

        impl_parse_for_custom_punctuation!($ident, $($tt)+);
        impl_to_tokens_for_custom_punctuation!($ident, $($tt)+);
        impl_clone_for_custom_punctuation!($ident, $($tt)+);
        impl_extra_traits_for_custom_punctuation!($ident, $($tt)+);
    };
}

// Not public API.
#[cfg(feature = "parsing")]
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_parse_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {
        impl $crate::token::CustomToken for $ident {
            fn peek(cursor: $crate::buffer::Cursor) -> bool {
                $crate::token::parsing::peek_punct(cursor, stringify_punct!($($tt)+))
            }

            fn display() -> &'static $crate::export::str {
                my_concat!("`", stringify_punct!($($tt)+), "`")
            }
        }

        impl $crate::parse::Parse for $ident {
            fn parse(input: $crate::parse::ParseStream) -> $crate::parse::Result<$ident> {
                let spans: [$crate::export::Span; punct_len!(lenient, $($tt)+)] =
                    $crate::token::parsing::punct(input, stringify_punct!($($tt)+))?;
                Ok($ident(spans))
            }
        }
    };
}

// Not public API.
#[cfg(not(feature = "parsing"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_parse_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {};
}

// Not public API.
#[cfg(feature = "printing")]
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_to_tokens_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {
        impl $crate::export::ToTokens for $ident {
            fn to_tokens(&self, tokens: &mut $crate::export::TokenStream2) {
                $crate::token::printing::punct(stringify_punct!($($tt)+), &self.spans, tokens)
            }
        }
    };
}

// Not public API.
#[cfg(not(feature = "printing"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_to_tokens_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {};
}

// Not public API.
#[cfg(feature = "clone-impls")]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_clone_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {
        impl $crate::export::Copy for $ident {}

        impl $crate::export::Clone for $ident {
            fn clone(&self) -> Self {
                *self
            }
        }
    };
}

// Not public API.
#[cfg(not(feature = "clone-impls"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_clone_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {};
}

// Not public API.
#[cfg(feature = "extra-traits")]
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_extra_traits_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {
        impl $crate::export::Debug for $ident {
            fn fmt(&self, f: &mut $crate::export::Formatter) -> $crate::export::fmt::Result {
                $crate::export::Formatter::write_str(f, stringify_punct!($($tt)+))
            }
        }

        impl $crate::export::Eq for $ident {}

        impl $crate::export::PartialEq for $ident {
            fn eq(&self, _other: &Self) -> $crate::export::bool {
                true
            }
        }

        impl $crate::export::Hash for $ident {
            fn hash<__H: $crate::export::Hasher>(&self, _state: &mut __H) {}
        }
    };
}

// Not public API.
#[cfg(not(feature = "extra-traits"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_extra_traits_for_custom_punctuation {
    ($ident:ident, $($tt:tt)+) => {};
}