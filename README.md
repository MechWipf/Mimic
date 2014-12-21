
Mimic
=====

A ComputerCraft emulator written in Rust, using the Java Native Interface to run Java.

Much of the heavy lifting (such as interfacing with JNI and rendering using OpenGL) is offloaded to custom APIs. The code here basically ties these two interfaces together. The JNI library is [here](https://github.com/GravityScore/jni-rs), and the Terminal library is [here](https://github.com/GravityScore/Terminal).

The JNI library makes use only of the Java Native Interface. The library is Mac only at the moment (mainly because I can't be bothered installing a Windows/Linux VM to test on other OSes), hence this whole project is Mac only. I'm pretty sure the OpenGL setup won't work on Windows also, but who knows.

Feel free to submit a pull request or issue if you have an improvement or feature request.

### Building

If you want to build the emulator by yourself, firstly install Rust. There's a simple command on [The Rust Guide](http://doc.rust-lang.org/guide.html) to do this:

```bash
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

Then `cd` into the project folder and run:

```bash
cargo run
```

That'll download and build all the dependencies and (assuming everything's up to date) run the emulator.

### License

The whole thing is licensed under the MIT license, meaning basically do whatever you want with the code, although I would like credit for its use.
