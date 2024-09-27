# `py-tauri` 🦀🐍
This repo shows how you can embed a Python interpreter, both statically and dynamically, into a Tauri project. The interpreter can then be used to call some python code that is shipped with the app. In this case, call a function that adds 2 numbers together.

## Dynamic vs Static linking
Dynamically linking to Python leads to a smaller executable by linking to a Python shared library. This automatically resolves paths to things like the Python standard library and site packages associated to that installation. The drawback is that the end user must have that exact version of Python (and the site packages) installed in their machines. This is not ideal for non-technical users who just want to download and install a program.

Statically embedding Python leads to a larger executable but means that end users do not need to have Python installed on their machine to run the program 👍. The **massive** drawback is that you have to find a way to package all the other parts of Python, like the standard library and site packages, into the build as well. Building Python for static linking only seems to give access to the interpreter, but you won't be able to do `import math`, for example 👎.

## Building Python
1. Download the [Python source code](https://www.python.org/downloads/source/).
2. Unzip the archive `cd` into the directory.
3. In the source code directory, run the following commands to build for dynamic linking:
```bash
CPPFLAGS="-I$(brew --prefix openssl@3)/include" LDFLAGS="-L$(brew --prefix openssl@3)/lib" ./configure --with-openssl=$(brew --prefix openssl@3) --enable-optimizations --enable-shared --prefix=$HOME/CustomPython
make -j4
make install
```
For this to work you will need to have `openssl` installed on your machine. This is necessary for your compiled `pip` to be able to get packages using `ssl`.

After running these commands, you will have a new Python installed in your home directory under `CustomPython`. In `CustomPython` you'll see 2 directories (`Python3.X` and `pkgconfig`) and a file called `libpython3.X` (`.so` on Linux, `.dylib` on MacOS, or `.dll` on Windows) which will be your shared library to link to the program.

To build for a static embed, replace `--enable-shared` in the command above with `--enable-shared=no`. Now you will get a file called `libpython3.X.a` on Linux and MacOS. Static linking is not done for Windows, see [here](https://pyo3.rs/v0.22.3/building-and-distribution#statically-embedding-the-python-interpreter) for more information.

## Build and run `py-tauri`
1. To build this app, run the following command at the root of this repo:
```bash
CUSTOM_PYTHON=$HOME/CustomPython/lib cargo tauri build
```

2. Run the app:
```bash
./src-tauri/target/release/py-tauri
```

## Help wanted 🙏
If anyone seeing this has any ideas about how to fix the static embedding so that it can see the standard library and site packages, I'd really appreciate the feedback 😊