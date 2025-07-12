# arxiv_taxonomy

This crate includes a build script for automatically generating a type-safe taxonomy tree that the official arXiv website uses.

> [!NOTE]
> - This script does not need to be ran automatically every time when `cargo build` is ran, and is thus separated in its own crate instead of a `build.rs` file.
> - Please be mindful and respectful when running this script locally, since it makes an HTTP request to the official arxiv website via <https://arxiv.org/category_taxonomy>.

To run the script, run these commands in separate terminals:
```sh
# this will print out the local server URL
# it's running on, including the port number:
# "ChromeDriver was started successfully on port <port>."
chromedriver

# run from
# - root project directory
# - or crates/arxiv_taxonomy (with just `cargo run <port>`)
# pass <port> as the port number that chromedriver is using
cargo run -p arxiv_taxonomy <port>
```
