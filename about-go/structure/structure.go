package structure

type Person struct {
	Name string
	Age  uint8
}

func NewPerson(name string, age uint8) *Person {
	return &Person{
		Name: name,
		Age:  age,
	}
}
