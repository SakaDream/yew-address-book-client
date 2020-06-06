# build stage
FROM sakadream/rust-wasm as build

# create new project using cargo generate
RUN USER=root cargo generate --git https://github.com/yewstack/yew-wasm-pack-template app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml .
COPY ./package.json .
COPY ./yarn.lock .

# build this project to cache dependencies
RUN yarn install \
    && yarn run build \
    && rm src/*.rs \
    && rm tests/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./static ./static
COPY ./tests ./tests
COPY ./.cargo-ok .
COPY ./bootstrap.js .
COPY ./package.json .
COPY ./webpack.config.js .
COPY ./yarn.lock .

# build web app
RUN yarn install \
    && yarn run build

# deploy stage
FROM svenstaro/miniserve

# create app directory
RUN mkdir app
WORKDIR /app

# copy webapp files
COPY --from=build /app/dist /app

# expose port
EXPOSE 80

# run the binary
ENTRYPOINT ["miniserve", "/app", "--index", "index.html"]
