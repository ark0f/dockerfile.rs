# v0.3.0 (2019-04-04)
Breaking changes:
* Use `Into<String>` instead of `AsRef<str>`

# v0.2.3 (2019-02-23)
Fixes:
* Remove useless traits `Eq` and `Hash` in `impl<K, V> From<(K, V)>` for `Copy` and `Add`

Breaking changes:
* Yank [v0.2.2](#v022-2019-02-23)

# v0.2.2 (2019-02-23)
Features:
* Add `impl<K, V> From<(K, V)> where K: AsRef<str>, V: AsRef<str>` for `Copy` and `Add` structures

# v0.2.1 (2019-02-18)
Features:
* Add documentation

# v0.2.0 (2019-02-18)
Fixes:
* Typo in `rustacean` word
* Invalid formatting for `Add` structure
* No return in `STOPSIGNAL!` macro

Features:
* `ENTRYPOINT` and `CMD` can be defined just once
* Add `Comment` structure to add comments
* Don't set default protocol for `Expose` structure

Breaking changes:
* Delete `Dockerfile!` macro
* Make `DockerFile::instruction` function private

# v0.1.1 (2019-02-17)
Fixes:
* Invalid category in `Cargo.toml`
* Invalid link to license

# v0.1.0 (2019-02-17)
