// you can try this out as follows:
// 	cd stdg
// 	cargo build
// 	go run playground/basic.go | target/debug/stdg[.exe]

package main

import "fmt"

func main() {
	fmt.Println("window 400 400")
	fmt.Println("title A Rectangle")

	for {
		fmt.Println("color 255 255 255")
		fmt.Println("clear")
		fmt.Println("color 255 0 0")
		fmt.Println("rect 50 50 100 100")
		fmt.Println("present")
		fmt.Println("handle")
	}
}