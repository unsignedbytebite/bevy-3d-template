# Bevy 3D template

Create a Bevy project from an opinionated [Bevy](https://bevy.org/) 3D template using [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli).

After [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli) is installed run...

```
bevy new -t unsignedbytebite/bevy-3d-template my-new-3d-game
```
```
```

This template uses a project layout that prioritise a multi-plugin pattern for solutions that I find useful.

The template also features a few dependencies that features common Bevy tools to aid development.

## Template dependencies

- [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui) = Runtime debug inspector.
- [`rand`](https://docs.rs/rand/latest/rand/) = Random number generator.
- [`bevy_skein`](https://bevyskein.dev/) = Use Blender to create GTFL scenes.
- [`avian3d`](https://github.com/Jondolf/avian) = Physics engine.
- [`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets) = Parse data files as assets so that value changes can be hot reloaded.

