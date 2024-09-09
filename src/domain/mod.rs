//src/domain/mod.rs

mod user;
pub(crate) mod user_tests;
pub use user::*;
mod user_domain_errors;
pub use user_domain_errors::*;

