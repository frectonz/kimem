# Kimem (ቅመም)

Safaricom Kimem MIFI CLI

## Installation

### Install via shell script (MacOS and Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/frectonz/kimem/releases/download/v0.1.4/kimem-installer.sh | sh
```

### Install via powershell script (Windows)

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/frectonz/kimem/releases/download/v0.1.4/kimem-installer.ps1 | iex"
```

### Nix (Build from source)

#### Fetch from [FlakeHub](https://flakehub.com/flake/frectonz/kimem)

```
nix shell "https://flakehub.com/f/frectonz/kimem/0.1.4"
```

#### Fetch from GitHub

```
nix shell github:frectonz/kimem
```
