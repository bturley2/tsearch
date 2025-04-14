# build the application
FROM rust:1.84 as builder
WORKDIR /usr/src/tsearch
COPY tsearch .

RUN cargo install --path .

# we only want the compiled application, not a bunch of extra stuff:
FROM debian:bullseye-slim

# no additional dependencies required right now
# RUN apt-get update && apt-get install -y <extra-runtime-dependencies> && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tsearch /usr/local/bin/tsearch

CMD ["tsearch"]
