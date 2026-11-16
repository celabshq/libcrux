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
    (for $chunk:ident in $values:ident.chunks_exact($($chunk_size:expr),*) $body:block) => {
        for _cloop_i in 0..$values.len() / ($($chunk_size)*) {
            let $chunk = &$values[_cloop_i*($($chunk_size)*) .. _cloop_i*($($chunk_size)*)+($($chunk_size)*)];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $val:ident.iter().enumerate() $body:block) => {
        for $i in 0..$val.len() {
            let $item = &$val[$i];
            $body
        }
    };
    (for $item:ident in $val:ident.iter() $body:block) => {
        for _cloop_j in 0..$val.len() {
            let $item = &$val[_cloop_j];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $self:ident.$val:ident.iter().enumerate() $body:block) => {
        for $i in 0..$self.$val.len() {
            let $item = &$self.$val[$i];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $val:ident.into_iter().enumerate() $body:block) => {
        for $i in 0..$val.len() {
            let $item = $val[$i];
            $body
        }
    };
    (for ($i:ident, $item:ident) in $val:ident.$values:ident.into_iter().enumerate() $body:block) => {
        for $i in 0..$val.$values.len() {
            let $item = $val.$values[$i];
            $body
        }
    };
    (for $item:ident in $val:ident.$values:ident.into_iter() $body:block) => {
        for _cloop_k in 0..$val.$values.len() {
            let $item = $val.$values[_cloop_k];
            $body
        }
    };
    (for $i:ident in ($start:literal..$end:expr).step_by($step:literal) $body:block) => {
        for $i in ($start..($end / $step + 1)) {
            let $i = $i * $step;
            if $i >= $end { break; }
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
    (for $chunk:ident in $values:ident.chunks_exact($($chunk_size:expr),*) $body:block) => {
        for $chunk in $values.chunks_exact($($chunk_size),*) $body
    };
    (for ($i:ident, $item:ident) in $val:ident.iter().enumerate() $body:block) => {
        for ($i, $item) in $val.iter().enumerate() $body
    };
    (for $item:ident in $val:ident.iter() $body:block) => {
        for $item in $val.iter() $body
    };
    (for ($i:ident, $item:ident) in $self:ident.$val:ident.iter().enumerate() $body:block) => {
        for ($i, $item) in $self.$val.iter().enumerate() $body
    };
    (for ($i:ident, $item:ident) in $val:ident.into_iter().enumerate() $body:block) => {
        for ($i, $item) in $val.into_iter().enumerate() $body
    };
    (for ($i:ident, $item:ident) in $val:ident.$values:ident.into_iter().enumerate() $body:block) => {
        for ($i, $item) in $val.$values.into_iter().enumerate() $body
    };
    (for $item:ident in $val:ident.$values:ident.into_iter() $body:block) => {
        for $item in $val.$values.into_iter() $body
    };
    (for $i:ident in ($start:literal..$end:expr).step_by($step:literal) $body:block) => {
        for $i in ($start..$end).step_by($step) $body
    };
}

pub(crate) use cloop;
