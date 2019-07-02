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
	print("fill circle " + str(x + 25) + " 221 12");
	print("fill circle " + str(x + 75) + " 221 12");

	print("present")
	print("handle")

	x = x + 1
```

![An example output through standard graphics](https://i.imgur.com/aRbhapW.png)
