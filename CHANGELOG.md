# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-07-22

### Bug Fixes

- Windows-safe path asserts in video integration tests - ([9116703](https://github.com/MrDwarf7/chapterize.rs/commit/9116703317936cccbf20c204f0e2269e7c2d932f))
- Windows-aware path error assert in video test - ([11b6a2b](https://github.com/MrDwarf7/chapterize.rs/commit/11b6a2b4ff687e29d90544741c9e3be21910c74f))
- binary_candidates test is Windows-aware - ([9f96ae7](https://github.com/MrDwarf7/chapterize.rs/commit/9f96ae7146a46398dc644a737ab60439ead692e0))
- use actions/checkout@v7 directly (centralized action can only run after checkout) - ([b72bda5](https://github.com/MrDwarf7/chapterize.rs/commit/b72bda5d6a81ee4d0ee62bdcd76237cc7f0f8ef9))
- update setup-rust checkout to @v7, align with latest template - ([012daa9](https://github.com/MrDwarf7/chapterize.rs/commit/012daa9d5bb5b76f1d48d7b5481691844f71fe05))

### Documentation

- README banners use assets/ header + icon - ([46da4af](https://github.com/MrDwarf7/chapterize.rs/commit/46da4af6598309035899e7017b4509f0de97fb92))

### Features

- assets/ icons + Windows build.rs (winresource) - ([8b7e4c9](https://github.com/MrDwarf7/chapterize.rs/commit/8b7e4c9302d293e419ed9a1c519c9d698b4a0afe))
- apply rust_template conventions (workflows, configs, README) - ([2b83b8c](https://github.com/MrDwarf7/chapterize.rs/commit/2b83b8c3d8d0200936c45069c632a25740bbe75e))
- implement chapterize core - ([32e6e9a](https://github.com/MrDwarf7/chapterize.rs/commit/32e6e9a77cc1b1a06a3f1f95c94cdc840cd3db86))

### Miscellaneous Tasks

- **release:** bump to 0.1.0 - ([463eae0](https://github.com/MrDwarf7/chapterize.rs/commit/463eae0549359ff0eb3cfc4455d1db939f2088c9))
- sync assets/generate-assets.sh from rust_template - ([52de2fb](https://github.com/MrDwarf7/chapterize.rs/commit/52de2fb21a7529603e842a922d4446b2d617a97d))
- publish promotes draft via gh API (drop svenstaro force path) - ([6e8ea76](https://github.com/MrDwarf7/chapterize.rs/commit/6e8ea760cb371d77e9f47342ef12bdbe9182c22e))
- install ffmpeg before cargo test - ([93d8a24](https://github.com/MrDwarf7/chapterize.rs/commit/93d8a24fed9ea04634fce545668e4993a96cd00a))
- sync workflows to latest rust_template (publish.yml, no force-push nightly) - ([0adc083](https://github.com/MrDwarf7/chapterize.rs/commit/0adc0834ea4cb06c3d163d8b695990409c7509ff))
- add .gitattributes, centralized checkout action - ([fd58f19](https://github.com/MrDwarf7/chapterize.rs/commit/fd58f19d3929b63c6e8bffe342c940d06776f9b2))
- sync workflows with latest rust_template (checkout@v7 centralized) - ([6e965f7](https://github.com/MrDwarf7/chapterize.rs/commit/6e965f7f2a77dc82504eb3ce4c80259ee8237cb4))
- add GitHub workflows and templates - ([a05d772](https://github.com/MrDwarf7/chapterize.rs/commit/a05d772b489de53992fbae88db5a4ca83ccf9307))

### Styling

- cargo fmt -- all 3 repos - ([7a23dac](https://github.com/MrDwarf7/chapterize.rs/commit/7a23daca29c18e97cf69b98a68e2922455a9abe4))

### Testing

- add tests - ([0a726ab](https://github.com/MrDwarf7/chapterize.rs/commit/0a726ab3d799f5b220cdb9486b8fe53a77ec77c6))

### Build

- add ffmpeg fetch script for release bundling - ([6fa2851](https://github.com/MrDwarf7/chapterize.rs/commit/6fa28510250cf45e855802921b3168d7edcdf5b0))


