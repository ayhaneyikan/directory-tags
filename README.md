# Directory Tags

Utility for quickly navigating around your filesystem

## Setup

Clone and build the binary using:
```
cargo build --release
```

Update `cdt_wrapper.sh` `PROJECT_DIR` variable with the path to the cloned repository.
Then source `cdt_wrapper.sh` in your `.bashrc` so that you can easily execute the binary.
The wrapper file provides the ability to actualy change directories based on the binary's output.
It also provides autocomplete for the directory tags.

## Usage

#### Add Directory Tag

Use the 'add' argument `-a`, `--add` to add the current directory with the given tag
```
# cdt --add <tag-name>
cdt -a cur-dir-tag
```

Optionally, you can use the 'path' argument `-p`, `--path` to specify the directory to map the tag to
```
# cdt --add <tag-name> --path <absolute-directory-path>
cdt -a dev -p ~/dev/
```

#### Jump to Tagged Directory

Supplying a tag name directly to the command will cd you to the associated directory
(Note: make sure you set up the `cdt_wrapper.sh` script properly for this to work)
```
# cdt <tag-name>
cdt dev
```
`cdt_wrapper.sh` also provides autocomplete for available tags

#### Viewing Created Tags

If curious about your tags, use the 'list' argument `-l`, `--list` to view the mappings you've created
```
cdt -l
```
Note that there's also a variant of this argument `--list-tags` which lists only the tag names.
This is used to create the bash autocomplete function in `cdt_wrapper.sh`.

## Future Functionality?

Possible plans to add functionality like CLI tag removal.
(If necessary you can just manually remove entries the `.directory-tags` file saved in your home directory)

Also could use some better autocomplete for the CLI arguments and options.
