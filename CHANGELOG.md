# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚀 Features

- Add slice and io readers ([084d8d3](https://github.com/sbr075/serde-influxlp/commit/084d8d32a459faf3c8152d06aca56295683aab46))

### 🐛 Bug Fixes

- Reported error position should be corrrect ([0950bc4](https://github.com/sbr075/serde-influxlp/commit/0950bc48ec63ec070d57404b6f3a24b1b9fcc1a9))
- Check for ascii whitespace not only newline ([01062a1](https://github.com/sbr075/serde-influxlp/commit/01062a1dbcc11a53278773718ed9b782bad0c0a9))

### 🚜 Refactor

- Move first read flag to seq deserializer ([aa46bc0](https://github.com/sbr075/serde-influxlp/commit/aa46bc0b2b0bff3e1d262a26fd1185ed5e76f722))

### 📚 Documentation

- Add comments for reader functions ([072aaa0](https://github.com/sbr075/serde-influxlp/commit/072aaa0f41c0542b7aadc22539c06c8dca1e1715))

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

