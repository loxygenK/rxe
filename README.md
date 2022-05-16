# 🧑‍💻 `rxe`
[![Clippy check](https://github.com/loxygenK/rxe/actions/workflows/clippy.yml/badge.svg?branch=main)](https://github.com/loxygenK/rxe/actions/workflows/clippy.yml)
[![Test with coverage](https://github.com/loxygenK/rxe/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/loxygenK/rxe/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/loxygenK/rxe/branch/main/graph/badge.svg?token=bCOvChidDc)](https://codecov.io/gh/loxygenK/rxe)

Easily customizable command runner made with Rust 🦀

**WIP: Please check [`progress`](https://github.com/loxygenK/rxe#-progress) section for the progress of this project!**


### 📝 Examples
Create the following configuration:
```yaml
cmd:
  test:
    args:
      type:
        choice: [core, frontend, types]
      snapshot:
        flag:
    run:
      pnpm --filter=${type} jest ${snapshot?:-u}
```
and you can run the command like this:

```sh
$ rxe test --type core --snapshot
>> [rxe] pnpm --filter=type jest -u

#...
```

### 🏃 Progress
- [X] Parsing configuration
- [X] Parsing the command argument
- [ ] Running the script
  - [ ] Fill the placeholder
    - [ ] Parse place holder
    - [ ] Replace the place holder
  - [ ] Run the script
---
- [ ] Support short hand style (like `-t` for `--type`)
