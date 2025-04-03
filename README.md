# Certain Cat Game Utils

## Overview

Some Utils use at a certain cat game.

## Setup

1. Rename `.env.example` to `.env` and fill in the required variables.
   
2. Rename `cf_private_key.pem.example` to `cf_private_key.pem` and fill in the private key get from libnative-lib.so.

## Features

1. [Decrypt BCU pack files without knowing the password.](src/bcuzip)

2. [Get the game announcement file and pictures.](src/placement)

3. [Get the game event files.](src/event)

4. [Dump the assets through xapk file.](src/local)

5. [Get the assets from server assets through xapk file.](src/server)

6. [Get your gacha seed through transfer code And confirmation code.](src/seed)

## Running the Project

To run the project in release mode, use the following command:

```bash
cargo run --release
```

To run the project with automatic updates, use this command:

```bash
cargo run update --release
```

## TODOS

- [] EN server assets download