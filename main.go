package main

import (
	"fmt"
	"log"
	"os"
	"os/user"
	"regexp"
)

const LOGPATH = "/Users/%s/Library/Application Support/minecraft/logs/latest.log"

func getCurrentUserName() string {
	currentUser, err := user.Current()
	if err != nil {
		log.Fatalf(err.Error())
	}

	username := currentUser.Username
	return username
}

func getFileContents(path string) string {
	content, err := os.ReadFile(path)
	if err != nil {
		log.Fatalf(err.Error())
	}

	return string(content)
}

func getMCWorldName(logContents string) string {
	// Get the last instance of the match
	re := regexp.MustCompile(`'ServerLevel\[(.*)\]`)
	matches := re.FindAllStringSubmatch(logContents, -1)
	if len(matches) > 0 {
		return matches[len(matches)-1][1]
	}

	return ""
}

func getLatestWorldPath() string {
	logContents := getFileContents(fmt.Sprintf(LOGPATH, getCurrentUserName()))
	mcWorldName := getMCWorldName(logContents)

	path := fmt.Sprintf("/Users/%s/Library/Application Support/minecraft/saves/%s", getCurrentUserName(), mcWorldName)

	return path
}

func main() {
	latestWorldPath := getLatestWorldPath()

	fmt.Printf("Latest world path is: %s\n", latestWorldPath)
}
