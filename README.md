![Header image](https://user-images.githubusercontent.com/55672846/170165815-b3ec6814-70eb-4416-85a8-b390fddf8a74.png)

[![Clippy check](https://github.com/loxygenK/rxe/actions/workflows/clippy.yml/badge.svg?branch=main)](https://github.com/loxygenK/rxe/actions/workflows/clippy.yml)
[![Test with coverage](https://github.com/loxygenK/rxe/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/loxygenK/rxe/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/loxygenK/rxe/branch/main/graph/badge.svg?token=bCOvChidDc)](https://codecov.io/gh/loxygenK/rxe)

Easily customizable command runner made with Rust 🦀

## 📦 Usage

Install by the following command:

```bash
cargo install rxe
```

Or build from the source.

```
git clone https://github.com/loxygenK/rxe
cargo run
```

### 🏃‍♀️ Running scripts

Prepare `rxe.yaml` (or `rxe.yml`, `.rxe.yml`, `.rxe.yaml`) in the same current directory, and run:

```
rxe (your command) (and some arguments)
```

For more detailed explanation, see **Examples** or **Creating configuration**.

### 🔎  Specifying the configuration

By setting the environment variable `RXE_CONFIG`, you can use any name for the configuration.

```bash
RXE_CONFIG=rxe.config.yml rxe
# Now rxe will use rxe.config.yml as the configuration file
```

## 📝 Examples

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
      echo "Executing the test for {type} {snapshot|true=(with snapshot)}"
```

and you can run the command like this:

```sh
$ rxe test --type core --snapshot
#...
```

If the arugment was somehow wrong against the configuration, the error is shown.

```bash
$ rxe test --type backend  # There is no `backend` in the choice for `type` argument.
Could not parse the command argument: The value of the argument is invalid: The value was not appropriate: 'backend' is not available as the choice.
Please check the argument you passed to `rxe`, or configuration file.
Exiting abnormally due to the above error.
```

## 🛠 Creating configuration

**TL;DR**:

- There are four types for the argument, **text**, **number**, **flag**, **choice**.
- The value of the arguments is filled into the **placeholder** which is the text surrounded by `{}`.
  - It can be escaped using `\`. Please see the **placeholder** section for the behavior around the escaping

### 🔭 Overview

The configuration file is written in **YAML**. The configuration looks like this: 

```yaml
cmd:
  {Command name here}:
    args:
      {argument name here}:
        {argument type here}:
          {some additional argument configuration if neccesary}
      # other arguments can continue.

    run: |
      echo "commands. Can include the placeholder."
      
  # other commands can continue.
```

### 🔖 Arguments

```yaml
    args:
      {name}:
        {type}:
          {some additional configuration}
```

Arguments for the command can be defined in the `args`. The argument has two information: **name** and **type**[^1].  Please see **types** for available types.

### 📝 Run script

```yaml
    run: |
      echo "commands"
```

Script is defined in the `run`. Script can include **placeholders** for embedding the value of arguments.

#### Placeholder

```bash
run: |
  # The result assumes that the rxe is executed by following command line:
  #   rxe {command name} --arg "Some text" --flag

  # This is the placeholder
  echo "{arg}"     # => Some text
  
  # The placeholder can be omitted using "\"
  echo "\{arg}"    # => \{arg}
  
  # The placeholder is not omitted if there was more than two "\"
  echo "\\{arg}"   # => \Some text
  echo "\\\{arg}"  # => \\Some text
  
  # Some argument type like Flag type require more information, called Property.
  # Properties can be specified using "|".
  echo "{flag|true=enabled|false=disabled}"
                   # => enabled
```

Placeholder is the text that the surrounded by `{}`. It contains informations: **name** and **properties**. The placeholder is replaced by the value of argument (called **filling**).

The placeholder syntax looks like following:

```
{name of the argument|property name=config value|property name=config value|...}
```

For example, for the placeholder `{flag|true=enabled|false=disabled}`...

- The value of the argument "flag" is filled.

  The following properties will be used for filling:

  - `true`: `enabled`
  - `false`: `disabled`

### 🧩 Types

There is four types currently. 

#### Text type

```yaml
cmd:
  exec:
    args:
      name:
        text:
    run: |
      echo "Filled: >{name}<"
```

```bash
$ rxe exec --name "Some text"
Filled: >some text<
```

Any text. If no value is specified, rxe fails before executing the script specified in `run`.

#### Number type

```yaml
cmd:
  exec:
    args:
      name:
        number:
    run: |
      echo "Filled: >{name}<"
```

```bash
$ rxe exec --name "12345"
Filled: >12345<

$ rxe exec --name "abcde"
Could not parse the command argument: The value of the argument is invalid: The value was not appropriate: 'abcde' could not be parsed as the number (esp. f64)
Please check the argument you passed to `rxe`, or configuration file.
Exiting abnormally due to the above error.
```

Any number. If non-numeric value is used (like `abcde` or `0x12345`) or no value is specified, rxe fails.

#### Flag type

```yaml
cmd:
  exec:
    args:
      name:
        flag:
    run: |
      echo "Filled 1: >{name|true=specified|false=not specified}<"
      echo "Filled 2: >{name|true=specified}<"
```

```bash
$ rxe exec --name
Filled 1: >specified<
Filled 2: >specified<

$ rxe exec
Filled 1: >not specified<
Filled 2: ><

$ rxe exec --name "bruh"
Could not parse the command argument: The value of the argument is invalid: The value of the argument should not be given.
Please check the argument you passed to `rxe`, or configuration file.
Exiting abnormally due to the above error.
```

The argument becomes flag. The arguments can be omitted, and takes no value. If any value was passed to the flag type argument, rxe fails.

##### Properties

| name    | value                                                        | Optional? |
| ------- | ------------------------------------------------------------ | --------- |
| `true`  | The text that should be filled when the argument is used.    | Yes       |
| `false` | The text that should be filled when the argument is **not** used. | Yes       |

- `true` and `false` can be specified in the same time, but **cannot be omitted in the same time**.

#### Choice type

```yaml
cmd:
  exec:
    args:
      name:
        choice:
          - rust
          - ruby
          - python
    run: |
      echo "Filled: >{name}<"
```

```bash
$ rxe exec --name "rust"
Filled: rust

$ rxe exec --name "p"
Filled: python

$ rxe exec --name "ru"
Could not parse the command argument: The value of the argument is invalid: The value was not appropriate: 'ru' is too ambiguous. Type the choice longer
Please check the argument you passed to `rxe`, or configuration file.
```

---

[^1]: `Constraint` in the code.
