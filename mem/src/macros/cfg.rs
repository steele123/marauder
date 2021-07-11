macro_rules! cfg_macros {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "macros")]
            $item
        )*
    }
}

macro_rules! cfg_injector {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "injector")]
            $item
        )*
    }
}

macro_rules! cfg_internal {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "internal")]
            $item
        )*
    }
}

macro_rules! cfg_external {
        ($($item:item)*) => {
        $(
            #[cfg(feature = "external")]
            $item
        )*
    }
}
