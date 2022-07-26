# Getting Started
1. Clone the repository.
2. Create a `.env` file in the root directory.
3. Copy the contents from `sample.env` over to `.env` and change it to match
   credentials that should be used.

## Installation

```bash
$ pnpm install
```

## Running the app

```bash
# development
$ npm run start

# watch mode
$ npm run start:dev

# production mode
$ npm run start:prod
```

## Test

```bash
# unit tests
$ npm run test

# e2e tests
$ npm run test:e2e

# test coverage
$ npm run test:cov
```

## Troubleshoot
* I get a `ERROR [TypeOrmModule] Unable to connect to the database.` error.
  * Ensure your PostgreSQL database is up and the credentials in your `.env` is
    correct.