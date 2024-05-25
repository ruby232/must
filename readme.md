# Must
Must is a simple music player form command line. 
It is clon of [mocp](https://github.com/jonsafari/mocp).
Use the libraries [Symphonia](https://github.com/pdeljanov/Symphonia) to play music.

## Getting Started
```bash
git clone https://github.com/ruby232/must.git
cd must
cargo run -- -command
# example
cargo run -- -S
# for display help 
cargo run -- -h
```

## Comands
- `s` - Start server
- `p` - Play/pause
- `q` - Quit

## Todo
- [ ] Agregar mecanismo de señales para emeitir eventos de los coman dos que le llegan al servidor