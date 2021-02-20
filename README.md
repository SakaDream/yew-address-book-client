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

## Run with backend API
- This project uses **actix-web-rest-api-with-jwt** for backend-side, go to https://github.com/SakaDream/actix-web-rest-api-with-jwt for more details.

## TODOs
- [x] Hello World page
- [x] 404 - Page Not Found page
- [x] Routing
- [x] Dashboard UI
- [x] Add mock data and fetch get all items from backend
- [ ] App config
- [ ] Signup, Login
- [ ] Add item UI and logic
- [ ] Update item UI and logic
- [ ] Delete item UI and logic
- [ ] Account profile UI and logic