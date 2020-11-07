module.exports = {
  "env": {
    "browser": true,
    "commonjs": true,
    "es6": true,
    "node": true,
  },
  "extends": [
    "eslint:recommended",
    "eslint-config-google",
  ],
  "globals": {
    "Atomics": "readonly",
    "SharedArrayBuffer": "readonly",
  },
  "parserOptions": {
    "ecmaVersion": 11,
  },
  "rules": {
    // Off
    "require-jsdoc": [
      "off",
    ],
    "max-len": [
      "off",
    ],

    // Error
    "linebreak-style": [
      "error",
      "unix",
    ],
    "semi": [
      "error",
      "always",
    ],

    // Warn
    "camelcase": [
      "warn",
      {
        "ignoreDestructuring": true,
      },
    ],
    "one-var": [
      "warn",
      {
        "initialized": "never",
        "uninitialized": "consecutive",
      },
    ],
    "no-unused-vars": [
      "warn",
      {
        "vars": "all",
        "args": "none",
        "ignoreRestSiblings": true,
        "caughtErrors": "none",
      },
    ],
    "guard-for-in": [
      "warn",
    ],
    "indent": [
      "warn",
      2,
    ],
    "comma-dangle": [
      "warn",
      {
        "arrays": "always-multiline",
        "objects": "always-multiline",
        "imports": "always-multiline",
        "exports": "always-multiline",
        "functions": "only-multiline",
      },
    ],
    "quotes": [
      "warn",
      "double",
    ],
    "arrow-parens": [
      "warn",
      "always",
    ],
  },
};
