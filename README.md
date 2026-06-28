# RSDK Mod Manager

A WIP rewrite of the original RSDK Mod Manager from MainMemory into the all-encompassing Rust, with the main goal of bringing it to all non-Windows platforms.

## Features
- Enable/disable any mod in your folder
- Launch game directly from manager
- Extract mod from archive or install from other folder
- Give each game its own unique nickname
- One click handling from GameBanana

## Future features
- OTA updates
- *Maybe* a way to extract Origins audio from game files

## Known issues
- One click handling:
    - Windows users need to install the urls manually for now
    - macOS handling is currently not implemented as it's different from Windows and Linux and I ain't got time for that
- Mod creation:
    - Creating mod from scratch is not implemented as of now
    - The success of whether an extracted mod is detected depends on the mod's structure, you will need to fix it manually
