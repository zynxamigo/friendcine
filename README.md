# 🎬 FriendCine

Salas leves para assistir e compartilhar tela com amigos. Servidor em Rust (Axum + WebSocket), sem banco de dados.

- **Tela:** qualquer pessoa da sala pode compartilhar a tela (480p/720p/1080p, 15/30/60 fps).
- **Cinema:** o dono da sala carrega sua lista M3U; o servidor organiza filmes e séries por categoria. Espectadores só sugerem; só o dono dá play/pause.
- **App:** botão "Baixar app" instala como PWA no PC e no celular.

## Rodar
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

> O GitHub Pages não serve, pois o app precisa de servidor WebSocket. Hospede em Render, Fly.io, Railway ou VPS usando o Dockerfile.

Licença MIT.
