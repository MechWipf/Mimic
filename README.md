
Mimic
=====

A ComputerCraft emulator written in Rust, using the Java Native Interface to run Java.

Much of the heavy lifting (such as interfacing with JNI and rendering using OpenGL) is offloaded to custom APIs. The code here basically ties these two interfaces together. The JNI library is [here](https://github.com/GravityScore/jni-rs), and the Terminal library is [here](https://github.com/GravityScore/Terminal).

The JNI library makes use only of the Java Native Interface. The library is Mac only at the moment (mainly because I can't be bothered installing a Windows/Linux VM to test on other OSes), hence this whole project is Mac only. I'm pretty sure the OpenGL setup won't work on Windows also, but who knows.

Feel free to submit a pull request or issue if you have an improvement or feature request.

### To Do

* Windows and Linux support
* App Bundles
* ROM files
* Improve CPU usage
* Console API

### Usage

**Commands**

Mimic is operated entirely through keyboard shortcuts, as GLFW sadly doesn't allow me to create menu bars. There are a few commands, listed below. The command key on Mac is the command key, and on Windows and Linux it's the control key.


Command               | Description
--------------------- | ---------------------------------------------------------
`Command + n`    	    | Create a new advanced computer.
`Command + shift + n` | Create a new basic computer without colors.
`Command + b`         | Create a new advanced pocket computer.
`Command + shift + b` | Create a new basic pocket computer without colors.
`Commnad + a`         | Add a modem on the top of the currently focused computer.
`Command + r`         | Reboot the currently focused computer.
`Command + t`         | Terminate the current program in the focused computer.
`Command + s`         | Shutdown the currently focused computer.

**Files**

You can find Mimic's data, including computer files and configuration under:

* Mac: ~/Library/Application Support/mimic
* Linux: ~/.mimic
* Windows: %APPDATA%\mimic

The configuration is in the `config.json` file. The configuration options are:

Option            | Description
----------------- | ------------------------------------------------------------------------
`computer width`  | The width of a terminal window in cells (number, default 51).
`computer height` | The height of a terminal window in cells (number, default 19).
`pocket width`    | The width of a pocket computer window in cells (number, default 26).
`pocket height`   | The height of a pocket computer window, in cells (number, default 20).
`space limit`     | The space limit for computers, in bytes (number, default 2097152 - 2MB).

**ROM**

You can add your own files to the ROM by creating a `rom` directory in the data folder (paths above), and putting your programs in there. All files in the `rom` directory will be loaded into the `rom/programs` folder at runtime.

### Building

If you want to build the emulator by yourself, firstly install Rust. There's a simple command on the [Rust Guide](http://doc.rust-lang.org/guide.html) to do this:

```
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

Then install GLFW through its [website](http://www.glfw.org/). There's some install instructions on there - should be fairly simple to follow.

Make sure you also have Java installed (type `javac`), then `cd` into the project folder and run:

```
make
cargo run
```

That'll download and compile all the dependencies, compile Mimic itself, and (assuming everything was successful) run the emulator.

### License

The whole thing is licensed under the MIT license, meaning basically do whatever you want with the code.
