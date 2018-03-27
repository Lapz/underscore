#[macro_use]
extern crate lazy_static;

extern crate underscore_syntax as syntax;
extern crate underscore_util as util;
// mod types;
mod constraints;
mod env;
// mod subst;
mod trans;
// mod tiger;
// mod types;
pub use constraints::Infer;
pub use env::Env as TypeEnv;
