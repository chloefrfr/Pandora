package server

import (
	"bytes"
	"encoding/json"
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
	if s.Conn == nil {
		logging.Error("Server connection is nil")
		return
	}

	reader := bytes.NewReader(data)
	for reader.Len() > 0 {
		packetId, err := functions.ReadVarInt(reader)
		if err != nil {
			logging.Error("Error reading packetId: " + err.Error())
			return
		}

		switch PacketID(packetId) {
		case PacketHandshake:
			handshake, err := packets.ReadHandshake(reader)
			if err != nil {
				logging.Error("Error reading Handshake packet: " + err.Error())
				return
			}

			logging.Info("Received handshake packet: " + fmt.Sprintf("%v", handshake))

			switch s.state {
			case 0:
				s.state = int32(handshake.NextState)

				if s.state == 1 {
					status := packets.StatusResponse{
						Version: packets.VersionInfo{
							Name:     "1.21.3",
							Protocol: handshake.ProtocolVersion,
						},
						Description: "Pandora",
						Players: packets.PlayersInfo{
							Max:    10,
							Online: int32(len(s.clients))},
					}
					response, err := json.Marshal(status)
					if err != nil {
						logging.Error("Error marshalling status response: " + err.Error())
						return
					}
					if s.Conn != nil {
						functions.WriteString(s.Conn, string(response))
						packet, err := functions.BuildPacket(0x00)
						if err != nil {
							fmt.Println("Error building packet:", err)
							return
						}
						s.Conn.Write(packet.Bytes())
					} else {
						logging.Error("Server connection is nil during handshake response")
						return
					}
				}
				if s.state == 2 {
					functions.ReadVarInt(reader)
					functions.ReadVarInt(reader)
				}
			case 1:
				status := packets.StatusResponse{
					Version: packets.VersionInfo{
						Name:     "1.21.3",
						Protocol: handshake.ProtocolVersion,
					},
					Description: "Pandora",
					Players: packets.PlayersInfo{
						Max:    10,
						Online: int32(len(s.clients))},
				}
				response, err := json.Marshal(status)
				if err != nil {
					logging.Error("Error marshalling status response: " + err.Error())
					return
				}
				if s.Conn != nil {
					functions.WriteString(s.Conn, string(response))
					packet, err := functions.BuildPacket(0x00)
					if err != nil {
						fmt.Println("Error building packet:", err)
						return
					}
					s.Conn.Write(packet.Bytes())
				} else {
					logging.Error("Server connection is nil during handshake response")
					return
				}
			}

		case PacketStatusRequest:
			logging.Debug("State: " + fmt.Sprintf("%d", s.state))
			switch s.state {
			case 1:
				// TODO
				logging.Debug("Pong")
			case 2:
				// TODO
			case 3:
				// TODO
			}

		default:
			logging.Error("Unknown packetId: " + fmt.Sprintf("%d", packetId))
		}
	}
}
