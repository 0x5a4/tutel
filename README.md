# tutel
a minimalist todo app trying to integrate with your existing workflow.

## What is this?
`tutel` is built upon a simple principle: **One Todo-List per directory.**  
Why? Because you (propably) already have some kind of directory based
organization, so there really is no need for your todo app to redo it.  
Run it and `tutel` will either use the todo list(saved in a `.tutel.toml` file) from
your current directory or search upwards until one is found. 

## Feature Rundown
- [X] Add Todos
- [X] Edit existing Todos
- [X] Remove Todos
- [X] Remove all completed todos
- [X] Consistent Indices of Todos across removals
- [ ] Due dates
- [X] Really ugly list visualization that needs improvement(Help me [pls](https://github.com/0x5a4/tutel/issues/2))
- [X] Shell completions

## Installation

### Using cargo
`cargo install tutel`

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
[X] list with important things
001 │ [X]really important thing

// Mark the task as being completed
tutel done 0

// Edit the task, launches $EDITOR
tutel edit 0

// Remove it
tutel rm 0

// Or remove everything already completed
tutel rm --cleanup
```

## What are all those symbols in my todo list?
```
 ┌─ sums up if the whole list is completed or not
 │    ┌─ how many recursive steps were taken to reach this
 ▼    ▼
[X] [-1] project name 
001 │ [X]description
  ▲    ▲
  │    └─ the completion state of this task
  └─ the index of the task, used for referencing it in commands
```

## Why the name?
[This 🐢](https://youtu.be/oxzEdm29JLw)
