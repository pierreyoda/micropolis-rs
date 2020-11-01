FROM rust:latest

RUN apt-get update && apt-get install -y libudev-dev

RUN ["cargo", "install", "cargo-web"]

COPY . /usr/src/micropolis-app/

WORKDIR /usr/src/micropolis-app/

RUN ["cargo", "build"]

WORKDIR /usr/src/micropolis-app/micropolis_quicksilver/

CMD ["cargo web start", "--features quicksilver/stdweb]

EXPOSE 8000
