//! Library surface of the Oxide developer CLI.
//!
//! The crate ships a binary (`oxide`) that is a thin argument-parsing and
//! dispatch shell over this library. Exposing the modules here means the
//! integration tests in `tests/` exercise the same code the binary runs,
//! rather than re-including individual source files with `#[path]` attributes.
//!
//! A previous version of the test suite pulled `src/commands/*.rs` in directly
//! via `#[path]`, which meant `main.rs`, `cli.rs`, `doctor/*` and `process/*`
//! had no coverage at all in the real binary, and a shared module used by two
//! commands could not be reached from a test without a hand-written stub.

pub mod cli;
pub mod commands;
pub mod common;
pub mod doctor;
pub mod exit;
pub mod process;
