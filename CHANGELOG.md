# 📜 Changelog

All notable changes to this project will be documented in this file.

## [0.7.2](https://github.com/vemonet/ptrie/compare/v0.7.1..v0.7.2) - 2024-12-09

### ⚙️ Miscellaneous Tasks

- Update CHANGELOG.md - ([f82e30b](https://github.com/vemonet/ptrie/commit/f82e30b6e02d8343be4f97066ac1c4423e83de9b))
- Bump version to 0.7.2 - ([7bc0d30](https://github.com/vemonet/ptrie/commit/7bc0d30d0bf743746fddf37734acbab8f6f0ee4d))

### ⛰️ Features

- Exporse get_mut - ([b858503](https://github.com/vemonet/ptrie/commit/b85850393a115874dfe9a6f92b3762b7b78cc558))

### 🧪 Testing

- Improve tests for new get_mut() function - ([3545088](https://github.com/vemonet/ptrie/commit/354508896e007cc6c0ec048003977f80d217d3a8))

## [0.7.1](https://github.com/vemonet/ptrie/compare/v0.7.0..v0.7.1) - 2024-08-22

### ⚙️ Miscellaneous Tasks

- Update changelog - ([68db9f7](https://github.com/vemonet/ptrie/commit/68db9f73eeacd33096baf811d9ce6e1a01aa0567))
- Bump version to 0.7.1 - ([c4ea8bb](https://github.com/vemonet/ptrie/commit/c4ea8bbb1258c3ecd5ea3e94ac8c6a0479cb5318))

### ⛰️ Features

- Add .remove(key) method, closes https://github.com/vemonet/ptrie/issues/1 - ([c4707ae](https://github.com/vemonet/ptrie/commit/c4707aea3002e88952a3dc53ba1bb13014b34648))

### 🚜 Refactor

- Small refactor - ([1c9e3cf](https://github.com/vemonet/ptrie/commit/1c9e3cfc131b075339a62c61e590f83bf03faa2f))

## [0.7.0](https://github.com/vemonet/ptrie/compare/v0.6.0..v0.7.0) - 2023-12-23

### ⚙️ Miscellaneous Tasks

- Update changelog - ([6c5b144](https://github.com/vemonet/ptrie/commit/6c5b144757a8e891f00c4277a153c33df6544479))
- Fix benchmark in workflow - ([4761c49](https://github.com/vemonet/ptrie/commit/4761c49fb7c5de66d1e17a07217bd4cefbfed5a0))
- Improve benchmark workflow - ([63324e2](https://github.com/vemonet/ptrie/commit/63324e2e4c72924ca40af252e79e7652f29867d2))
- Improve benchmark workflow - ([4f423f5](https://github.com/vemonet/ptrie/commit/4f423f526fc3f0bbf563fc84929add76ef959ca1))
- Fix benchmark workflow - ([2214f64](https://github.com/vemonet/ptrie/commit/2214f64a3681fb293454bd3c701ac8313e8f5a68))
- Fix benchmark workflow - ([71660f5](https://github.com/vemonet/ptrie/commit/71660f5395fea61e39cf648ef1f34a6120013f2b))
- Bump version to 0.7.0 - ([317466f](https://github.com/vemonet/ptrie/commit/317466fc706c9c4733fb574ea0a0de5dc87c3937))

### ⚡ Performance

- [**breaking**] Refactor the `Trie` and `TrieNode` structs to replace the 2 vectors used to store values and trie node, by the root node of the trie. The values are now directly stored in the TrieNode (instead of having integers to get values from an array), and the Trie only contains 1 field: the root node of the trie. This reduces amount of effective line of code from 127 to 97. - ([e7b9531](https://github.com/vemonet/ptrie/commit/e7b9531f146b85fd798b59dbac0b01de146042e5))

### 🧪 Testing

- Add benchmark with criterion, add benchmark job to test workflow - ([79a15d7](https://github.com/vemonet/ptrie/commit/79a15d7ddc9c24c9eca2336ad9d77163bfd1f919))
- Improve criterion benchmark use of black_box - ([617b25e](https://github.com/vemonet/ptrie/commit/617b25eef6007b88280c770d89d973349c36859b))
- Add massive benchmark for prefix/postfix - ([b0bf31a](https://github.com/vemonet/ptrie/commit/b0bf31a097a3099ddbebd801154c8929525894ae))

## [0.6.0](https://github.com/vemonet/ptrie/compare/v0.5.4..v0.6.0) - 2023-12-22

### ⚙️ Miscellaneous Tasks

- Update changelog - ([baef8a9](https://github.com/vemonet/ptrie/commit/baef8a948f6bc674c0da34c4102025dc759454e7))
- Update release script and readme - ([e06dd4c](https://github.com/vemonet/ptrie/commit/e06dd4cf4e964957a930e76a96cf9a0113304556))
- Bump version to 0.6.0 - ([15c6519](https://github.com/vemonet/ptrie/commit/15c6519949677850d9ef910940b8f5140faed87f))

### 🚜 Refactor

- [**breaking**] Rename `trie.get_value()` to `trie.get()` and return a reference to the value instead of the value to be closer to hashmap API - ([fd1e530](https://github.com/vemonet/ptrie/commit/fd1e5307825beb450c96acfaef5b7939ec894d3a))

### 🧪 Testing

- Add test for serde serialize feature - ([308a2ab](https://github.com/vemonet/ptrie/commit/308a2ab2e5d13dcbf50ebe009a15d4fc5989adca))

## [0.5.4](https://github.com/vemonet/ptrie/compare/v0.5.3..v0.5.4) - 2023-12-21

### ⚙️ Miscellaneous Tasks

- Fix release script - ([89b58ab](https://github.com/vemonet/ptrie/commit/89b58ab726512a2cd5fb905ae899660c241b2b3d))
- Bump version to 0.5.4 - ([eb9c7bb](https://github.com/vemonet/ptrie/commit/eb9c7bb6b8d36443cabbbcb38a0b09497f07219c))

## [0.5.3](https://github.com/vemonet/ptrie/compare/v0.5.2..v0.5.3) - 2023-12-21

### ⚙️ Miscellaneous Tasks

- Add bash script to easily perform a release - ([ba03b9b](https://github.com/vemonet/ptrie/commit/ba03b9b9681e3ff31ee3552776b74edecc7a8e52))
- Update release script, improve readme - ([3e61eb0](https://github.com/vemonet/ptrie/commit/3e61eb066e620bca5fd4104bdd9996d323498a34))
- Update codecov action version and readme - ([20e35cc](https://github.com/vemonet/ptrie/commit/20e35cc8a43ef60e06ac4205e31dce9b2abc5c33))

### ⛰️ Features

- Add simple iterator for the trie - ([a4da3f5](https://github.com/vemonet/ptrie/commit/a4da3f58a4e380bc7cbddf7d151d6c02c9a070a9))

### 🐛 Bug Fixes

- Fix print - ([59e4bb7](https://github.com/vemonet/ptrie/commit/59e4bb73cf9370d2337b6f46e05c948567170008))

## [0.5.2](https://github.com/vemonet/ptrie/compare/v0.5.1..v0.5.2) - 2023-12-21

### ⚙️ Miscellaneous Tasks

- Bump to 0.5.2 - ([b46ced6](https://github.com/vemonet/ptrie/commit/b46ced6bde2ce59d061b212ac6a9fbbad4faae75))

## [0.5.1](https://github.com/vemonet/ptrie/compare/v0.5.0..v0.5.1) - 2023-12-21

### ⚙️ Miscellaneous Tasks

- Remove typos check from release workflow and add it to pre-commit - ([6ac4326](https://github.com/vemonet/ptrie/commit/6ac4326d422377fe438dca7fb55ca1628497f4be))

## [0.5.0](https://github.com/vemonet/ptrie/compare/0.4.0..v0.5.0) - 2023-12-21

### ⛰️ Features

- Add functions to find_prefixes, find_postfixes, and find_longest_prefix. Rename files - ([ccbae67](https://github.com/vemonet/ptrie/commit/ccbae673304c0d052e8625f2040b2a2005afc408))

### 📚 Documentation

- Update readme, add contributing.md, add git cliff to generate changelog, add pre-commit to automatically fmt and update changelog - ([61da9f9](https://github.com/vemonet/ptrie/commit/61da9f9456793d786ca3bb04719b27aa34f95759))
- Improve readme examples - ([50c3acb](https://github.com/vemonet/ptrie/commit/50c3acb6f26157e4a27c345bfdb6ba9da38d0b38))

### 🧪 Testing

- Improve tests, add GitHub actions workflows for testing and releasing, remove travis CI, update benchmark script - ([8391056](https://github.com/vemonet/ptrie/commit/839105644ff00e1ac9a8fee08bf0c5f6eb2fddf8))
- Fix tests - ([4203259](https://github.com/vemonet/ptrie/commit/42032593f3f5886ed198043d3983bc5231f72641))
- Fix codecov upload - ([8666e6a](https://github.com/vemonet/ptrie/commit/8666e6a7eba82cdfbce4acf1d564564da9b10368))

## [0.4.0](https://github.com/vemonet/ptrie/compare/0.3.0..0.4.0) - 2018-07-09

### ⚡ Performance

- Extracts values to a vector from nodes - ([8032921](https://github.com/vemonet/ptrie/commit/8032921117659093525956f35b0bee8c2b508b5b))
- Improves the perfomance - ([1542fd9](https://github.com/vemonet/ptrie/commit/1542fd90728d6e4c5123af031b635d9c7e282e81))
- Fixes the case of existing value overriding in the trie - ([89c08ad](https://github.com/vemonet/ptrie/commit/89c08ad74d7994efc97f307f46c78b537e80a3c2))

### 🎨 Styling

- Fixes formatting with rust-fmt - ([57b8acf](https://github.com/vemonet/ptrie/commit/57b8acf6ddaed88c391a7548982fcef8fa7eb491))

## [0.3.0](https://github.com/vemonet/ptrie/compare/0.2.1..0.3.0) - 2017-12-19

### ⚙️ Miscellaneous Tasks

- Improves the performance by keys localization in memory

Previous version of the TrieNode structure caused cache miss on each
comparison iteration.

Placing the child key in the node itself makes these comparisons much
faster because they keys are localized in CPU cache
 - ([2cc8e88](https://github.com/vemonet/ptrie/commit/2cc8e882f32e99044b8e6a89a236de4accb9f5b0))

## [0.2.1](https://github.com/vemonet/ptrie/compare/0.2.0..0.2.1) - 2017-12-19

## [0.2.0](https://github.com/vemonet/ptrie/compare/0.1.2..0.2.0) - 2017-12-17

## [0.1.2](https://github.com/vemonet/ptrie/compare/0.1.1..0.1.2) - 2017-12-12

## [0.1.1](https://github.com/vemonet/ptrie/tree/0.1.1) - 2017-12-12

<!-- generated by git-cliff -->
