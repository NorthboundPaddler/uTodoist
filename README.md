# μTodoist

A simple CLI tool written in Rust to quickly list, add, and complete tasks from Todoist using their official API.

At it's core, `utodoist` will only query tasks with a filter, add tasks using NLP, and complete tasks by ID. All of this is done via a simple CLI, and there's nothing else to it.

## Quickstart

There is only three things you can do with this CLI tool: query, add, and complete tasks. We'll run through each of them individually.

> [NOTE]
> An easy way to use this tool is by setting your Todoist API key to an environment variable.
> On linux, that is done like so:
> `$ export API_KEY=foobar`

### Query tasks

You will need a filter querystring and a Todoist API key. You can simply get your tasks back using the following command:

```sh
utodoist list -f "today" -k "$API_KEY"
```

### Add task

You will need a "quick add" string of text and a Todoist API key. This utilizes Todoist's natural language processing, and can be used with the following command:

```sh
utodoist add -t "tomorrow Get Milk @errand" -k "$API_KEY"
```

### Complete task

This subcommand requires a task ID and a Todoist API key. By specifying both of those arguments, a task will be completed by using the following command:

```sh
utodoist complete -i 1234567890 -k "$API_KEY"
```

## Development

To utilize the environment defined in the Nix-based `flake.nix` file, you can drop into a shell with all the required dependencies to build this project using the following command:

```sh
nix develop 
```

If you aren't using Nix for your development machine, well...

### Packaging in Nix

I'm still a bit new to this, so be mindful that this section is all trial and error.

First, prefetch the has using the following:

```sh
nix-shell -p nix-prefetch-git
nix-prefetch-git https://github.com/NorthboundPaddler/uTodoist
```

Then, copy the `hash` key's value from the resulting output to the `sha256` attribute in `default.nix`. Also, update the `rev` attribute with the latest release name. For example, it might look like `v0.1.0-alpha`.

You can test if the project builds by running `nix build` and finding the binary in the `result` directory.

## Roadmap

- [x] Write the beginnings of a README.md
- [x] Get a request to go through
- [x] Query tasks with filter
- [x] Add task using Natural Language Processing (NLP)
- [x] Complete task by ID
- [x] Implement both JSON output and pretty-printed output
- [ ] Handle errors gracefully and standardize output for user
- [ ] Write *quality* documentation/man pages
- [ ] Write the finishings of a README.md
- [ ] Package for distribution in Cargo/Nixpkgs
