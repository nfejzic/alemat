//! Elements found in the MathML Core Specification. These elements are called MathML Core
//! elements.

macro_rules! pub_mods {
    ($($mod:ident),* $(,)?) => {
        $(mod $mod;)*

        $(pub use $mod::*;)*
    };
}

pub_mods! {
    annotation,
    maction,
    math,
    merror,
    mfrac,
    mi,
    mmultiscripts,
    mn,
    mo,
    munderover,
    mpadded,
    mphantom,
}
