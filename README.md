
Mimic
=====

A ComputerCraft emulator written in Rust, using the Java Native Interface to run Java.

Much of the heavy lifting (such as interfacing with JNI and rendering using OpenGL) is offloaded to custom APIs. The code here basically ties these two interfaces together. The JNI library is [here](https://github.com/GravityScore/jni-rs), and the Terminal library is [here](https://github.com/GravityScore/Terminal).

The JNI library makes use only of the Java Native Interface. The library is Mac only at the moment (mainly because I can't be bothered installing a Windows/Linux VM to test on other OSes), hence this whole project is Mac only. I'm pretty sure the OpenGL setup won't work on Windows also, but who knows.

Feel free to submit a pull request or issue if you have an improvement or feature request.

### Usage

Mimic is operated entirely through keyboard shortcuts, as GLFW sadly doesn't allow me to create menu bars. There are a few commands, listed below. The command key on Mac is the command key, and on Windows and Linux it's the control key.

**Commands**

Command               | Description
--------------------- | -------------
`Command + n`    	    | Create a new advanced computer (with colors).
`Command + shift + n` | Create a new basic computer (without colors).
`Commnad + m`         | Add a modem on the top of the currently focused computer.
`Command + r`         | Reboot the currently focused computer.
`Command + t`         | Terminate the current program in the focused computer.
`Command + s`         | Shutdown the currently focused computer.

### Building

If you want to build the emulator by yourself, firstly install Rust. There's a simple command on [The Rust Guide](http://doc.rust-lang.org/guide.html) to do this:

```bash
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

Then `cd` into the project folder and run:

```bash
cargo run
```

That'll download and compile all the dependencies, compile Mimic itself, and (assuming everything was successful) run the emulator.

### License

The whole thing is licensed under the MIT license, meaning basically do whatever you want with the code.
