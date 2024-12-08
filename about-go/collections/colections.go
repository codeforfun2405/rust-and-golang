package collections

type KV struct {
	Key string
	Val int32
}

func Vec() []int32 {
	var i = int32(0)
	var result []int32
	for {
		if i < 10 {
			result = append(result, i)
		} else {
			break
		}
		i++
	}

	return result
}

func HashMap(kvs []*KV) map[string]int32 {
	data := make(map[string]int32, len(kvs))
	for _, kv := range kvs {
		data[kv.Key] = kv.Val
	}
	return data
}
