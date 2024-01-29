// check-pass
// compile-flags: --document-private-items

//! Regression test for an ICE: <https://github.com/rust-lang/rust/issues/120444>

mod webdavfs {
    pub struct WebDavFS;
}

/// [`Vfs`]
pub use webdavfs::WebDavFS;

pub struct Vfs;
