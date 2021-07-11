/// Makes a function with a pointer to a memory address pretty useful for when
/// you need to call a function that isn't yours.
///
/// # Example:
///
/// ## Target Functions
/// ```rust
/// // Target Function 1
/// fn hello_world(hello: &str) {
///     println!(hello);
/// }
///
/// // Target Function 2
/// fn void_no_params_function() {
///     println!("Hello World!");
/// }
///
/// // Target Function 3
/// fn return_type_function() -> bool { true }
/// ```
///
/// ```rust
/// // Target 1
/// let hello_world_void_function = make_fn!(0x1337, (), "Hello World");
/// // Target 2
/// let void_no_params_function = make_fn!(0x6969);
/// // Target 3
/// let return_type_function = make_fn!(0x6913, bool);
/// ```
#[macro_export]
macro_rules! make_fn {
    ($address:expr) => {
        unsafe { std::mem::transmute::<*const usize, fn()>($address as *const usize) }
    };
    ($address:expr, $returntype:ty) => {
        unsafe { std::mem::transmute::<*const usize, fn() -> $returntype>($address as *const usize) }
    };
    ($address:expr, $returntype:ty, $($argument:ty), *) => {
        unsafe { std::mem::transmute::<*const usize, fn($($argument,)*) -> $returntype>($address as *const usize) }
    };
}

/// Makes a function with a pointer that has no return type.
///
/// # Safety:
/// This function is very unsafe extremely unsafe due to the use of
/// std::mem::transmute. Make sure you know what you are doing so you don't get
/// a access violation which will crash your process.
///
/// # Example:
///
/// ## Target Functions
/// ```rust
/// // Target Function 1
/// fn hello_world(hello: &str) {
///     println!(hello);
/// }
///
/// // Target Function 2
/// fn void_function() {
///     println!("Hello World!");
/// }
/// ```
///
/// ```rust
/// // Target 1
/// let hello_world = make_void!(0x1337, "Hello World");
/// // Target 2, this is the same as using make_fn!(0x6969);
/// let void_function = make_void!(0x6969);
/// ```
#[macro_export]
macro_rules! make_void {
    ($address:expr) => {
        unsafe { std::mem::transmute::<*const usize, fn()>($address as *const usize) }
    };
    ($address:expr, $($argument:ty), *) => {
        unsafe { std::mem::transmute::<*const usize, fn($($argument,)*)>($address as *const usize) }
    };
}

/// Makes working with pointers a bit easier since they can be a pain.
///
/// # Example:
///
/// ```rust
/// ptr!(0x1337, usize) = 1337 
/// ```
#[macro_export]
macro_rules! ptr {
    ($address:expr, $type:ty) => {
        *($address as *mut $type)
    };
}
