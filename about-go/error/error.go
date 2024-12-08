package error

import "fmt"

var (
	ErrInvalidNum = fmt.Errorf("invalid num")
)

func SumData(data []int32) (int32, error) {
	var result int32
	for _, num := range data {
		if num < 0 {
			return -1, ErrInvalidNum
		}

		result += num
	}

	return result, nil
}
