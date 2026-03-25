import antfu from "@antfu/eslint-config";

export default antfu({
  type: "app",
  svelte: true,
  typescript: true,
  formatters: true,
  stylistic: {
    indent: 2,
    semi: true,
    quotes: "double",
  },
  ignores: [
    ".pnpm-store/**",
    "**/src-tauri/gen/*",
    "**/src-tauri/target/*",
  ],
}, {
  files: ["**/*.svelte"],
  rules: {
    "svelte/max-attributes-per-line": [
      "error",
      {
        singleline: 2,
        multiline: 1,
      },
    ],
  },
}, {
  files: ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.svelte"],
  rules: {
    "ts/no-redeclare": "off",
    "ts/consistent-type-definitions": ["error", "type"],
    "no-console": ["warn"],
    "antfu/no-top-level-await": ["off"],
    "node/prefer-global/process": ["off"],
    "node/no-process-env": ["error"],
    "perfectionist/sort-imports": [
      "error",
      {
        type: "natural",
        order: "asc",
        groups: [
          "type",
          "builtin",
          "external",
          "internal",
          ["parent", "sibling", "index"],
          "unknown",
        ],
        internalPattern: ["^\\$lib/.*"],
      },
    ],
  },
}, {
  rules: {
    "unicorn/filename-case": [
      "error",
      {
        case: "kebabCase",
        ignore: [
          "\\.md$",
          "\\.yml$",
          "\\.json$",
          "\\.rs$",
          "\\.toml$",
          "\\.ipynb$",
        ],
      },
    ],
  },
}, {
  files: ["src/lib/components/updater/update-dialog.svelte"],
  rules: {
    "svelte/no-at-html-tags": "off",
  },
});
