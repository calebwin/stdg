<p align="center">
<!--   <img height="90px" src="https://i.imgur.com/7650OtP.png"/> -->
<!--   <img height="90px" src="https://i.imgur.com/tpdsIFX.png"/> -->
  <img height="45px" src="https://i.imgur.com/wxTHOMd.png"/> 
  <br>
  <img height="90px" src="https://i.imgur.com/VuokGxr.png"/>
<!--   <br> -->
<!--   <img height="45px" src="https://i.imgur.com/DpjP4aH.png"/> -->
  <br>
  <img height="45px" src="https://i.imgur.com/wxTHOMd.png"/> 
</p>

Standard Graphics (abbreviated as `stdg`) is a command-line tool for printing 2D graphics from any language to any screen. It uses `stdin` and `stdout` to make 2D graphics as simple as printing commands with Python's `print()` or Java's `System.out.println()` for example. You can even write plain text and handle user interaction.

You only need to learn `stdg` once. Then it will no longer matter what language or frameworks you happen to be using. You will always be able to easily print 2D graphics using the same easy-to-remember commands.

# features

- **Learn once** - `stdg` provides a minimal set of commands for 2D graphics. These commands are designed to be intuitive, easy-to-learn, and easy-to-remember. They are mostly inspired by Khan Academy's computer programming environment and Processing.
- **Use anywhere** - `stdg` is both cross-language and cross-platform. This means that you can freely switch between programming languages and operating systems without having to learn a totally new 2D graphics library for the language/OS.
- **Get fast, performant graphics** - `stdg` is an abstraction over MiniFB and Raqote which are both implemented entirely in Rust (except for MacOS which uses *some* Objective-C). Therefore, your graphics will be rendered at effectively native speed. There is no use of an Electron-like framework or "dynamic" languages in the rendering, event-handling.

# example

Standard Graphics can be used with plain text. As an example, you could save the following to a text file, `rectangle.txt`.

```txt
start 400 400 Untitled

background 255 255 255
fill 255 0 0
rect 50 50 100 100

present forever
```

If you have `stdg` and `python` correctly installed, you can open a terminal (command line/prompt), navigate to the folder containing `rectangle.txt` by using `cd`, type out the following, and press enter.

```cmd
rectangle.txt | stdg
```

This is what you should see.

