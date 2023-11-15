macro_rules! cfg_macros {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "macros")]
            $item
        )*
    }
}

#[allow(unused)]
macro_rules! cfg_injector {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "injector")]
            $item
        )*
    }
}

#[allow(unused)]
macro_rules! cfg_internal {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "internal")]
            $item
        )*
    }
}

#[allow(unused)]
macro_rules! cfg_external {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "external")]
            $item
        )*
    }
}
