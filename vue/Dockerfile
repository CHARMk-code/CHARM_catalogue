FROM node:16 AS builder

WORKDIR /app

# Install packages using Yarn
ADD package.json yarn.lock /tmp/

RUN cd /tmp && yarn --ignore-engines
ADD  ./vite.config.* index.html  ./
ADD ./public ./public
ADD ./src ./src
RUN ln -s /tmp/node_modules && ln -s /tmp/package.json

RUN yarn build

RUN mv /app/dist /dist && rm /app /tmp -r

FROM joseluisq/static-web-server:2
WORKDIR /
COPY --from=builder /dist /dist
COPY --from=builder /bin/sh /bin/sh


CMD ["--port", "8080", "--host", "0.0.0.0", "--root", "/dist"]
