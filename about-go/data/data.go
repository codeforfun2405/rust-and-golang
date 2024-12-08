package data

import "fmt"

type Status int

const (
	Pending Status = iota
	Scheduled
	Running
	Stopped
)

func Data() {
	var num1 int8 = 1
	var num2 int16 = 1
	var num3 int32 = 1
	var num4 int64 = 1
	fmt.Printf("%d, %d, %d, %d\n", num1, num2, num3, num4)

	var num5 uint8 = 1
	var num6 uint16 = 1
	var num7 uint32 = 1
	var num8 uint64 = 1
	fmt.Printf("%d, %d, %d, %d\n", num5, num6, num7, num8)
}

func Strings() {
	var str = "Hello, World"
	var char = 'a'
	fmt.Printf("%s, %c\n", str, char)
}
