package functions

import (
	"bytes"
	"errors"
	"io"
	"net"
)

func ReadUnsignedByte(r io.Reader) (uint8, error) {
	var b [1]byte
	n, err := r.Read(b[:])
	if err != nil {
		return 0, err
	}
	if n != 1 {
		return 0, errors.New("failed to read a single byte")
	}
	return b[0], nil
}

func ReadVarInt(r *bytes.Reader) (int32, error) {
	var result int32
	var shift uint
	for {
		k, err := ReadUnsignedByte(r)
		if err != nil {
			return 0, err
		}

		result |= int32(k&127) << (shift * 7)
		shift++
		if shift > 5 {
			return 0, errors.New("VarInt too big")
		}
		if k&128 != 128 {
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

func WriteString(conn net.Conn, str string) error {
	data := []byte(str)

	var buffer bytes.Buffer

	WriteVarInt(&buffer, int32(len(data)))

	if _, err := buffer.Write(data); err != nil {
		return err
	}

	_, err := conn.Write(buffer.Bytes())
	return err
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

func BuildPacket(id int) (*bytes.Buffer, error) {
	var buffer bytes.Buffer

	WriteVarInt(&buffer, int32(id))

	totalLength := int32(buffer.Len()) + int32(len(buffer.Bytes()))
	WriteVarInt(&buffer, totalLength)

	return &buffer, nil
}
