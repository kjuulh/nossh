# nossh

A lightning-fast SSH endpoint finder and launcher with fuzzy history lookup and ⬢ `.ssh/config` reference.

---

## Features

- **Fuzzy history search**  
  Quickly fuzzy-find and re-run past `ssh` commands you’ve executed.

- **`.ssh/config` lookup**  
  Instantly search host entries from your `~/.ssh/config`.

- **Interactive mode**  
  Presents a searchable list of endpoints for one-shot selection.

- **External invocation**  
  Falls back to standard `ssh` when provided arguments.

- **Configurable config path**  
  Override the default `~/.ssh/config` via an `--ssh-config-path` flag or `SSH_CONFIG_PATH` env var.

---

## Installation

```bash
cargo install nossh
````

Or build from source:

```bash
git clone https://github.com/yourorg/nossh.git
cd nossh
cargo build --release
cp target/release/nossh /usr/local/bin/
```

---

## Quickstart

1. **Launch interactive search**

   ```bash
   nossh
   ```

   * Fuzzy-find against:

     * Your cached history of `ssh …` commands.
     * Host entries in your `~/.ssh/config`.

2. **Run a past command or config entry**

   ```bash
   # Start typing “git.fr” and press Enter:
   $ nossh
     1. ratchet:22
     2. somegateway:222
   # 3. git.front.kjuulh.io
   > git.fr
   # then Enter runs:
   ssh git.front.kjuulh.io
   ```

3. **Direct `ssh` passthrough**

   ```bash
   nossh user@host -p 2222
   ```

   Behaves exactly like `ssh …`; also logs the invocation for future fuzzy lookup.

---

## Usage

```text
nossh [--ssh-config-path <path>] [<ssh-args>...]
```

| Option                   | Description                                                                                    |
| ------------------------ | ---------------------------------------------------------------------------------------------- |
| `--ssh-config-path PATH` | Path to your SSH `config` file (default: `~/.ssh/config`). Overrides `SSH_CONFIG_PATH` env var |
| `<ssh-args>…`            | If provided, forwarded to `ssh` as normal.                                                     |

### Environment

* `SSH_CONFIG_PATH`
  Alternative way to point to your SSH config:

  ```bash
  export SSH_CONFIG_PATH="/custom/path/config"
  ```

---

## Configuration

The default SSH config path is `${HOME}/.ssh/config`. If that file is missing or you wish to use a different config, set the flag or env var:

```bash
nossh --ssh-config-path /mnt/shared/ssh_config
```

---

## How It Works

1. **Startup**

   * Reads `~/.ssh/config` (or your override).
   * Loads your SSH‐command history database (`~/.local/share/nossh/db.sqlite3`).

2. **Interactive Mode**

   * Presents a fuzzy prompt (via `fzf`-style) combining:

     * Host patterns from config.
     * Host strings from history (e.g. `user@host -p 2222`).

3. **Selection**

   * On Enter, launches `ssh` with the selected arguments.
   * In external (non-interactive) mode, simply proxies `ssh …` and appends to history.

---

## Examples

* **Basic interactive**

  ```bash
  nossh
  ```

* **Forced interactive**

  ```bash
  nossh interactive
  ```

* **One-liner passthrough**

  ```bash
  nossh -i ~/.ssh/id_ed25519 root@example.com
  ```

---

## Development

Clone and run tests:

```bash
git clone https://github.com/kjuulh/nossh.git
cd nossh
cargo test
```

---

## License

MIT-licensed

```

