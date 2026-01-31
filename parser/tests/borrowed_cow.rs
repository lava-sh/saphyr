//! Deprecated: borrowing tests now live in `scanner.rs` unit tests.
//!
//! Borrowing for `Parser` events is only safe when the value is a verbatim slice of the input.
//! Plain scalars may legally fold/normalize across line breaks in both block and flow contexts,
//! so scanner-level tests provide better coverage for the borrowable token types (anchors,
//! aliases, and `%TAG` directive parts).
