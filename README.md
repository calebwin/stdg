# standard graphics

Standard Graphics is a command-line tool for printing 2D graphics from any language to any screen. It uses `stdin` and `stdout` to make 2D graphics as simple as printing commands with Python's `print()` or Java's `System.out.println()` for example. You can even write plain text and handle user interaction.

You only need to learn `stdg` once. Then it will no longer matter what language or frameworks you happen to be using. You will always be able to easily print 2D graphics using the same easy-to-remember commands.

# features

- **Learn once** - a simple new API for 2D graphics
- **Use anywhere** - cross-language and cross-platform
- **Get fast, performant graphics** - renders at native speed

# example

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
```cmd
python moving_car.py | stdg
```

![An example output through standard graphics](https://i.imgur.com/aRbhapW.png)

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

Then, `stdg` will run your process and read its output in exactly the same way the first usage has `stdg` reading. Commands get interpreted. Everything else gets printed as output. But now in addition to that, `stdg` will also sometimes print input to the process. This input will be information regarding things like mouse position, mouse click, etc.. 

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

# about

Standard Graphics is designed to be useful for many sorts of things-

- User interfaces for Bash scripts
- Visualization of Python-scripted simulations
- Visualization of data
- Desktop games written in JavaScript
- Simple vector graphics with plain text
- Simple animations with C
- and much more...

The software itself is written entirely in pure Rust with the only exception being the MacOS back-end. It uses [Raquote](https://github.com/jrmuizel/raqote) and [MiniFB](https://github.com/emoon/rust_minifb) behind the scenes for drawing stuff.

# getting started

The easiest way to install Standard Graphics is by downloading and installing the binaries from [here](https://github.com/calebwin/stdg/releases/tag/v0.1.0).

You can also install Standard Graphics by building from source.
```console
$ curl https://sh.rustup.rs -sSf | sh
$ git clone https://www.github.com/calebwin/stdg
$ cd stdg
$ cargo install --path .
```
