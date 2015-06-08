# Let's Build an iOS App in Rust, part 1

This is the repository accompanying the [Let's Build an iOS App in Rust, part
1](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/) blog post.

## Building and Running

After cloning the repo, go into the `rust/hello` directory and run `make`. This
requires you to have a working Rust-to-iOS toolchain; see the blog post for
instructions on setting up `multirust` appropriately. (Don't forget to
`multirust override ios` this project directory.)

After `libhello.a` is built, open up
`ios/RustHelloWorld/RustHelloWorld.xcodeproj` and run the app.

## Author

John Gallagher, jgallagher@bignerdranch.com

## License

This sample app is available under the MIT license. See the LICENSE file for
more info.
