package functions

import (
	"bytes"
	"fmt"
)

func ReadVarInt(r *bytes.Reader) (int32, error) {
	var result int32
	var bytesRead int
	for {
		if bytesRead >= 5 {
			return 0, fmt.Errorf("VarInt is too long")
		}
		b, err := r.ReadByte()
		if err != nil {
			return 0, err
		}
		result |= (int32(b) & 0x7F) << (bytesRead * 7)
		bytesRead++
		if b&0x80 == 0 {
			break
		}
	}
	return result, nil
}
