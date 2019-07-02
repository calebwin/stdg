# Standard Graphics
Standard Graphics is a universal rendering engine abstraction for 2D graphics. It provides a cross-platform, language-agnostic interface based on the `stdin` and `stdout` streams available on most operating systems. This allows you or your team to change developer environments (operating system, languages, stack, etc.) without having to change platform support (operating systems, hardware, back-end, etc.) and to change platform support without having to change developer environment. Standard Graphics is implemented in Rust with a binding to SDL2 and compiles to a safe, efficient binary with help from LLVM.

# Features

Standard Graphics currently provides a small but growing set of commands to print graphics and handle events.

- Draw lines, points
- Draw circles, squares, rectangles
- Draw with fill or outline
- Handle mouse events, touch events

This allows Standard Graphics to be used to create simple data visualizations, visual simulations, animations, etc.

# Examples

Here is an example of `stdg` being used by a Python program-
```python
while True:
	print("color 255 255 255")
	print("clear")
	print("color 255 0 0")
	print("rect 50 50 100 100")
	print("present")
	print("handle")
```

The graphics for this program can be rendered by piping the program's standard output to standard graphics.
```cmd
python draw_rectangle.py | stdg
```

This will open the following window to output the graphics.

![An example output through standard graphics](https://i.imgur.com/bPnUYoJ.png)

Here's another example. This example is taken from Khan Academy's "Making animations" lesson.

```python
# position of the car
x = 10

while True:
	print("color 151 244 247")
	print("clear")

	# draw the car body
	print("color 255 0 115")
	print("fill rect " + str(x) + " 200 100 20")
	print("fill rect " + str(x + 15) + " 178 70 40")

	# draw the wheels
	print("color 77 66 66")
	print("fill circle " + str(x + 25) + " 221 12")
	print("fill circle " + str(x + 75) + " 221 12")

	print("present")
	print("handle")

	x = x + 1
```

![An example output through standard graphics](https://i.imgur.com/aRbhapW.png)

# Usage

The easiest way to install Standard Graphics is by downloading and installing the binaries from [here](https://github.com/calebwin/stdg/releases/tag/v0.1.0).

You can also install Standard Graphics by building from source.
```console
$ curl https://sh.rustup.rs -sSf | sh
$ git clone https://www.github.com/calebwin/stdg
$ cd stdg
$ cargo install --path .
```
