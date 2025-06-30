# DragonSkale
Game Off 2023 submission.

## Setup

Follow Bevy's setup instructions [here](https://bevyengine.org/learn/quick-start/getting-started/setup/#rust-setup).

## Running the game

```sh
# Running the Development Build
cargo run
```

```sh
# Running the Release Build
cargo run --release
```

## Web Builds

### Setup

Install Bevy CLI:
```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --branch main --rev 0a4df19a729dbbb83774c4ddacf21770bafceb21 --locked bevy_cli
```

### Running

```sh
# Run Development Web Build in Browser
bevy run web --open
```

```sh
# Run Release Web Build in Browser
bevy run --release web --open
```

### Building

```sh
# Build for Release (artifacts will be in `target/bevy_web`)
bevy build --release web --bundle
```

### Releasing to Itch.io

Create a Release on GitHub. This will trigger the `publish.yml` workflow which will build the game with the `release` configuration and push it to Itch.io using Butler.

### Manual Release (Not Recommended)

1. Use the following command to generate a web production build that works on Itch.io:

   ```sh
   bevy build --release web --bundle
   ```

2. Go to the generated `target/bevy_web/web-release/dragonskale` directory and create a .zip file with all files. Name it `dragonskale.zip`. You should see the following files/directories inside:

   ```
   assets/
   build/
   index.html
   ```

3. Go to the "Edit Game" page on Itch.io and click "Upload File". Make sure you are replacing the file `dragonskale.zip` which is the `web` build.

4. Check the "This file will be played in the browser" checkbox (or make sure it's checked).

5. Hit "Save" and try the game.