![An example output through standard graphics](https://i.imgur.com/bPnUYoJ.png)

Standard Graphics enables you to do even more complex things. This next example is taken from Khan Academy's "Making animations" lesson. Since this program is written in Python, you will have to use the Python interpreter to run it.

```python
print("start 400 400 Untitled")


# position of the car
x = 10

while True:
  print("background 151 244 247")

  # draw the car body
  print("fill 255 0 115")
  print("nostroke")
  print("rect " + str(x) + " 200 100 20")
  print("rect " + str(x + 15) + " 178 70 40")

  # draw the wheels
  print("fill 77 66 66")
  print("circle " + str(x + 25) + " 221 12")
  print("circle " + str(x + 75) + " 221 12")

  print("present") # this is what actually says to draw, must be in an infinite loop

  x = x + 1
```

Enter the following in a terminal.

```cmd
python moving_car.py | stdg
```

You should see the following pop up as a window.

![An example output through standard graphics](https://i.imgur.com/aRbhapW.png)

Read on the see how you can use `stdg` for not only simple graphics and animations but also interactive user interfaces.

# usage

There are 2 ways of using Standard Graphics.

The first, and simplest, is by simply piping your program's output to `stdg`. All of your program's output (stuff that would normally be printed) will be sent to `stdg`. You can open a terminal and type in something like the following, provided `stdg` is installed.

```cmd
my_data.csv | ruby my_program.rb | stdg
```
For each line of input that `stdg` recieves, it will check if the first word (first token in line split by whitespace) matches a command. If the word matches, it will look at the rest of the line and execute the command by printing a rectangle, or setting color, etc.. If the word doesn't match, it will just print the line out.

The second way is by giving `stdg` a process to run.

```cmd
stdg node my_thing.js | settings.csv
```

In this case, `stdg` doesn't accept any input. You can't pipe anything to it. You can't interactively type stuff in the terminal. You can, however, give it a process to run.

You give it a process by providing arguments to `stdg`. These arguments get parsed into a single command that can be executed as a process. Basically you can write `stdg python options.py` or `stdg ./menu` but you can't do `stdg python options.py && ./menu`.

Then, `stdg` will run your process and read its output in exactly the same way the first usage has `stdg` reading. Commands get interpreted. Everything else gets printed as output. But now in addition to that, `stdg` will also sometimes print input to the process itself. This input will be information regarding things like mouse position, mouse click, etc.. The process can read this input one line at a time to get the mouse/key/etc. information.

This second way of using `stdg` allows for interactivity. You can do things like this-

```python
print("start 400 400 A Rectangle")

while True:
  print("background 255 255 255")
  print("fill 255 0 0")
  
  print("get mousex")
  print("get mousey")
  print("rect " + str(float(input()) - 25.0) + " " + str(float(input()) - 25.0) + " 50.0 50.0")
  print("present")
```

# cheat sheet

The following is a cheat sheet/reference for using `stdg`.

The following are commands for the most basic usage of `stdg`. Note that all whitespace in text like in `start` and `text` commands are converted to single spaces.

| Command                   | Example                     | Note                                 |
| ------------------------- | --------------------------- | ------------------------------------ |
| Start everything          | `start 400 400 A rectangle` | Must be first line printed           |
| Present stuff to be drawn | `present`                   | Must be in an infinite loop          |
| Present forever           | `present forever`           | Useful in `.txt` files               |
| Get position of mouse     | `get mousex`, `get mousey`  | Sends back line containing position  |
| Get "is mouse pressed?"   | `get mouseispressed left`   | Must be `left`, `center`, or `right` |
| Get "is key pressed?"     | `get keyispressed space`    | Valid keys listed below              |
| Get all keys pressed      | `get keys`                  | Sends space-seperated valid keys           |

The following are useful for styling.

| Command                    | Example                    | Note                                              |
| -------------------------- | -------------------------- | ------------------------------------------------- |
| Set background color       | `background 220 220 220`   |                                                   |
| Set fill color             | `fill 255 0 0 240`         | All values of red-green-blue-alpha are 0-255      |
| Set stroke color           | `stroke 20 20 20`          | Stroke is also called outline                     |
| Don't fill or don't stroke | `nostroke`, `nofill`       | Default color is `nofill`, `stroke 0 0 0`         |
| Set stroke weight          | `strokeweight 5`           | Default is 1                                      |
| Set stroke cap             | `strokecap round`          | Must be `square`, `project`, or `round` (default) |
| Set stroke join            | `strokejoin bevel`         | Must be `miter` (default), `bevel`, or  `round`   |

We can do transformations.

| Command               | Example          | Note                                       |
| --------------------- | ---------------- | ------------------------------------------ |
| Push a transformation | `push`           | This lets us begin a new transformation    |
| Pop top               | `pop`            | This will revert to the old transformation |
| Translate top         | `translate 50 0` |                                            |
| Scale top             | `scale 0.5 1.0`  |                                            |
| Rotate top            | `rotate 180`     | Degrees are in degrees                     |

There are the common 2D primitives.

| Command        | Example                  | Note                          |
| -------------- | ------------------------ | ----------------------------- |
| Draw rectangle | `rect 50 50 300 300`     | Must be x, y coordinates, width, height                              |
| Draw ellipse   | `ellipse 200 200 50 40`  | Centered at given coordinates |
| Draw circle    | `circle 200 200 50`      | All coordinates in 4th quadrant                              |
| Draw line      | `line 300 100 100 300`   |                               |
| Draw arc       | `arc 200 200 50 40 0 90` | Degrees are in degrees        |
| Draw polygon   | `poly 130 70 180 20 340 100 360 200 270 250 130 70` | Arbitrary number of points allowed |

Last but not least, we have text and images.

| Command       | Example                                   | Note                                    |
| ------------- | ----------------------------------------- | --------------------------------------- |
| Set text font | `textfont C:\Windows\Fonts\Arial.ttf`     | `\` might have to be `\\`               |
| Set text size | `textsize 20`                             |                                         |
| Draw text     | `text 30 30`                              | Must be followed by line with text      |
| Load image    | `open character.png as char`              | Should be called only once, if possible |
| Draw image    | `image char 30 70`, `image char 5 5 50 1` | Should be a loaded image                |

And here are the keys supported by `stdg`-

- All numeric characters
- All lower-case alphabetic characters (use `leftshift` or `rightshift` to check for upper-case)
- `up`, `down`, `left`, `right`
- `space`, `tab`, `enter`
- `leftshift`, `rightshift`
- `escape`, `backspace`, `delete`

# about

Standard Graphics is designed to be useful for many sorts of things-

- User interfaces for Bash scripts
- Visualization of Python-scripted simulations
- Visualization of data
- Desktop games written in JavaScript
- Simple vector graphics with plain text
- Simple animations with C
- *and much more...*

The software itself is written entirely in pure Rust with the only exception being the MacOS back-end. It uses [Raqote](https://github.com/jrmuizel/raqote) and [MiniFB](https://github.com/emoon/rust_minifb) behind the scenes for drawing stuff.

# getting started

There are two ways to install Standard Graphics.

The first way is to download the binaries from [here](https://github.com/calebwin/stdg/releases/tag/v0.2.0). Once downloaded, make sure that the folder location where the binaries are stored is added to `PATH` (look that up on the Internet if you aren't sure "how to add folder location to PATH". So basically what you would need to do is...

1. Download binary file (`stdg.exe` or `stdg`)
2. Move binary to desired folder (maybe `Program Files\stdg\stdg.exe` for example)
3. Add the desired folder to `PATH`.

The second way is to install Standard Graphics with `cargo`. Make sure you have [installed Rust](https://www.rust-lang.org/tools/install). Then, simply install as follows.
```cmd
cargo install stdg
```

During installation, you may have to install a bunch of packages. On Windows, I was personally able to simply install and run. However, on Linux, I had to install at least `libfontconfig1-dev`, `xcursor`.

Once installed, you can take a look at the cheat sheet for more information on the various commands you can print.
