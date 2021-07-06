# mem-rs
mem-rs aims to provide amazing support for internal/external process injection. As nice as game hacking in c++ can be, 
I believe it would be even better inside of rust because you have things like macros to make a lot less boilerplate code.
The main issue with game hacking in rust is without a library you need a lot of boilerplate code for every single project
that you create. We can fully get rid of that issue and make things that required maybe 200 lines of boilerplate code that
in maybe just 20 or so lines which is pretty amazing. You also won't need to deal with windows API functions which most 
people hate doing. In c++ using a library for every game hacking project sucks, but in rust its perfect, it simplifies
all of it, and you can opt in for things you want, so you code have a bunch of random things being compiled also you don't
have to deal with the pain of aiding libraries in c++ because rust does libraries much better.

## Priority
Currently, internal mem-rs is at the very bottom of the priority list, the injector is very high priority as well as internal
mem-rs stuff.

## In Development
I am using mem-rs for my own usage personally and would like to have it shared on GitHub because I couldn't find any memory 
libraries doing the same things that I wanted from them. The APIs may change a lot, or they may change not at all just 
please keep that in mind...

## Plans
Currently, I plan on making this project have anything you would need, so you have no need to touch c++ or windows APIs
I would like for this to do as follows
* Process injection (x86 or x64)
* Support for internal and external hacks (x86 or x64)
* Keep the library small using features so people can decide what they need
* Have all the required WINAPI functions that this library uses open for your use just in case you want to use them yourself
* Support EGUI(Rust version of ImGUI) for drawing graphics along with providing hooks for Direct3d / Vulkan