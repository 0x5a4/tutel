# tutel
Most people organize their projects in folders. If something doesnt fit in a single file they make a subfolder.
And thats how tutel is organized as well! Run ``tutel new`` in a directory and it becomes a project.
You can now add tasks to this project, mark them as being done and so forth. If something is so complex
you feel like you could use some sort of category, just make a subfolder. 

# tutelnav
*a little bonus*
All your projects and their locations are bookmarked in a per user file.
You can directly go there by using ``tutelnav <name>``.

For this to work you need to add a line to your shell config:
``
fish(config.fish):
tutel init fish | source

bash(.bashrc):
eval "$(tutel init bash)"

``
