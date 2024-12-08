# Minecraft Datapack Installer
I'm making a command line tool that allows you to install minecraft datapacks into the currently open minecraft world.
Think something like pip for minecraft datapacks.

See this repository for the Datapack Index that this tool will pull from: https://github.com/Nicolas-Gatien/MCD-Index

## Installation
> The steps below only work on MacOS
After downloading the latest release, unzip the archive and follow these steps to add mcdi as a function:

Step #1. Copy the path to the mcdi executable you just downloaded

Step #2. Open ~/.zshrc
```bash
nano ~/.zshrc
```

Step #3. Type the following:
```bash
mcdi() { 
    /path/to/the/mcdi/executable "$@"
}
```
Make sure you replace the path with the actual path.

Step #4. Save and exit the nano editor.

Step #5. Apply the Changes
```bash
source ~/.zshrc
```

## Usage
MCDI installs datapacks into whatever minecraft world is currently open.
If there's no world open, it will install to the one which was most recently open.

You can install a datapack with the `install` command:
```bash
mcdi install datapack-name
```