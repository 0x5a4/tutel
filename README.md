# tutel
a minimalist todo app.

## What is this?
Tutel is a very small todo app allowing the user to create per-directory task-lists. Tutel can also recursively walk
the directory tree until a todo list is found.

## Installation
### Arch Linux
[There's an AUR package](https://aur.archlinux.org/packages/tutel).

### Gentoo
[My overlay](https://github.com/0x5a4/ruhtra) contains an ebuild.

### Everything else
A Linux-binary is provided in the [Release-Tab](https://github.com/0x5a4/tutel/releases).

## Usage
```
// Create a list within the current directory
tutel new

// Add a todo to the list
tutel add really important thing // Everything after the subcommand is merged, no quotes necessary

// Print the todo list
tutel

Output:
[X]test
000) [X]really important thing

// Mark the task as being completed
tutel done 0

// Edit the task, launches $EDITOR
tutel edit 0

// Remove it
tutel rm 0
```

