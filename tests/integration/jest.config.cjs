module.exports = {
  transform: {
    // JS
    "^.+\\.js$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [
              [require.resolve("../../swc_plugin_import_meta_env.wasm"), {}],
            ],
          },
          parser: {
            syntax: "ecmascript",
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
    // TS
    "^.+\\.ts$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [
              [require.resolve("../../swc_plugin_import_meta_env.wasm"), {}],
            ],
          },
          parser: {
            syntax: "typescript",
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
    // JSX
    "^.+\\.jsx$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [
              [require.resolve("../../swc_plugin_import_meta_env.wasm"), {}],
            ],
          },
          parser: {
            syntax: "ecmascript",
            jsx: true,
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
    // TSX
    "^.+\\.tsx$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [
              [require.resolve("../../swc_plugin_import_meta_env.wasm"), {}],
            ],
          },
          parser: {
            syntax: "typescript",
            jsx: true,
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
  },
  testEnvironment: "@happy-dom/jest-environment",
  testRegex: "(\\.|/)(test)\\.([jt]sx?)$",
  moduleFileExtensions: ["js", "jsx", "ts", "tsx"],
  moduleDirectories: ["src", "node_modules"],
};
