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

You will need a filter querystring and a Todoist API key. You can simply get your tasks back as JSON using the following command:

```sh
utodoist list -f "today" -k "$API_KEY"
```

## Development

To utilize the environment defined in the Nix-based `flake.nix` file, you can drop into a shell with all the required dependencies to build this project using the following command:

```sh
nix develop 
```

If you aren't using Nix for your development machine, well...

## Roadmap

- [x] Write the beginnings of a README.md
- [x] Get a request to go through
- [x] Query tasks with filter
- [ ] Add task using Natural Language Processing (NLP)
- [ ] Complete task by ID
- [ ] Implement both JSON output and pretty-printed output
- [ ] Write documentation/man pages
- [ ] Write the finishings of a README.md
- [ ] Package for distribution in Cargo/Nixpkgs
