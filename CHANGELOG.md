# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚀 Features

- *(value)* Add converting string and number to bool ([b77d4b8](https://github.com/sbr075/serde-influxlp/commit/b77d4b80029d75c69032222708feba72120892c5))

### 🚜 Refactor

- Adhere to (some of) clippy guidelines ([c53ffc0](https://github.com/sbr075/serde-influxlp/commit/c53ffc06aa566e30a8df88c357bc78512469fc8a))

### 🧹 Routine Tasks

- Bump version number up to 0.1.5 ([091f0b7](https://github.com/sbr075/serde-influxlp/commit/091f0b70bd7a57b0239694b863bb44bfec1ae5b5))

## [0.1.4] - 2025-01-13

### 🚀 Features

- Make number enum public ([7b73c6c](https://github.com/sbr075/serde-influxlp/commit/7b73c6c1f38d090ab6fa86775996b26c2006e6e6))

### 🧹 Routine Tasks

- Update version to 0.1.4 ([5c57224](https://github.com/sbr075/serde-influxlp/commit/5c572244460d79164886bc5c061dd963cb311d8a))

## [0.1.3] - 2025-01-11

### 🚀 Features

- Use ryu and itoa for conversion in number ([ab50379](https://github.com/sbr075/serde-influxlp/commit/ab50379fc46566fbc157fdddc45217298a5718f1))

### 🐛 Bug Fixes

- Spelling mistake in doc comment ([a1aada0](https://github.com/sbr075/serde-influxlp/commit/a1aada00f360f88628304ea5e15e8585c4b80e98))
- Switch escape as it was the wrong way ([6fd9ae1](https://github.com/sbr075/serde-influxlp/commit/6fd9ae12ede558edeec4151c357c9fd0cc30710b))

### 🧹 Routine Tasks

- Update version number ([8a8b148](https://github.com/sbr075/serde-influxlp/commit/8a8b1488ecf7385de6c77029ed6f6530421ebe31))

## [0.1.2] - 2024-11-16

### 🚀 Features

- Add slice and io readers ([084d8d3](https://github.com/sbr075/serde-influxlp/commit/084d8d32a459faf3c8152d06aca56295683aab46))
- Add vec and io writers ([d5681dd](https://github.com/sbr075/serde-influxlp/commit/d5681dddde009881e29652bde668ff33d2a6057b))

### 🐛 Bug Fixes

- Reported error position should be corrrect ([0950bc4](https://github.com/sbr075/serde-influxlp/commit/0950bc48ec63ec070d57404b6f3a24b1b9fcc1a9))
- Check for ascii whitespace not only newline ([01062a1](https://github.com/sbr075/serde-influxlp/commit/01062a1dbcc11a53278773718ed9b782bad0c0a9))

### 🚜 Refactor

- Move first read flag to seq deserializer ([aa46bc0](https://github.com/sbr075/serde-influxlp/commit/aa46bc0b2b0bff3e1d262a26fd1185ed5e76f722))

### 📚 Documentation

- Add comments for reader functions ([072aaa0](https://github.com/sbr075/serde-influxlp/commit/072aaa0f41c0542b7aadc22539c06c8dca1e1715))
- Update ser doc comments ([e96b537](https://github.com/sbr075/serde-influxlp/commit/e96b537681440a3fab2c85bd355ecda5a4bdb0d4))

### 🧹 Routine Tasks

- Fix license names ([b6cdf3d](https://github.com/sbr075/serde-influxlp/commit/b6cdf3d17b6b8c56809671149904c00f98672368))
- Bump version number from 0.1.1 to 0.1.2 ([1085371](https://github.com/sbr075/serde-influxlp/commit/1085371e5c3deb3ad89f1261104a1ede2d75a75e))

## [0.1.1] - 2024-11-14

### 🚀 Features

- Add support for none values ([7c4f37f](https://github.com/sbr075/serde-influxlp/commit/7c4f37ff33e2059d26f4545dd94ede192c6abdec))
- Add partialeq, hash, and eq to value ([f8fe6de](https://github.com/sbr075/serde-influxlp/commit/f8fe6de3dc15e184f23e550ec6f38373764e9253))

### 🐛 Bug Fixes

- Deserialize single line to vec returns empty vec ([720c898](https://github.com/sbr075/serde-influxlp/commit/720c898f3567d221d3a44a848c4b9c62e7fb8916))
- Skip writing none values ([649b8f4](https://github.com/sbr075/serde-influxlp/commit/649b8f4a7e2fc506bb5948dba62d0006440beab3))

### 🧹 Routine Tasks

- Bump version from 0.1.0 to 0.1.1 ([d2823a2](https://github.com/sbr075/serde-influxlp/commit/d2823a25275296bcc4d6ede7afdc7a392545f2fb))

## [0.1.0] - 2024-11-13

### 🚀 Features

- Add first version of serde lp ([7d9137c](https://github.com/sbr075/serde-influxlp/commit/7d9137c314217e4eb92c1f8ab4ef66634a847d0c))

