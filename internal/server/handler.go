package server

import (
	"net"
)

func (s *Server) handleClient(conn net.Conn) {
	defer func() {
		conn.Close()
		delete(s.clients, conn)
		logging.Info("Connection closed from: " + conn.RemoteAddr().String())
	}()

	buffer := make([]byte, 1024)
	for {
		n, err := conn.Read(buffer)
		if err != nil {
			logging.Error("Error reading from client: " + err.Error())
			break
		}
		if n > 0 {
			s.handlePackets(buffer[:n])
		}
	}
}
