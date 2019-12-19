FROM rust:1.39-slim

RUN apt-get -yqq update && \
	apt-get -yqq install curl;

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Node.js
RUN apt-get -yqq install gnupg;									\
	curl -sL https://deb.nodesource.com/setup_12.x | bash -;	\
	apt-get install -yqq nodejs

# yarn
RUN apt-get -yqq remove cmdtest yarn;																	\
	apt-get -y autoremove;																				\
	curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -;									\
	echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list;		\
	apt-get update;																						\
	apt-get install -yqq yarn;

# app
WORKDIR /usr/src/micropolis_web_front

COPY . .

RUN yarn install;

CMD ["yarn", "run", "dev"]

EXPOSE 3000 3000
