# blurhash-rs

> A pure Rust implementation of [Blurhash](https://github.com/woltapp/blurhash).

Blurhash is an algorithm written by [Dag Ågren](https://github.com/DagAgren) for [Wolt (woltapp/blurhash)](https://github.com/woltapp/blurhash) that encodes an image into a short (~20-30 byte) ASCII string. When you decode the string back into an image, you get a gradient of colors that represent the original image. This can be useful for scenarios where you want an image placeholder before loading, or even to censor the contents of an image [a la Mastodon](https://blog.joinmastodon.org/2019/05/improving-support-for-adult-content-on-mastodon/).

## Usage

Add `blurhash` to your `Cargo.toml`:

```toml
[dependencies]
blurhash = "0.1.0"
```

### Encoding
```rust
use blurhash::encode;
use image::GenericImageView;

fn main() {
  // Add image to your Cargo.toml
  let img = image::open("octocat.png").unwrap();
  let (width, height) = img.dimensions();
  let blurhash = encode(4, 3, width, height, &img.to_rgba().into_vec());
}
```

### Decoding
```rust
use blurhash::decode;

let pixels = decode("LBAdAqof00WCqZj[PDay0.WB}pof", 50, 50, 1.0);
```

## Licence

This project is licensed under the [MIT License](LICENSE).