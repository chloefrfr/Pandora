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

func ReadString(r *bytes.Reader) (string, error) {
	length, err := ReadVarInt(r)
	if err != nil {
		return "", err
	}
	strBytes := make([]byte, length)
	_, err = r.Read(strBytes)
	if err != nil {
		return "", err
	}
	return string(strBytes), nil
}

func ReadUnsignedShort(r *bytes.Reader) (int32, error) {
	b1, err := r.ReadByte()
	if err != nil {
		return 0, err
	}
	b2, err := r.ReadByte()
	if err != nil {
		return 0, err
	}
	return (int32(b1) << 8) | int32(b2), nil
}

func WriteVarInt(w *bytes.Buffer, value int32) {
	for {
		if (value & ^0x7F) == 0 {
			w.WriteByte(byte(value))
			return
		} else {
			w.WriteByte(byte(value&0x7F | 0x80))
			value >>= 7
		}
	}
}
