package main

import (
	"archive/zip"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/user"
	"path/filepath"
	"regexp"
	"strings"
)

const LOG_PATH = "/Users/%s/Library/Application Support/minecraft/logs/latest.log"
const WEB_URL = "http://localhost:8000"

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
	re := regexp.MustCompile(`'ServerLevel\[(.*)\]`)
	matches := re.FindAllStringSubmatch(logContents, -1)
	if len(matches) > 0 {
		return matches[len(matches)-1][1]
	}

	return ""
}

func getLatestWorldPath() string {
	logContents := getFileContents(fmt.Sprintf(LOG_PATH, getCurrentUserName()))
	mcWorldName := getMCWorldName(logContents)

	path := fmt.Sprintf("/Users/%s/Library/Application Support/minecraft/saves/%s", getCurrentUserName(), mcWorldName)

	return path
}

func main() {
	// Define subcommands
	installCmd := flag.NewFlagSet("install", flag.ExitOnError)

	// Parse the command
	if len(os.Args) < 2 {
		fmt.Println("expected 'install' subcommand")
		os.Exit(1)
	}

	switch strings.ToLower(os.Args[1]) {
	case "install":
		installCmd.Parse(os.Args[2:])
		if len(installCmd.Args()) < 1 {
			fmt.Println("expected 'datapack' argument")
			os.Exit(1)
		}
		if installCmd.Arg(0) == "datapack" {
			installDatapack()
		} else {
			fmt.Printf("unknown install target: %s\n", installCmd.Arg(0))
			os.Exit(1)
		}
	default:
		fmt.Printf("unknown command: %s\n", os.Args[1])
		os.Exit(1)
	}
}

func installDatapack() {
	latestWorldPath := getLatestWorldPath()

	resp, err := http.Get(WEB_URL)
	if err != nil {
		log.Fatalf("Failed to fetch datapack: %v", err)
	}
	defer resp.Body.Close()

	datapack, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Failed to read response body: %v", err)
	}

	datapackDirectoryPath := fmt.Sprintf("%s/datapacks", latestWorldPath)
	datapackPath := fmt.Sprintf("%s/datapack.zip", datapackDirectoryPath)
	err = os.WriteFile(datapackPath, datapack, 0644)
	if err != nil {
		log.Fatalf("Failed to write datapack: %v", err)
	}

	zipReader, err := zip.OpenReader(datapackPath)
	if err != nil {
		log.Fatalf("Failed to open zip file: %v", err)
	}
	defer zipReader.Close()

	extractPath := filepath.Join(datapackDirectoryPath, "datapack")
	if err := os.MkdirAll(extractPath, os.ModePerm); err != nil {
		log.Fatalf("Failed to create datapacks directory: %v", err)
	}

	for _, file := range zipReader.File {
		filePath := filepath.Join(extractPath, file.Name)

		if file.FileInfo().IsDir() {
			os.MkdirAll(filePath, os.ModePerm)
			continue
		}

		if err := os.MkdirAll(filepath.Dir(filePath), os.ModePerm); err != nil {
			log.Fatalf("Failed to create directory: %v", err)
		}

		dstFile, err := os.OpenFile(filePath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
		if err != nil {
			dstFile.Close()
			log.Fatalf("Failed to create destination file: %v", err)
		}

		srcFile, err := file.Open()
		if err != nil {
			dstFile.Close()
			log.Fatalf("Failed to open source file: %v", err)
		}

		if _, err := io.Copy(dstFile, srcFile); err != nil {
			dstFile.Close()
			srcFile.Close()
			log.Fatalf("Failed to copy file contents: %v", err)
		}

		dstFile.Close()
		srcFile.Close()
	}

	if err := os.Remove(datapackPath); err != nil {
		log.Printf("Warning: Failed to remove zip file: %v", err)
	}

	fmt.Printf("Latest world path is: %s\n", latestWorldPath)
}
