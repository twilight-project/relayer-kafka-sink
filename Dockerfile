FROM rust:1.87.0 as builder 
RUN USER=root apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install git curl g++ build-essential libssl-dev pkg-config && \
    apt-get -y install software-properties-common && \
    apt-get update

COPY ./ ./relayer-kafka-sink
WORKDIR /relayer-kafka-sink

RUN cargo build --release
FROM rust:1.87.0
RUN apt-get update && apt-get install -y ca-certificates curl libpq-dev libssl-dev

WORKDIR /app
COPY --from=builder ./relayer-kafka-sink/target/release/main ./
COPY --from=builder ./relayer-kafka-sink/target/release/main.d ./
COPY ./.env ./.env
ENTRYPOINT ["/app/main"]
