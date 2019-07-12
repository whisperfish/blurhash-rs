workflow "Test and Publish" {
  on = "push"
  resolves = ["Fmt Check and Test"]
}

action "Fmt Check and Test" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
