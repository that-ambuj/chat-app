# Actix + Angular ChatApp

This is a simple Chat App built using [Rust](https://www.rust-lang.org/), [Actix-Web](https://actix.rs), [Angular](https://angular.io) and Websockets.

Live Demo: https://that-ambuj-chat-app.shuttleapp.rs/

## Dependencies

This project requires certain dependencies to be install in order to run this app.

- **Rust** - Install Rust's toolchain using rustup.rs
- **Shuttle CLI** - The backend uses [Shuttle](shuttle.rs) for deployment and hence require `cargo-shuttle` installed and logged in to run this app. Install `cargo-shuttle` using `cargo install cargo-shuttle` or `cargo-binstall cargo-shuttle` if you have `cargo-binstall` installed.
- **Node.js and yarn** - For managing and running the frontend and nx cli we are using node.js and yarn in this project. Install node.js from nodejs.org/en or your favourite package manger and enable yarn using `corepack enable` or refer to the [docs](https://yarnpkg.com)
- **Cargo Watch(Optional)** - For running the backend in dev mode. Can be installed using `cargo install cargo-watch`.

## Start the app

This repo is an [nx](https://nx.dev) based monorepo, so it contains all the frontend and backend code.

To run just the backend:

```bash
npx nx run backend
```

Open http://localhost:8000 in the browser to see the backend

To run both the frontend and backend in development(hot reload) mode, run the below commands in two seperate terminals:

```bash
npx nx dev backend
```

```bash
npx nx serve frontend
```

OR if you're using zsh then run both of them in parallel using:

```zsh
npx nx dev backend & npx nx serve frontend
```

Open http://localhost:4200/ to see the frontend in dev mode.
