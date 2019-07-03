# you can try this out as follows:
# 	cd stdg
# 	cargo build
# 	python playground/basic.py | target/debug/stdg[.exe]

# x = 0

# while True:
# 	x += 10;

# 	print("color 255 255 255")
# 	print("clear")
# 	print("color 255 20 20")
# 	print("fill circle " + str(x) + " 150 80")
# 	print("present")
# 	print("handle")

print("window 400 400")
print("title a moving car")

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