package packets

type VersionInfo struct {
	Name     string `json:"name"`
	Protocol int32  `json:"protocol"`
}

type PlayersInfo struct {
	Max    int32 `json:"max"`
	Online int32 `json:"online"`
}

type StatusResponse struct {
	Version     VersionInfo `json:"version"`
	Description string      `json:"description"`
	Players     PlayersInfo `json:"players"`
}
