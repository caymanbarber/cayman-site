FROM alpine:3 AS builder
WORKDIR /app
COPY index.html .
COPY bingo.PNG .

FROM caddy:2-alpine
COPY --from=builder /app /srv/site
COPY Caddyfile /etc/caddy/Caddyfile
EXPOSE 8080
