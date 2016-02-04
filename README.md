# rustlab: Interfacing Matlab and Rust

This is a demonstration/experiment in how to use the Rust FFI to create a C library and use that in Matlab through it's support for external C libraries.

So far demonstrations for the following are included:
* pass a c-style string to a rust library from Matlab
* add two doubles (f64s) in rust library
* from Matlab: create a struct in a rust library, hold a pointer to it, and pass the pointer back to free the struct instance in memory
* pass a Matlab matrix to rust library as a flat array, manipulate it, and see the results in Matlab

## Rust Language References
* [Creating a C API for a Rust library](http://www.joshmatthews.net/blog/2015/10/creating-a-c-api-for-a-rust-library/)
* [Writing Unsafe and Low-Level Code in Rust](http://smallcultfollowing.com/rust-int-variations/isize-usize/guide-unsafe.html)
* [Stack Overflow: Answer to "Understanding Pointer Types in Rust"](http://stackoverflow.com/a/31953048/1217063)
* [Calling a Rust library from C](http://mainisusuallyafunction.blogspot.com/2014/08/calling-rust-library-from-c-or-anything.html)
* [Using Rust objects from other languages](http://jakegoulding.com/rust-ffi-omnibus/objects/)
* [Creating a Rust Dynamic Library](http://oppenlander.me/articles/rust-ffi)

## Matlab – Rust Interfacing References
* [Allowing Matlab to Talk to Rust - Elliot Smith](http://smitec.io/2016/02/04/allowing-matlab-to-talk-to-rust.html)

## Matlab – C Interface References
* [Call C Shared Libraries](http://www.mathworks.com/help/matlab/using-c-shared-library-functions-in-matlab-.html)
* [Pass Pointers](http://www.mathworks.com/help/matlab/matlab_external/pass-pointers.html)
* [Pass Arrays](http://www.mathworks.com/help/matlab/matlab_external/pass-arrays.html)
* [A Few Words About FFIs](http://matlabsadness.tumblr.com/post/81146476827/a-few-words-about-ffis)
* [Serializing/deserializing Matlab data](http://undocumentedmatlab.com/blog/serializing-deserializing-matlab-data)
