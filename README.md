
Mimic
=====

A ComputerCraft emulator written in Rust, using the Java Native Interface to run Java.

Much of the heavy lifting (such as interfacing with JNI and rendering using OpenGL) is offloaded to custom APIs. The code here basically ties these two interfaces together. The JNI library is [here](https://github.com/GravityScore/jni-rs), and the Terminal library is [here](https://github.com/GravityScore/Terminal).

The JNI library makes use only of the Java Native Interface. The library is Mac only at the moment (mainly because I can't be bothered installing a Windows/Linux VM to test on other OSes), hence this whole project is Mac only. I'm pretty sure the OpenGL setup won't work on Windows also, but who knows.

I also ignore all non-ASCII characters. Passing these between Rust (which supports Unicode quite well), into C (which doesn't), into Java (which I have no idea about) is just too painful. I've simply ignored any non-English characters, so typing one doesn't do anything.

Feel free to submit a pull request or issue if you have an improvement or feature request.

### To Do

* Windows and Linux support
* Fix memory leak
* Icon
* Screenshot functionality
* Customizable computer folder location
* Move default folder location to ~/Documents
* Remove config.json from being included in ROM

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

Add your own files to ROM by putting them inside the `programs` folder in the Mimic data folder (the paths to the data folder are listed above). These programs are also live-updating, so changes to the files are reflected in Mimic while it's running. This makes for an easy way of getting files onto all of the computers without having to do anything!

### Building

1. Install Rust
2. Install GLFW
3. Install Java
4. Download and install ComputerCraft.jar
5. Compile Java code
6. Compile Rust code
7. (Optionally) create the application bundles

_Install Rust_

There's a simple command on the [Rust Guide](http://doc.rust-lang.org/guide.html) to do this:

```
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

_Install GLFW_

Do this through its [website](http://www.glfw.org/). There's some install instructions on there - should be fairly simple to follow.

_Install Java_

If you're wanting to build this, I'm going to assume you've already done this. If not, Google. You can check you've got Java installed by typing `javac` into the command line. It should spurt out a bunch of configuration options.

_Download and install ComputerCraft.jar_

Go download the latest version of ComputerCraft from [the website](http://www.computercraft.info/). As of writing, it's ComputerCraft 1.65. Rename it to `computercraft.jar`, create a folder called `Resources` in the root of the git repository, and put it in there.

_Compile Java_

`cd` into the git repository and run `make`. This will build all the Java code and put it into `Resources/mimic.jar`.

_Compile Rust_

`cd` into the git repository and run `cargo run`. This will download and compile all the dependencies, compile Mimic itself, and (assuming everything was successful) run the emulator.

_Create App Bundles_

An app bundle on OSX is a .app executable file. The basically combine all the project resources into one easy to run package. You can create a `Mimic.app` bundle by running `make osxbundle`. This relies on the previous compilation steps in order to work, so make sure everything is installed and you can compile Mimic without this command first.

### License

The whole thing is licensed under the MIT license, meaning basically do whatever you want with the code.
