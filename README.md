# Let's Build an iOS App in Rust, part 1

This is the repository accompanying the [Let's Build an iOS App in Rust, part
1](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/) blog post.

## Building and Running

After cloning the repo, go into the `rust/hello` directory and run `make`. This
requires you to have a working Rust-to-iOS toolchain. Make sure you have the `stable` toolchain selected when running `make`.

After `libhello.a` is built, open up
`ios/RustHelloWorld/RustHelloWorld.xcodeproj` and run the app.

## Setting up a cross-compile toolchain for Rust

Install https://www.rustup.rs/ and run:

```sh
$ rustup default stable
$ rustup target install aarch64-apple-ios
$ rustup target install armv7-apple-ios
$ rustup target install armv7s-apple-ios
$ rustup target install i386-apple-ios
$ rustup target install x86_64-apple-ios
````

## Author

John Gallagher, jgallagher@bignerdranch.com

## License

This sample app is available under the MIT license. See the LICENSE file for
more info.
