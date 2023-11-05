# swc-plugin-import-meta-env

> Simple plugin to transform `import.meta.env` to `process.env`

[![npm](https://img.shields.io/npm/v/swc-plugin-import-meta-env.svg)](https://www.npmjs.com/package/swc-plugin-import-meta-env)

This `@swc` plugin provides a simple transformation from `import.meta.env` to `process.env`.

The original purpose of this was to allow `@swc` usage in a large Jest test suite while also using Vite for bundling.

## Install 🌱

```shell
npm i -D swc-plugin-import-meta-env
```

## Usage 🚀

Simply add this to the plugins field of your `.swcrc`.

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["swc-plugin-import-meta-env", {}]]
    }
  }
}
```

Or programmatically as an extension to your existing `.swcrc` parsing:

```js
const swcrc = JSON.parse(fs.readFileSync(".swcrc", "utf8"));
((swcrc.jsc ??= {}).experimental ??= {}).plugins = [
  ["swc-plugin-import-meta-env", {}],
]; // This may need updating to suit your requirements
```

## How do I populate my environment? 🤔

The purpose of this plugin currently is to keep this transformation simple. There are many tools and utilities to load `.env` files into your environment already, such as performing this during your `setupTests` phase of testing.

- [`dotenv`](https://github.com/motdotla/dotenv)
  - This is what [Vite itself uses to populate the env](https://vitejs.dev/guide/env-and-mode.html#env-files), so it makes sense to also do the same here.

If there is enough demand I can investigate adding this as core functionality to this plugin.
