# Astrox

A rust driven axum served platform for linking high performance statically generated astro sites to a rust API for extremely light weight and high performance web applications on constrained systems. The base implementation of this project uses around 2 MB of memory

## Features

- [x] Rust Axum API
- [x] Astro Static Site Generation
- [x] Tailwind CSS Styling
- [x] Formatters
- [x] Rust Linting
- [ ] Web Linting
- [x] Interactive Islands (React)
- [x] Absolute Imports for Astro + TSX `@/*`

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [PNPM](https://pnpm.io/installation)

### Dev Mode

- `pnpm dev:frontend` - Start the frontend astro server on port 4321
- `pnpm dev:backend` - Start the backend axum server on port 3000

### Production Build

- `pnpm build` - Build the frontend astro site, and the backend axum server

### Run Production

- `pnpm start` - Start the production server on port 3000 serving the astro site and running the axum server for routes under `/api/`

### Linting & Formatting

- `pnpm lint` - Lint the rust code, web code linting coming soon
- `pnpm format` - Format the rust and web code

### Quick Launch

- `pnpm launch` - Build and start the production server
