package main

import (
	"fmt"

	"youtube/about-go/abs"
	"youtube/about-go/collections"
	"youtube/about-go/data"
	"youtube/about-go/function"
	"youtube/about-go/structure"
)

func main() {
	fmt.Printf("Hello, World\n")

	data.Data()
	data.Strings()

	function.PrintData("Awesome Go")

	var person = structure.NewPerson("Alice", 20)
	fmt.Printf("person: %v\n", person)

	fmt.Printf("pending: %v\n", data.Pending)

	abs.RunApps([]abs.AppRunner{
		&abs.MacOS{},
		&abs.Iphone{},
	})

	fmt.Printf("collections Vec: %v\n", collections.Vec())
	fmt.Printf("collections HashMap: %v\n", collections.HashMap([]*collections.KV{
		{Key: "Hello", Val: 5},
		{Key: "Nice", Val: 4},
		{Key: "You", Val: 3},
	}))
}
