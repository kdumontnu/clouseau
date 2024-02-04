# Clouseau

<img src="./documents/images/Inspector_Clouseau_Animated.png" height="250" />

Clouseau is an proof of concept using [Tauri](https://tauri.app/) with [Sveltekit](https://kit.svelte.dev/), [Layer Cake](https://layercake.graphics/) (charts), and [sysinfo](https://crates.io/crates/sysinfo) to create a simple, cross-platform resource monitor. The primary purpose is to learn about data and resource management in Tauri. 

It's very much a work in progress, but PRs are welcome. 

<img src="./documents/images/App_Running.png" />

## Developing

Run `npm install` and `npm run dev` to start the application in development mode. You should see a Tauri app pop up once everything is built.

## Building

To create a production version of your app:

```bash
npm run build
```

## Ideas and Next Steps (PRs Welcome)

- Improve error handling and clean up data types passed between front-end and back-end
- Make unit tests and integration tests actually test something relevant
- Improved styling
- Add CI test & build
- Investigate other methods to decrease performance overhead (eg. OpenGL render, tauri event stream, etc.)
