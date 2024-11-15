package server

import (
	"io"
	"net"
)

func (s *Server) handleClient(conn net.Conn) {
	defer func() {
		conn.Close()
		delete(s.clients, conn)
	}()

	buffer := make([]byte, 1024)
	for {
		n, err := conn.Read(buffer)
		if err != nil {
			if err == io.EOF {
				break
			}
			logging.Error("Error reading from client: " + err.Error())
			break
		}
		if n > 0 {
			s.handlePackets(buffer[:n])
		}
	}
}
