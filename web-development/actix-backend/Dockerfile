FROM rust:alpine AS compile

WORKDIR /app

COPY . .

RUN cargo b -r

FROM alpine AS runner

COPY --from=compile /app/target/release/actix-backend /bin/app

ENTRYPOINT [ "/bin/app" ]
