package server

import (
	"bytes"
	"fmt"
	"pandora/internal/functions"
)

type PacketID int32

const (
	PacketHandshake PacketID = 0x00
)

func (s *Server) handlePackets(data []byte) {
	reader := bytes.NewReader(data)
	for reader.Len() > 0 {
		packetId, err := functions.ReadVarInt(reader)
		if err != nil {
			logging.Error("Error reading packetId: " + err.Error())
			return
		}

		switch PacketID(packetId) {
		case PacketHandshake:
			logging.Info("Handshake")
		default:
			logging.Error("Unknown packetId: " + fmt.Sprintf("%d", packetId))
		}
	}
}
