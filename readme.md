# Must
**In construction**
Must is a simple music player form command line.     
It is clon of [mocp](https://github.com/jonsafari/mocp).   
Use the libraries [Symphonia](https://github.com/pdeljanov/Symphonia) to play music.   

Develop on twitch live streams [rubyc232](https://www.twitch.tv/rubyc232)

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
- [x] Agregar mecanismo de señales para emitir/capturar eventos de los comandos que le llegan al servidor.
- [ ] Hacer un singleton para la clase play.
- [ ] Hacer que el servidor se quede ejecutanodoce en segundo plano.
- [ ] Escribir logs en un fichero.
- [ ] Mejorar la clase play.
- [ ] Agregar comando de play/pause.
- [ ] Agregar comando de stop.
- [ ] Agregar comando de next.
- [ ] Agregar comando de previous.
- [ ] Agregar comando de volume.
- [ ] Agregar soporte para cambiar de canción.
- [ ] Agregar Sistema de plugins para las fuentes de audio.