FROM rust:1.71 as builder

# Build binary
WORKDIR /src
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-dev musl-tools

# Pre build the project dependencies, doing this ahead of project build 
# allows us to skip it if no dependencies have changed
RUN USER=root cargo new backend
WORKDIR /src/backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Build the actual project
COPY . ./
RUN RUSTFLAGS='-C relocation-model=static' cargo build --release --target x86_64-unknown-linux-musl

RUN mkdir files/upload -p && mkdir files/storage -p

FROM scratch as cleanup

COPY --from=builder /src/backend/target/x86_64-unknown-linux-musl/release/backend /backend-bin 

VOLUME /usr/src/app/files/
ENV UPLOAD_PATH="files/upload" STORAGE_PATH="files/storage"
CMD ["/backend-bin"]





