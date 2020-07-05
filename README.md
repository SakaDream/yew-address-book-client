# yew-address-book-client

An Address Book frontend using yew

## Require

- [Rust Stable](https://rustup.rs)
- [NodeJS](https://nodejs.org)
- [Yarn](https://yarnpkg.com)

Or using [Docker](https://www.docker.com)

## How to run

### Manual

- Install Node dependencies: `yarn install`
- Build: `yarn run build`
- Run: `yarn run start:dev`
- Go to http://localhost:8000 (Windows) or http://127.0.0.1:8000 (Linux/macOS) and enjoy! 😄

### Docker

- Build: `docker build -t address-book .`
- Run: `docker run -p 8000 -d address-book`
- Go to http://localhost:8000 (Windows) or http://127.0.0.1:8000 (Linux/macOS) and enjoy! 😄
