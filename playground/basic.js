// you can try this out as follows:
// 	cd stdg
// 	cargo build
// 	node playground/basic.js | target/debug/stdg[.exe]

console.log("window 400 400");
console.log("title A Rectangle");

while (true) {
	console.log("color 255 255 255");
	console.log("clear");
	console.log("color 255 0 0");
	console.log("rect 50 50 100 100");
	console.log("present");
	console.log("handle");
}
