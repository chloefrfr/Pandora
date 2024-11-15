package server

import (
	"bytes"
	"fmt"
	"pandora/internal/functions"
	"pandora/internal/packets"
)

type PacketID int32

const (
	PacketHandshake      PacketID = 0x00
	PacketStatusRequest  PacketID = 0x01
	PacketStatusResponse PacketID = 0x02
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
			_, err := packets.ReadHandshake(reader)
			if err != nil {
				logging.Error("Error reading Handshake packet: " + err.Error())
				return
			}
		case PacketStatusRequest:
			response := packets.HandleStatusRequest()
			s.SendStatusResponse(response)
		default:
			logging.Error("Unknown packetId: " + fmt.Sprintf("%d", packetId))
		}
	}
}
