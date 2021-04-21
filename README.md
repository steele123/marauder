# mem-rs
mem-rs is a library that intends to make game hacking in rust easy so you don't need to deal with the bullshit of converting c++ types into rust types or dealing with unsafe keywords everywhere.

## Important
This library makes great use of Rust's features system to keep the library small and efficient

### Rust Features
```toml
[features]
# If the library will be used for an external cheat.
external = []
# If the library will be used for an internal cheat.
internal = []
# If the library will be to create an injector.
injector = []
```


## Plans
Currently I plan on making this project have anything you would need so you have no need to touch c++ or windows APIs
I would like for this to do as follows
* Process injection (x86 or x64)
* Support for internal and external hacks (x86 or x64)
* Keep the library small using features so people can decide what they need
* Have all of the required WINAPI functions that this library uses open for your use just in case you want to use them yourself
* Support ImGUI for drawing graphics along with providing hooks for Direct3d / Vulkan

## Features
* Wrapped WINAPI functions that the library uses