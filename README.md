# WASM-PNG

Combines a Javascript and WASM file into a single executable polygot PNG+HTML file.

Usage:
* `cargo install wasmpng`
* `wasmpng --wasm-file my.wasm --js-file my.js`
* This will generate `index.png.html` which will automatically unpack and execute, loading itself as a PNG and extracting the data from there

NOTE: By default file access is blocked in Chrome, so you can't test this locally unless you spin up an HTTP server. Start Chrome with the `--allow-file-access-from-file` to remove this restriction.

The Javascript will receive the WASM as a regular array in `arguments[0]`. See the `examples` folder for an example.

You can also pass `--max-width` to restrict the width of the generated PNG.

Inspiration and parts of the loader code come from [pnginator](https://gist.github.com/gasman/2560551) by Gasman et al.