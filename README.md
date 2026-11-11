# 🎬 FriendCine

Salas leves para assistir e compartilhar tela com amigos. Servidor em Rust (Axum + WebSocket), sem banco de dados.

- **Tela:** qualquer pessoa da sala pode compartilhar a tela (480p/720p/1080p, 15/30/60 fps).
- **Cinema:** o dono da sala carrega sua lista M3U; o servidor organiza filmes e séries por categoria. Espectadores só sugerem; só o dono dá play/pause.
- **App:** botão "Baixar app" instala como PWA no PC e no celular.

## Dois modos
- **Site (GitHub Pages)** em `docs/`: HTML/CSS/JS puro. O navegador do dono da sala é o servidor (PeerJS só faz a conexão inicial). Ativar: Settings → Pages → *Deploy from a branch* → `main` / `docs`.
- **App em Rust** (`src/`): servidor próprio (Axum), baixe pronto em Releases. Bom para hospedar sua própria instância ou usar em rede local.

## Rodar o app Rust
```bash
cargo run --release   # http://localhost:8080  (PORT=3000 para trocar)
```
Compartilhar tela exige `localhost` ou HTTPS (use Caddy, Cloudflare Tunnel, etc).

## Docker
```bash
docker build -t friendcine . && docker run -p 8080:8080 friendcine
```

## Publicar versão
`git tag v0.1.0 && git push --tags` — o GitHub Actions gera binários para Linux, Windows e macOS na aba Releases.


Licença MIT.
