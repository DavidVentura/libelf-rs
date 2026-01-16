#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod error;
mod handle;
pub mod read;
pub mod types;
pub mod write;

// Re-export types
pub use handle::{Elf, Elf_Scn};
pub use types::*;

// Re-export C API functions so cbindgen can find them
pub use error::{elf_errmsg, elf_errno};
pub use read::*;
pub use write::*;
