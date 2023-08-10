FROM rust:1.71

WORKDIR /usr/src/backend
COPY . .

RUN cargo build -r

WORKDIR /usr/src
RUN cp ./backend/migrations/ ./migrations/ -r
RUN cp ./backend/config.toml .
RUN cp /usr/src/backend/target/release/backend ./backend-bin

RUN rm ./backend -r 

RUN mkdir /usr/src/files/upload -p && mkdir /usr/src/files/storage -p
VOLUME /usr/src/files/

CMD ["./backend-bin"]





