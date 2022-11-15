FROM rust:slim-bullseye
RUN pwd
RUN apt-get update -y && apt-get install -y curl git libssl-dev pkg-config
RUN git clone https://github.com/fermyon/spin -b v0.6.0 /spin
WORKDIR /spin 
RUN ls -la
RUN rustup target add wasm32-wasi && \
	cargo install --locked --path /spin
	#spin --help
WORKDIR /home
COPY . ./
RUN spin build

CMD ["spin","up","--listen","0.0.0.0:3000","--follow-all"]
