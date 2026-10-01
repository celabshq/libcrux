/// This is a (partial) mirror of [`core::arch`]
pub mod x86;
pub use x86 as x86_64;
pub mod arm;
pub use arm as aarch64;
