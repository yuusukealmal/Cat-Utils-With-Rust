# Certain Cat Game Utils

## Overview

A set of utilities for use with a certain cat game.

## Setup

1. Rename `.env.example` to `.env` and fill in the required variables.

2. Rename `cf_private_key.pem.example` to `cf_private_key.pem` and paste the private key obtained from `libnative-lib.so`.

## Features

1. [Decrypt BCU pack files without knowing the password.](src/bcuzip)

2. [Retrieve the game announcement file and images.](src/placement)

3. [Fetch the game event files.](src/event)

4. [Dump assets from an XAPK file.](src/local)

5. [Fetch assets from the server using an XAPK file.](src/server)

6. [Obtain your gacha seed using the transfer code and confirmation code.](src/seed)

## Running the Project

To run the project in release mode, use the following command:

```bash
cargo run --release
```

To run the project with automatic updates, use this command:

```bash
cargo run update --release
```

# TODOS
- [ ] Download EN server assets