# star-trek-attack-wing

Game of Star Trek Attack Wing in Rust on Dioxus (desktop, web browser)

# Data

Check this for data
https://docs.google.com/spreadsheets/d/1aeo9a95bnjVV5ahojfcfDv0jsFMM-dcY-4MF1DH2xgU/edit?gid=25819877#gid=25819877

There is also the obvious fleet builder data
https://github.com/PrudentAndorian/PrudentAndorian.github.io/tree/main/src/data

Space ship dimensions according to lore
https://www.st-minutiae.com/resources/comparison/references.html

3d printing models for bases (from which you can derive sizes)
https://cults3d.com/en/3d-model/game/fantasy-flight-miniatures-bases
https://www.stlfinder.com/model/star-trek-attack-wing-standard-ship-base-njDWOO4D/5122541/

Some photos to derive sizes
https://cf.geekdo-images.com/YdDBFRu2SXxj1ofiTuaMlA__imagepage/img/j8bGK3eZZrz8Kr4O_wGNgCIJUko=/fit-in/900x600/filters:no_upscale():strip_icc()/pic2522390.jpg

Ship images
https://static.wikia.nocookie.net/memoryalpha/images/4/4d/Star_Trek_Attack_Wing_Expansion_Waves.jpg/revision/latest?cb=20170621221049&path-prefix=en

Movement template images
https://i.ebayimg.com/images/g/npEAAOSwvRhddrXu/s-l1600.jpg

Movement template 3d images
https://www.thingiverse.com/thing:4844591

Borg Cube Base (ship plate)
https://www.stlfinder.com/model/star-trek-attack-wing-borg-cube-base-fan-version-AwwLmKBE/1632749/
https://www.thingiverse.com/thing:5025886

Card templates (including ship plates?)
https://boardgamegeek.com/filepage/122355/staw-card-design-starter-kit

# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

