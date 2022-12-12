FROM node:16 AS builder

WORKDIR /app

# Install packages using Yarn
ADD package.json yarn.lock /tmp/

RUN cd /tmp && yarn --ignore-engines
ADD  ./vite.config.js  ./
ADD ./index.html ./
ADD ./public ./public
ADD ./src ./src
RUN ln -s /tmp/node_modules && ln -s /tmp/package.json

RUN yarn build

RUN mv /app/dist /dist && rm /app /tmp -r

WORKDIR /
RUN yarn global add http-server

ADD ./setup_analytics.sh .

entrypoint /setup_analytics.sh; http-server -P http://localhost:8080? /dist
