package abs

import "fmt"

type AppRunner interface {
	Run()
}

type MacOS struct{}

func (os *MacOS) Run() {
	fmt.Printf("Macos is running App")
}

type Iphone struct{}

func (ip *Iphone) Run() {
	fmt.Printf("IPhone is running App")
}

func RunApps(runners []AppRunner) {
	for _, runner := range runners {
		runner.Run()
	}
}
