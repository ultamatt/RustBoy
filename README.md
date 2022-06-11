# RustBoy

## Goal

- Make a fully functional gameboy emulator to load up roms and play them
- Needs to have a simple UI to show the game and what input is being presented
- Would be nice to have a menu to select a rom to open

### GUI library

- Looking at Druid right now. Or maybe QT bindings
- Maybe check out iced

### Emulation

- Making some lists of information for the time being

## TODO List

- [ ] CLI REPL
 - [ ] Make this tool 'cli' based
 - [ ] Make it REPL to try and run different commands
- [ ] Loader
 - [ ] Attempt to load file and show outcome/error
 - [ ] Recall previously loaded file/location
 - [ ] Upon good load, be able to do a dump of the rom with formatting
