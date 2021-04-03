FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef --version 0.1.16
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef --version 0.1.16
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin libremaps

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/libremaps /usr/local/bin
EXPOSE 80
ENV DATABASE_URL=
ENV UPDATE_INTERVAL=30
ENV CORS=https://maps.librewiki.net
ENV HOST=0.0.0.0:80
ENTRYPOINT ["/usr/local/bin/libremaps"]
