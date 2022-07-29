# Brix

Brix is a CLI tool written in Rust for scaffolding and code generation.

## Attributions

Special thanks to [Caleb Cushing](https://github.com/xenoterracide) for the original Java version, early interface design and internal architecture.

## Installation

Brix is available on [crates.io](https://crates.io/crates/brix) and the [AUR](https://aur.archlinux.org/packages/brix-git) for Arch Linux.

Install with cargo:

```
cargo install brix
```

Arch Linux (use an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) like `yay` or `trizen`)

```
yay -S brix-git
```

## Running

Usage:

```
brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
```

#### Building locally

##### Requirements

- Cargo and a minimum Rust version of **1.43.1**

##### Running

- Run `cargo build`
- Run `cargo run`

##### Testing

Run `cargo test --all` to test the entire workspace.

##### Generating Docs

Run `cargo doc --no-deps --workspace --document-private-items --open`

## Using Brix

#### **Language directory**

To start using Brix in your project, create a `.config/brix` directory in your project.
This directory will contain your configuration files. Specifying the first argument after running `brix` will tell it which subdirectory to use. For example, starting Brix with `brix java` will search the `.config/brix/java` directory for configuration files in your project. This parameter is known as the **language**. If a config file isn't found, brix will move up to the parent directory all the way up to HOME, until it finds a directory that contains a `.config/brix/java`. Of course, this parameter doesn't have to be resitrcted to a programming language, it's there to group configuration files used in a similar way, perhaps for bootstraping a specific project.

#### **Config name**

Of course, running `brix [language]` without pointing Brix to a specific config file is a bit useless. The second argument specifies the file name to use. Running `brix java tutorial` will search for a file in `.config/brix/java` named `tutorial.brix.yml` or `tutorial.brix.yaml`. As of right now the files must be written in YAML, but more options will be supported in the future, like JSON and TOML.

#### **Project and module**

These two arguments are specific to your config file, you can use them however you want, so let's take a look at how the config file is structured first.

### Config file

Brix offers various commands for you to use to scaffold and generate your project. At the top-most level, the config file is just a list of commands, therefore we can start by declaring the commands list.

```yml
# example.brix.yml
commands:
  - search_replace:
    # ...
  - exec:
    # ...
```

The commands you list will be executed from top to bottom, one after the other. The following commands are supported:

- `copy`
  > Copies a file or directory to a new location.
- `search_replace`
  > Searches for a strin or regular expression in a file and replaces it with another string.
- `exec`
  > Executes a list of commands.
- `mkdir`
  > Creates a directory.
- `template`
  > Templates a file to a new location.

Let's start with the most basic `copy` command, and use Brix to simply copy a .gitignore file. Our config file would look something like this:

```yml
# .config/brix/js/gitignore.brix.yml
commands:
  - copy:
      source: .gitignore
      destination: app/.gitignore
```

The `source` directory is always relative to where the config file is located. In this case it would be `.config/brix/js/.gitignore`. The `destination` directory is relative to where you run `brix` from, this is also known as the working directory, and can be overriden with the `--workdir` or `-w` flag.
Now, if we were to run `brix js gitignore` in our project directory, we would actually get an error.
That's because brix requires the `project` and `module` arguments to be specified.
In our case, we're not using them yet so we can specify anything. Now, running `brix js gitignore project module` will run the command. If you didn't already have an `app` directory, brix will create one for you and copy the `.gitignore` file to it.

You might notice though that this isn't that convenient for our project if we wan't to put the file somewhere other than `app`. Let's say now we have a `backend` folder in addition to `app`. To copy the .gitignore file to backend, we would have to change the `destination` in the config file each time. Instead, this is where the project and module arguments come in handy. Let's change the config file to:

```yml
# .config/brix/js/gitignore.brix.yml
commands:
  - copy:
      source: .gitignore
      destination: {{project}}/.gitignore
```

Here, we are using the `project` argument in the destination.
Running `brix js gitignore backend module` will now copy the file to `backend/.gitignore`. If we wanted to copy it to somewhere else, all we would have to do is run `brix js gitignore somewhere_else module`.

We could also use the module parameter to, for example, sample a different gitignore file.

```yml
# .config/brix/js/gitignore.brix.yml
commands:
  - copy:
      source: {{module}}/.gitignore
      destination: {{project}}/.gitignore
```

Running `brix js gitignore dashboard default` will use `.config/brix/js/default/.gitignore` and copy it to `dashboard/.gitignore`.

Now, let's take a full look at all of the commands.

### Copy

```yml
commands:
  - copy:
      source: file.txt
      destination: output/file.txt
      overwrite: true # Optional, will ask by default to overwrite if the file already exists
```

### Search replace

Search replace uses [fancy regex](https://github.com/fancy-regex/fancy-regex) for regular expressions in the `search` field and supports backreferences. The syntax is best explained [here](https://docs.rs/fancy-regex/0.10.0/fancy_regex/#syntax).

```yml
commands:
  - search_replace:
      destination: file-to-search-replace.txt
      search: search string # fancy-regex supported
      replace: replace string
```

### Exec

Executes commands in order.

```yml
commands:
  - exec:
      commands:
        - 'echo "Hello World!"'
        - "prettier --write ."
        - "cargo --version"
      stdout: true # Optional
```

### Mkdir

Creates a directory.

```yml
commands:
  - mkdir:
      destination: output/directory
```

### Template

Templates a file.

```yml
commands:
  - template:
      source: file.ex.hbs
      destination: output/file.ex
      overwrite: true # Optional
      context: # Optional
        first: {{project}}Service
        second: {{module}}
```

`file.ex.hbs` could look something like:

```ex
defmodule App.{{first}.{{second}} do
  # ...
end
```

And when templated, `output/file.ex`:

```ex
defmodule App.UsersService.Store do
  # ...
end
```

### Context and Templating

Brix uses [Handlebars](https://handlebarsjs.com), specifically the [Rust version](https://github.com/sunng87/handlebars-rust) with both the `template` command and config files in general. The `context` parameter in the command isn't required, since `{{project}}` and `{{module}}` are automatically handled if specified in the template file.

There's also a way to specify global context in the config file:

```yml
context:
  edition: 2022
  name: {{project}}
commands:
  - template:
      source: file.txt.hbs
      destination: output/file-{{edition}}.txt
  - copy:
      source: {{edition}}/notes.txt
      destination: output/notes.txt

```

In this case, parameters `edition`, `name`, `project`, and `module` will all be available for use within the config file itself and all template files referenced in `template` commands. Global context is also useful for declaring constants that are shared between multiple commands.

#### **Templating helpers**

Brix also provides useful helpers for manipulating these variables, specifically for altering capitalization and case. The following helpers are provided:

- `to-upper`
- `to-lower`
- `to-title`
- `to-case`

All of these can be used to replace, for example, usage of `{{project}}`:

```yml
# project: 'foo BAR'
{{to-upper project}} # FOO BAR
{{to-lower project}} # foo bar
{{to-title project}} # Foo Bar
{{to-case 'snake' project}} # foo_bar
```

For the `to-case` helper, the following cases are supported:

- `toggle`
- `pascal`
- `camel`
- `upper-camel`
- `snake`
- `upper-snake`
- `screaming-snake`
- `kebab`
- `cobol`
- `train`
- `flat`
- `upper-flat`
- `alternating`

### Full Example

Finally, let's take a look at a full example using Brix to bootstrap a Java project. The `.config/brix` directory is conveniently located in HOME in order to be able to run `brix` from anywhere and create a project like this.

```yml
commands:
  - copy: # Copy all of the shared files to the project directory
      source: project/shared
      destination: .
      overwrite: true
  - template: # Template build.gradle.kts
      source: module/build.gradle.kts.hbs
      destination: "module/{{module}}/build.gradle.kts"
      overwrite: false
  - template: # Template module-info.java
      source: module/src/main/java/module-info.java.hbs
      destination: "module/{{module}}/src/main/java/module-info.java"
      overwrite: false
  - template: # Teplate package.json
      source: project/templates/shared/package.json.hbs
      destination: "package.json"
      overwrite: false
  - template: # Template settings.gradle.kts.hbs
      source: project/templates/shared/settings.gradle.kts.hbs
      destination: "settings.gradle.kts"
      overwrite: false
  - mkdir: # Create the main directory for the module
      destination: "module/{{module}}/src/main/java/com/example/{{module}}"
  - mkdir: # Create the test directory for the module
      destination: "module/{{module}}/src/test/java/com/example/{{module}}"
  - exec: # Run the following commands with no stdout
      commands:
        - git init
        - git add .
        - git commit -m initial
        - yarn up
      stdout: false
```

## More Examples

There are a few extra examples located in `./config/brix/rust`.

- **copy** `cargo run -- rust copy brix foo`
- **exec** `cargo run -- rust exec foo foo`
- **mkdir** `cargo run -- rust mkdir brix foo`
- **search_replace** `cargo run -- rust search_replace brix foo`
- **template** `cargo run -- rust template brix foo`
