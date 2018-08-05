FROM rust:1.27

ENV OPENSSL_INCLUDE_DIR "/usr/local/opt/include"
ENV OPENSSL_LIB_DIR "/usr/local/opt/lib"
ENV DEP_OPENSSL_INCLUDE "/usr/local/opt/include"

WORKDIR /usr/src/mqtt-bridge-rs

COPY . .

RUN apt update && apt install pkg-config ca-certificates wget gcc libssl-dev libc6-dev 
RUN curl -O https://www.openssl.org/source/openssl-1.1.0g.tar.gz && tar xf openssl-1.1.0g.tar.gz && cd openssl-1.1.0g && ./Configure --prefix=/usr/local/opt linux-x86_64 -fPIC && make -j$(nproc) && make install

RUN cargo build

RUN cargo install

CMD ["mqtt-bridge-rs"]

