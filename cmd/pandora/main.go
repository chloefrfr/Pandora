package main

import (
	"pandora/internal/server"
	"pandora/logger"
)

var logging = logger.New()

func main() {
	logging.Info("Starting Pandora")

	srv := server.NewServer(":25565")
	if err := srv.Start(); err != nil {
		logging.Error("Failed to start server: " + err.Error())
	}
}
