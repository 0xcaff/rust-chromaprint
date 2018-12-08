# rust-chromaprint
A pure Rust implementation of [chromaprint].

## Progress

This library is currently incomplete.

* [x] FFT With Overlaps
* [x] Chroma
* [x] Chroma Filter
* [x] Chroma Normalizer
* [x] Fingerprint Calculator
* [ ] Fingerprint Compression

## Implementation Notes

The [C library][chromaprint-git] is really clean, and this [post][blog] by the
creator explains the high level ideas well.

The input to this library is a mono PCM audio stream (32-bit Floating Point PCM
Data). It can be represented as a `&[f32]`. A bunch of operations are done on
this input stream. Here's a list of them used by the default algorithm
(fingerprints searchable in the AcoustID database).

#### Convert Multi-Channel to Mono

#### FFT With Overlaps

Does FFT with 2/3's of the next frame overlapping the current one.

#### Chroma

For each FFT frame, turn it into a list of notes + strength. Emits a double
vector of 12 doubles. Probably one for each note.

#### Chroma Filter

I'm not sure. This stage does some transformation and emits the vector of 12
doubles.

#### Chroma Normalizer

Normalizes the intensity of each element between 0 and 1. Emits a vector of 12
doubles (between 0 and 1 each).

#### Fingerprint Calculator

Does some quick maths and computes the fingerprint.

#### Fingerprint Compression

More bit twiddling and stuff. Can't be that hard to port.

### Complexity
It seems that most of the complexity from chromaprint comes from a few things:

* Features (CLI, FFMPEG, blah, blah, blah)
* Like 5 FFT Implementations
* Re-Inventing the Wheel Because Dependencies are Hard in C++ (base64)

## Why?
I really wanted to try out rust + web assembly in a web based chromaprint
viewer. I couldn't find a way get [rust-chromaprint-native] to work compile to
the wasm32-unknown-unknown target.

[rust-chromaprint-native]: https://github.com/0xcaff/rust-chromaprint-native
[chromaprint]: https://acoustid.org/chromaprint
[chromaprint-git]: https://github.com/acoustid/chromaprint
[blog]: https://oxygene.sk/2011/01/how-does-chromaprint-work/
