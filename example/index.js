const wasm = new Uint8Array(arguments[0]);
const run = async () => {
    const module = await WebAssembly.compile(wasm);
    const instance = await WebAssembly.instantiate(module);
    console.log(instance.exports.add(1, 2));
};
run();