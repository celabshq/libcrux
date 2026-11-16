/// Macro to simplify `kyber` feature gating
macro_rules! cfg_kyber {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "kyber")]
            #[cfg_attr(docsrs, doc(cfg(feature = "kyber")))]
            $item
        )*
    }
}

/// Macro to simplify `eurydice` cfg gating
#[allow(unused_macros)]
macro_rules! cfg_no_eurydice {
    ($($item:item)*) => {
        $(
            #[cfg(not(eurydice))]
            $item
        )*
    }
}
