# base image
FROM ghcr.io/sakadream/rust-wasm-docker-image:master as build
WORKDIR /app

# copy project files
COPY ./src ./src
COPY ./server ./server
COPY ./config ./config
COPY ./static ./static
COPY ./tests ./tests
COPY ./index.html .
COPY ./.cargo-ok .
COPY ./Cargo.toml .
COPY ./package.json .
COPY ./trunk.toml .

# Build node server
WORKDIR /app/server
RUN yarn install

# build client app
WORKDIR /app
RUN yarn install \
    && yarn global add nodemon \
    && yarn run build-release \
    && rm src/*.rs \
    && rm tests/*.rs \
    && rm static/*.*

# expose port
EXPOSE 3000

# run the server
ENTRYPOINT ["yarn", "run", "start-server-local"]
