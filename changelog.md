## 0.3.0
* Added `FontChain` for default and fallback fonts.  `FontChain` is an abstraction over the old `Font` and `FontCollection` structs, which are no longer part of the public API.
* Added proper support for multi-lingual monospace with `FontChain::multilingual_mono()`.

## 0.2.1
* Fix README.

## 0.2
* Depend on Footile for `PathOp`s rather than afi.
* Added vertical text layout support.
* Added right-to-left text layout support.
* Simpler `render` API replaces old nested iterator stuff.

## 0.1
* Initial release
