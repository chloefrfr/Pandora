package logger

import (
	"log"
	"os"
	"time"
)

const (
	Reset  = "\033[0m"
	Red    = "\033[31m"
	Green  = "\033[32m"
	Yellow = "\033[33m"
	Blue   = "\033[34m"
)

type Logger struct {
	*log.Logger
}

func New() *Logger {
	return &Logger{
		Logger: log.New(os.Stdout, "", 0),
	}
}

func (l *Logger) logMessage(level string, color string, msg string) {
	timestamp := time.Now().Format(time.RFC3339)
	l.Printf("[%s] %s%s: %s%s\n", timestamp, color, level, msg, Reset)
}

func (l *Logger) Info(msg string) {
	l.logMessage("info", Green, msg)
}

func (l *Logger) Error(msg string) {
	l.logMessage("error", Red, msg)
}

func (l *Logger) Debug(msg string) {
	l.logMessage("debug", Blue, msg)
}
