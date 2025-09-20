# Ὅμηρος (Omiros) - a home manager for normies

Omiros is a command-line tool to semi-declaratively automate the setup of a new
environment on macOS. It's configured using a simple TOML file (`system.toml`)
and a directory containing your dotfiles.

## What does it do?

It currently supports:

- Installing Homebrew formulae and casks.
- Installing Mac App Store applications using `mas`.
- Installing VSCode extensions.
- Symlinking dotfiles from a specified `dotfiles/` directory to wherever you
  need.
- Setting some macOS system configuration, pretty much the ones that I care
  about.

## Getting Started

1.  **Create a `system.toml` file:**

    This file defines the packages and applications you want to install, and
    enumerates the dotfiles you want to link.

    ```toml
    # system.toml

    # brew formulae and casks as you would find in `brew search` or `brew info`.
    [brew]
    formulae = ["fish", "neovim", "git"]
    casks = ["alacritty", "slack"]

    # mas apps declared by both name and app id.
    [[mas.apps]]
    name = "Amphetamine"
    id = "937984704"

    [dotfiles]
    files = [
        # By default, omiros will symlink your dotfiles to the same path in your
        # home directory. For example, the following line will create a symlink
        # from `dotfiles/.config/alacritty/alacritty.toml` to
        # `~/.config/alacritty/alacritty.toml`.
        ".config/alacritty/alacritty.toml",
        ".config/fish/config.fish",

        # If you want to create a symlink to a different path, you can specify a
        # `original` and `link` path. The `original` path is relative to your
        # dotfiles directory, and the `link` path can be anywhere you want, but
        # I'm partial to XDG-compliant configuration paths.
        { original = ".config/git/config", link = "~/.gitconfig" }
    ]

    [vscode]
    extensions = [
        # Extension names can be found under "Unique Identifier" in the "More
        # Info" section of the extension Marketplace. For example VSCodeVim can
        # be found here:
        # https://marketplace.visualstudio.com/items?itemName=vscodevim.vim
        "vscodevim.vim",
        "rust-lang.rust-analyzer"
    ]

    [macos.dock]
    orientation = "left"
    autohide = true
    icon-size = 48

    [macos.safari]
    show-full-url = true

    [macos.system]
    show-file-extensions = true
    # Set scrolling to "natural", like an animal.
    weird-mac-scrolling = true
    ```

2.  **Organize your dotfiles:**

    Create a directory containing the dotfiles you want to manage. The paths in
    the `[dotfiles]` section of your `system.toml` are relative to this
    directory.

    ```
    .
    ├── dotfiles/
    │   ├── .config/
    │   │   ├── alacritty/
    │   │   │   └── alacritty.toml
    │   │   ├── fish/
    │   │   │   └── config.fish
    │   │   └── git/
    │   │       └── config
    │   └── .zshrc
    └── system.toml
    ```

    This is a great directory to track under version control.

3.  **Run the application:**

    Build the application using `cargo build --release`. Then, run the executable, providing the path to the directory containing your `system.toml` and the path to your `dotfiles` directory.

    ```bash
    ./target/release/omiros --system-config-dir . --dotfiles-dir ./dotfiles
    ```

    -   `--system-config-dir`: The path to the directory containing your `system.toml` file.
    -   `--dotfiles-dir`: The path to the directory containing your dotfiles.

The tool will then check for missing packages and applications and install them, and symlink your dotfiles.

## ...But Why?!

**tldr;** Cuz I'm too dumb to use Nix, but nothing else comes close!

Many years ago I wrote a tool like this in Python to help set up my own personal
machines. It worked, but I was always on the lookout for something better.

That's when I discovered Ansible. I thought, that's a cool sci-fi inspired name.
At the time I was also getting into configuration management systems at work to
set up large batches of test machines. So it seemed like a reasonable thing to
try and convert my Python script to be a declarative Ansible playbook. That also
worked, but Ansible is a pretty large hammer for this particular nail. Ansible
is great at setting up a lot of the same machines over as short period of time,
not a single machine one at a time over long periods of time, and I was
discovering that each time I went back to run my ansible playbook, I'd have to
tinker with it to get it to work again. I wasn't setting up new personal
machines at a fast enough cadence to pay back my investment in keeping my
Ansible playbook up-to-date.

Having come to that realization, I decided to start over, this time writing the
whole thing in Bash to avoid having any external dependencies. This also worked!
There seems to be a pattern forming. These scripts always seem to work, but I'm
never really happy with them.

The reason is because:

> _An idea is like a virus, resilient, highly contagious. The smallest seed of an
> idea can grow. It can grow to define or destroy you._
>
> -Cobb (Inception)

In my case it was Nix, and it destroyed me. I had been playing with Nix on the
side on-and-off for a few years at this point. Mostly learning about the
language and reading about the theory behind this very particular package
manager. At some point I got a new computer and I decided this is my chance!
Finally, I would figure out how to use Nix (both the language and the package
manager) to declaratively and reproducibly generate / maintain my development
environment. This is not a decision to be taken lightly. One does not simply
walk into using Nix; it permeates through everything that you do on your
computer. It's a completely different way of thinking. When it worked, it was
beautiful. Everything was reproducible, everything was declarative. `nix-darwin`
and `home-manager` opened my eyes to a new way of thinking about how to manage a
machine. But at the same time Nix itself was completely impenetrable. I read a
lot of blogs and a lot of docs, but it never quite clicked, and the longer it
didn't click the less I wanted to use my system, because making modifications to
it became a pain. Immutable configuration caused all sorts of problems with
applications that did not expect that to be the case. I ran into edge-cases at
every turn, and the more that I ran into these issues, the less I wanted to hack
on stuff.

This is not a critique of Nix. Purity is not free. There is a price to pay, and
in my particular case, the price was too high.

Oh, I also tried a few dotfile managers like `dotter`. They worked, and they
were great, but they mostly focused on dotfiles, and `nix-darwin`/`home-manager`
really showed me how nice it is to have a single place to both declare system
configuration, package management, _and_ dotfiles related configuration.

So here I am. I've written a script like this at least 4 times now. Maybe this
will be the last time. Maybe...

### Inspiration / Prior Art

- [ansible](https://docs.ansible.com)
- [Determinate Nix](https://docs.determinate.systems/determinate-nix/)
- [nix-darwin](https://github.com/nix-darwin/nix-darwin)
- [home-manager](https://github.com/nix-community/home-manager)
- [dotter](https://github.com/SuperCuber/dotter)

## Future Plans

- Support `apt` package manager on Linux machines.