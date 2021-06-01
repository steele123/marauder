# mem-rs
mem-rs is a library that intends to make game hacking in rust easy so you don't need to deal with the bullshit of converting c++ types into rust types or dealing with unsafe keywords everywhere.

This library makes great use of Rust's features system to keep the library small and efficient

In the end I mostly just want to have this library to the point where you will never have to touch WINAPI in rust and you won't have to go through hell to setup a quite memory read and get all the amazing performance from rust itself.

I don't care how you use the library as long as it is useful for you. Use it in whatever you want.

## UPDATE 5/4/2021
I am still currently developing mem-rs but have been extremely busy as of lately and haven't been able to work as much 
as I have wanted to on mem-rs and the project is probably not usable in its current state YET but will be very very soon :)

## In Development
I am using mem-rs for my own usage personally and would like to have it shared on github because I couldn't find any memory libraries doing the same things that I wanted from them. The APIs may change a lot or they may change not at all just please keep that in mind...

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