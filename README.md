# ToDo CLI application written in Rust
Mostly made this to learn Rust a little more. Specifically database operations with it.

## TODO (get it?)
- [ ] Implement Date struct and methods
- [ ] Implement Task struct and methods
- [ ] Implement ADD task
- [ ] Implement LIST tasks
  - ID, STATUS, DESCRIPTION
- [ ] Setup Database's structural script for persistence
- [ ] Implement COMPLETE task (mark it as completed)
- [ ] Implement DELETE task
- [ ] Implement UPDATE task
- [ ] Implement clean argument parsing (prolly will start with an ugly one)
- [ ] Cleanly format all outputs
- [ ] Implement correct error messages and return codes
- Maybe more later...

---

# Downloading
I don't know why you would want this.
## From Source
Clone this repo
```bash
$ git clone https://github.com/maximeNoiret/rust-todolist
$ cd rust-todolist
```
Then build and install it
```bash
$ cargo build -r
$ cargo install --path .
```

### Uninstall
Obviously, you messed up installing it because it's useless.
```bash
$ cargo uninstall
```

---

# Contributing
just... do a pull request or smth. tho I'm doing this to learn Rust so I won't accept pull requests that ADD anything. Only those that make existing things BETTER.
