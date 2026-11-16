/// The following macros are defined so that the extraction from Rust to C code
/// can go through.

#[cfg(eurydice)]
macro_rules! cloop {
    (for ($i:ident, $chunk:ident) in $val:ident.$values:ident.chunks_exact($($chunk_size:expr),*).enumerate() $body:block) => {
        for $i in 0..$val.$values.len() / ($($chunk_size)*) {
            let $chunk = &$val.$values[$i*($($chunk_size)*) .. $i*($($chunk_size)*)+($($chunk_size)*)];
            $body
        }
    };
    (for ($i:ident, $chunk:ident) in $val:ident.chunks_exact($($chunk_size:expr),*).enumerate() $body:block) => {
        for $i in 0..$val.len() / ($($chunk_size)*) {
            let $chunk = &$val[$i*($($chunk_size)*) .. $i*($($chunk_size)*)+($($chunk_size)*)];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $val:ident.iter().enumerate() $body:block) => {
        for $i in 0..$val.len() {
            let $item = &$val[$i];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $val:ident.into_iter().enumerate() $body:block) => {
        for $i in 0..$val.len() {
            let $item = $val[$i];
            $body
        }
    };
    (for $i:ident in ($start:literal..$end:expr).step_by($step:literal) $body:block) => {
        for $i in $start..$end / $step {
            let $i = $i * $step;
            $body
        }
    };
}

#[cfg(not(eurydice))]
macro_rules! cloop {
    (for ($i:ident, $chunk:ident) in $val:ident.$values:ident.chunks_exact($($chunk_size:expr),*).enumerate() $body:block) => {
        for ($i, $chunk) in $val.$values.chunks_exact($($chunk_size),*).enumerate() $body
    };
    (for ($i:ident, $chunk:ident) in $val:ident.chunks_exact($($chunk_size:expr),*).enumerate() $body:block) => {
        for ($i, $chunk) in $val.chunks_exact($($chunk_size),*).enumerate() $body
    };
    (for ($i:ident, $item:ident) in $val:ident.iter().enumerate() $body:block) => {
        for ($i, $item) in $val.iter().enumerate() $body
    };
    (for ($i:ident, $item:ident) in $val:ident.into_iter().enumerate() $body:block) => {
        for ($i, $item) in $val.into_iter().enumerate() $body
    };
    (for $i:ident in ($start:literal..$end:expr).step_by($step:literal) $body:block) => {
        for $i in ($start..$end).step_by($step) $body
    };
}

pub(crate) use cloop;
