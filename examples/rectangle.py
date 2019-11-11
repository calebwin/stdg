# this file is super messy
# it looks really complicated
# it's only purpose is to show most, if not all, of the functionality of stdg

import math

print("start 400 400 A Rectangle")

print("textfont C:\\Windows\\Fonts\\Arial.ttf")
print("textsize 20")
print("open Untitled.png as untitled")

time = 0
r = 0
saved = False
while True:
	time += 1
	print("background 225 225 225")
	print("fill 255 0 0 100")
	print("stroke 0 0 0 100")
	print("strokeweight 10")
	print("get keys")
	print(input().split())
	print("get keyispressed space")
	# print("the mouse x is " + str(input()))
	# print("rect " + str(175 + 100 * math.sin(time/10)) + " " + str(input()) + " 50 50")
	if input() == "true":
		print("get mousex")
		print("get mousey")
		print("rect " + str(float(input()) - 25) + " " + str(float(input()) - 25) + " 50 50")
	print("rect 175 175 50 50")
	print("push")
	print("scale " + str(0.75 + math.sin(time/100)/2) + " " + str(0.75 + math.cos(time/10)/2))
	print("rotate " + str(r))
	print("translate 100 100")
	r += 1.0
	print("ellipse -45 -25 90 50")
	print("pop")
	if not saved:
		print("get keyispressed s")
		if input() == "true":
			print("save capture.png")
			saved = True
	print("text 300 300")
	print("hello world")
	print("arc 100 300 50 0 90")
	print("line 125 325 75 275")
	print("image untitled 300 100")
	print("strokeweight 1")
	print("poly 300 100 310 90 320 90 300 100")
	print("present")