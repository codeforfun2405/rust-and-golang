package error

import "testing"

func TestSumData(t *testing.T) {
	type args struct {
		data []int32
	}
	tests := []struct {
		name    string
		args    args
		want    int32
		wantErr bool
	}{
		{
			name: "happy",
			args: args{
				data: []int32{1, 2, 3},
			},
			want:    6,
			wantErr: false,
		},
		{
			name: "unhappy",
			args: args{
				data: []int32{1, -100, 3},
			},
			want:    -1,
			wantErr: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := SumData(tt.args.data)
			if (err != nil) != tt.wantErr {
				t.Errorf("SumData() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got != tt.want {
				t.Errorf("SumData() = %v, want %v", got, tt.want)
			}
		})
	}
}
