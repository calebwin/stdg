# Standard Graphics
Standard Graphics is the universal rendering engine for 2D graphics. It provides a cross-platform, language-agnostic interface based on the `stdin` and `stdout` streams available on most operating systems. Here is an example of `stdg` being used by a Python program-
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

![An example output through standard graphics](https://i.imgur.com/SMYhphg.png)

This projects is currently best described as a prototype but there is active progress being made towards supporting more 2D primitives as well as full-blown support for event handling.
