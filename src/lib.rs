macro_rules! some_or {
    ($e:expr, $b:block) => {
        if let Some(x) = $e {
            x
        } else {
            $b
        }
    };
}

extern crate git2;
extern crate tempdir;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate rs_tracing;

extern crate pest;
extern crate rand;

pub mod base_repo;
pub mod cgi;
pub mod run_proxy;
pub mod scratch;
mod shell;
mod treeops;
pub mod views;
pub mod virtual_repo;

pub use run_proxy::*;
pub use scratch::*;
pub use shell::Shell;
pub use treeops::*;

use views::*;

#[derive(Clone)]
pub enum UnapplyView {
    Done(git2::Oid),
    RejectNoFF,
    RejectMerge,
    NoChanges,
}
