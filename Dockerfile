# base image
FROM sakadream/rust-wasm as build

# copy project files
COPY ./src ./src
COPY ./static ./static
COPY ./tests ./tests
COPY ./.cargo-ok .
COPY ./bootstrap.js .
COPY ./Cargo.toml .
COPY ./package.json .
COPY ./webpack.config.js .
COPY ./yarn.lock .

# build this project to cache dependencies
RUN yarn install \
    && yarn run build \
    && rm src/*.rs \
    && rm tests/*.rs \
    && rm static/*.*

# expose port
EXPOSE 8080

# run the binary
ENTRYPOINT ["./miniserve", "/dist", "--index", "index.html"]
