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

Here is an example of `stdg` being used by a Ruby program-
```ruby
puts 'window 400 400'
puts 'title Untitled'

loop do
	puts 'color 255 255 255'
	puts 'clear'
	puts 'color 255 0 0'
	puts 'rect 50 50 100 100'
	puts 'present'
	puts 'handle'
end
```

The graphics for this program can be rendered by piping the program's output to stdg instead of stdout.
```cmd
ruby draw_rectangle.rb | stdg
```

<!--You can save the following to a text file.

```txt
window 400 400
title Untitled

color 255 255 255
clear
color 255 0 0
rect 50 50 100 100

present
handle forever
```

Instead of invoking a language interpreter such as `ruby` or `python`, you can pipe the contents of the file itself to stdg.

```cmd
draw_rectangle.txt | stdg
```
--->

This will open the following window to output the graphics.

![An example output through standard graphics](https://i.imgur.com/bPnUYoJ.png)

Here's another example. This example is taken from Khan Academy's "Making animations" lesson.  Since this program is written in Python, you will have to use the Python interpreter to run it.

```python
print("window 400 400")
print("title Untitled")


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
```cmd
python moving_car.py | stdg
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

# Commands

color [Red] [Green] [Blue]

Sets the color


`[fill | outline] square [x] [y] [side length]`

Draws a square from the its left corner


`[fill | outline] rect [x] [y] [width] [height]`

Draws a rectangle from its top left corner


`line [x1] [y1] [x2] [y2]`

Draws a line between (x1, y1) and (x2, y2)


`[fill | outline] circle [x] [y] [radius]`

Draws a circle from its center


`point [x] [y]`

Draws a point


`clear`

clears the screen


`present`

updates the screen


`handle`

handles input and prints keypresses to stdout


`window [width] [height]`

creates a window 


`title [title]`

titles the window
