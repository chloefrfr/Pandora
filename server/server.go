package server

import (
	"net"
	"pandora/logger"
)

var logging = logger.New()

type Server struct {
	address string
	clients map[net.Conn]bool
}

func NewServer(address string) *Server {
	return &Server{
		address: address,
		clients: make(map[net.Conn]bool),
	}
}

func (s *Server) Start() error {
	ln, err := net.Listen("tcp", s.address)
	if err != nil {
		logging.Error("error starting server: " + err.Error())
		return err
	}
	defer ln.Close()

	logging.Info("Server started on: " + s.address)
	for {
		conn, err := ln.Accept()
		if err != nil {
			logging.Error("Error accepting connection: " + err.Error())
			continue
		}

		logging.Info("New connection from: " + conn.RemoteAddr().String())
		s.clients[conn] = true

	}
}
