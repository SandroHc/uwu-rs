# uwu_wasm

A uwu package inspired by [Daniel Liu's uwu](https://github.com/Daniel-Liu-c0deb0t/uwu) that runs on modern browsers.

## Quick Guide

Install the package with:
```shell
npm install uwu-rs
pnpm install uwu-rs
yarn install uwu-rs
```

Then use as follows in your code:

```javascript
import init, { uwuify } from 'uwu-rs';

init().then(() => {
    console.log(uwuify('I have been uwuified!'));
});
```

## Building

```shell
wasm-pack build --target web --out-name uwu
```

## Publishing

```shell
wasm-pack login
wasm-pack publish
```
