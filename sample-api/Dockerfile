FROM rust:1.46 as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY src ./src

RUN cargo build --release
RUN rm src/*.rs

FROM debian:buster-slim as runner
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/sample-api ${APP}/sample-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./sample-api"]