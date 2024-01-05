# ![](icon.png) micronfig

Tiny crate for simple configuration management.

```rust
micronfig::config! {
	DATABASE_URI,
	APPLICATION_NAME: String,
	MAX_CONCURRENT_USERS: String > u64,
	SHOWN_ALERT?,
}
```

## Links

[![Crates.io](https://img.shields.io/crates/v/micronfig)](https://crates.io/crates/micronfig)
 
[![Documentation](https://img.shields.io/docsrs/micronfig)](https://docs.rs/micronfig/latest/micronfig/)

## Acknowledgements

Icon made with [Font Awesome](https://fontawesome.com/) ([CC-BY-4.0](https://fontawesome.com/license/free)) and [Emblematic](https://github.com/Steffo99/emblematic/)
