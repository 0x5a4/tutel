# tutel
a minimalist todo app.

## What is this?
Tutel is a very small todo app allowing the user to create per-directory task-lists. Tutel can also recursively walk
the directory tree until a todo list is found.

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

