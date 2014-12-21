
Mimic
=====

A ComputerCraft emulator written in Rust, using the Java Native Interface to run Java.

Much of the heavy lifting (such as interfacing with JNI and rendering using OpenGL) is offloaded to custom APIs. The code here basically ties these two interfaces together. The JNI library is [here](https://github.com/GravityScore/jni-rs), and the Terminal library is [here](https://github.com/GravityScore/Terminal).

The JNI library makes use only of the Java Native Interface. The library is Mac only at the moment (mainly because I can't be bothered installing a Windows/Linux VM to test on other OSes), hence this whole project is Mac only. I'm pretty sure the OpenGL setup won't work on Windows also, but who knows.

Feel free to submit pull requests or issues if you have an improvement or feature request.

### License

The whole thing is licensed under the MIT license, meaning basically do whatever you want with the code, although I would like credit for its use.
