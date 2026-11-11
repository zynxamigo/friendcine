# 🎬 FriendCine

Salas privadas para compartilhar tela e assistir a filmes e séries juntos. Interface inspirada em aplicativos de comunidades, com foco somente em tela compartilhada e cinema.

## O que existe nesta versão

- **Salas:** criar uma sala e convidar participantes por link/código.
- **Compartilhamento:** captura de tela via WebRTC com opções de 480p, 720p e 1080p e 15/30/60 FPS.
- **Cinema:** o anfitrião carrega uma lista M3U; o catálogo é agrupado por categorias e episódios.
- **Controle do cinema:** somente o anfitrião controla reprodução e pausa; participantes podem enviar sugestões.
- **Interface:** layout responsivo com canais laterais, palco principal e painel de participantes.
- **PWA:** pode ser instalado pelo navegador quando hospedado em HTTPS.

## Aplicativo para Windows

Ao publicar uma versão com uma tag `v*`, o GitHub Actions compila o servidor e gera:

- `FriendCine-Setup.exe`: instalador para Windows (atalhos e desinstalador).
- Binários portáteis para Linux, Windows e macOS.

O instalador coloca o FriendCine em `%LOCALAPPDATA%\Programs\FriendCine`. Ao abrir, o aplicativo inicia o servidor local e abre o navegador em `http://127.0.0.1:8080`. Para encerrar, feche a janela do servidor.

## Executar pelo código-fonte

Requer Rust stable:

```bash
cargo run --release
```

Abra `http://127.0.0.1:8080`. A porta padrão é 8080; defina `PORT` para usar outra.

## Hospedagem

O servidor Axum precisa ficar em um host com HTTPS/WSS para salas acessíveis pela internet. O endereço localhost do aplicativo é local ao computador que o executa; para convidar pessoas pela internet, publique o servidor atrás de um proxy HTTPS (por exemplo, Caddy).

O compartilhamento de tela depende das permissões do navegador e de contexto seguro (HTTPS ou localhost). O desempenho de vídeo depende da rede, navegador e hardware.

## Docker

```bash
docker build -t friendcine .
docker run --rm -p 8080:8080 friendcine
```

Licença MIT.
