FROM node

WORKDIR /app

# Install packages using Yarn
ADD package.json yarn.lock /tmp/
RUN cd /tmp && yarn
RUN mkdir -p /app && cd /app && ln -s /tmp/node_modules && ln -s /tmp/package.json

CMD yarn run serve --host 0.0.0.0
