<h1 align="center"> proj-cmd </h1>

<p align="center">
A simple project organizer written in rust using clap. Divedes your projects into subgroups called projgrps which can have their own projects.
</p>

## Installation

### Step 1
install the `proj-cmd` command -:

```zsh
cargo install proj-cmd
```

### Step 2


<details>
<summary>zsh</summary>
Add the following to your shell's config file -:

```zsh
eval "$(proj-cmd init zsh)"
```
</details>

<details>
<summary>bash</summary>
Add the following to your shell's config file -:

```zsh
eval "$(proj-cmd init bash)"
```
</details>

<details>
<summary>fish</summary>
Add the following to your shell's config file -:

```zsh
eval "$(proj-cmd init fish)"
```
</details>

<details>
<summary>nu</summary>

run this command `proj-cmd init nu | save -f ~/.proj.nu`
Add the following to your shell's config file -:
```zsh
source ~/.proj.nu
```
</details>

> The init command currenly supports `bash`, `zsh`, `nu` & `fish`

> By default, the init command generates a function named `proj`, you can pass in your own as `proj-cmd init <shell> my-cmd`

---

## Commands 

### `goto`

```zsh 
proj goto <projgrp>
proj goto <projgrp> <project>
```
Cd into a project or projgrp
> cd into projgrp if project is omitted.

### `make`
```zsh
proj make <projgrp>
```
Make a new projgrp

### `create`
```zsh 
proj create <projgrp> <project>
```
Create a new project in specified projgrp
### `list`
```zsh 
proj list
proj list <projgrp>
```
List all proj groups or projects in specified proj group

### `setup`
```zsh 
proj setup
proj setup <proj_homepath>
```
Set or get the home path

### `zip`
```zsh 
proj zip <projgrp>
proj zip <projgrp> <project>
```
Compress a project or proj group into a zip file

---
