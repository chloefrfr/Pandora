package packets

import (
	"encoding/json"
	"pandora/logger"
)

type StatusResponse struct {
	Version     VersionInfo `json:"version"`
	Description string      `json:"description"`
	Players     PlayersInfo `json:"players"`
}

type VersionInfo struct {
	Name     string `json:"name"`
	Protocol int32  `json:"protocol"`
}

type PlayersInfo struct {
	Max    int32 `json:"max"`
	Online int32 `json:"online"`
}

var logging = logger.New()

func HandleStatusRequest() string {
	response := StatusResponse{
		Version: VersionInfo{
			Name:     "1.21.3",
			Protocol: 760,
		},
		Description: "Pandora",
		Players: PlayersInfo{
			Max:    10,
			Online: 0,
		},
	}

	responseJSON, err := json.Marshal(response)
	if err != nil {
		logging.Error("Error marshalling status response: " + err.Error())
		return ""
	}
	return string(responseJSON)
}
