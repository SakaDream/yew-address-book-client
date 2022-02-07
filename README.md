# yew-address-book-client

An Address Book frontend using yew

## Require

- [Rust Stable](https://rustup.rs)
- [NodeJS](https://nodejs.org)
- [Yarn](https://yarnpkg.com)

Or using [Docker](https://www.docker.com)

## How to run

### Manual

- Install Node dependencies for client: `yarn install`
- Install Node dependencies for server: `cd /server && yarn install`
- Build: `cd .. && yarn run build`
- Run: `yarn run start-server`
- Go to http://localhost:3000 (Windows) or http://127.0.0.1:3000 (Linux/macOS) and enjoy! 😄

### Docker

- Build: `docker build -t yew-address-book-client .`
- Run: `docker run -p 3000:3000 -d yew-address-book-client`
- Go to http://localhost:3000 (Windows) or http://127.0.0.1:3000 (Linux/macOS) and enjoy! 😄

## Run with backend API
- This project uses **actix-web-rest-api-with-jwt** for backend-side, go to https://github.com/SakaDream/actix-web-rest-api-with-jwt for more details.

## TODOs
- [x] Hello World page
- [x] 404 - Page Not Found page
- [x] Routing
- [x] Dashboard UI
- [x] Add mock data and fetch get all items from backend
- [x] App config
- [ ] Signup, Login
- [ ] Add item UI and logic
- [ ] Update item UI and logic
- [ ] Delete item UI and logic
- [ ] Account profile UI and logic