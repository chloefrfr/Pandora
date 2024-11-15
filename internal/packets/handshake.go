package packets

import (
	"bytes"
	"pandora/internal/functions"
)

type Handshake struct {
	ProtocolVersion int32
	ServerAddress   string
	ServerPort      int32
	NextState       int32
}

func ReadHandshake(r *bytes.Reader) (*Handshake, error) {
	protocolVersion, err := functions.ReadVarInt(r)
	if err != nil {
		return nil, err
	}

	serverAddress, err := functions.ReadString(r)
	if err != nil {
		return nil, err
	}

	serverPort, err := functions.ReadUnsignedShort(r)
	if err != nil {
		return nil, err
	}

	nextState, err := functions.ReadVarInt(r)
	if err != nil {
		return nil, err
	}

	return &Handshake{
		ProtocolVersion: protocolVersion,
		ServerAddress:   serverAddress,
		ServerPort:      serverPort,
		NextState:       nextState,
	}, nil
}
