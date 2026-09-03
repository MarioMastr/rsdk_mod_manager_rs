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

## Known issues
- One click handling:
    - macOS handling is currently not implemented as it's different from Windows and Linux and I ain't got time for that
- Mod creation:
    - The success of whether an extracted mod is detected depends on the mod's structure, you will need to fix it manually
