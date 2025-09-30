# WeatherAlerts Docs

## Dev Setup

* Setup WASM target

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

* Install trunk

  ```bash
  cargo install --locked trunk
  ```

* Install nodejs (for tailwind support)

In order to run, this example requires a working [nodejs](https://nodejs.org/en/download) installation that includes npx.


* Run

  ```bash
  trunk serve
  ```

## Deploy

Use a web server to serve the files from trunk release.

```bash
trunk build --release
```
