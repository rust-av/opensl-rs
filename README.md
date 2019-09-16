# OpenSL ES bindings

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Actions Status](https://github.com/rust-av/opensl-rs/workflows/opensl/badge.svg)](https://github.com/rust-av/opensl-rs/actions)


It is a simple [binding][1] and safe abstraction over [opensles][2] with a focus on [Android][3].

## Building

The bindings are generated using the headers and libraries that ought to be present in the system.

You must set `ANDROID_NDK` and `NDK_PLATFORM` in order to build it.

## TODO
- [ ] Simple bindings
- [ ] cargo-apk support
- [ ] Safe abstraction
- [ ] Android-specific support
- [ ] Examples

[1]: https://github.com/servo/rust-bindgen
[2]: https://www.khronos.org/opensles/
[3]: https://developer.android.com/ndk/guides/audio/opensl-for-android.html
