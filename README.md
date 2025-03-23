# μTodoist

This simple CLI tool is only useful for quick terminal usage or for a building block for other programs. If you're looking for something fully featured, look elsewhere!

At it's core, `utodoist` will only query tasks with a filter, add tasks, and complete tasks. All of this is done via a simple CLI, and there's nothing else. If you want to extend it into a TUI, or use it within your giant monolith of a corporate software project... go for it! This is here to satisfy basic needs, and therefore provide no training wheels. I just want something fast and easy to use to query my tasks for the day in the CLI, rather than opening the official Todoist app that is built with Electron... It's not winning any drag races on my Linux host.

I wrote this in Rust so that I could wrap my mind around a different programming paradigm, given I have many years experience writing Python. This likely isn't a high-quality tool, but that's why it's mean to be kept lean and basic.

## Quickstart

There is only three things you can do with this CLI tool: query, add, and complete tasks. We'll run through each of them individually.

> [NOTE]
> An easy way to use this tool is by setting your Todoist API key to an environment variable.
> On linux, that is done like so:
> `$ export API_KEY=foobar`

### Query tasks

You will need a filter querystring and a Todoist API key. You can simply get your tasks back as JSON using the following command:

```sh
$ utodoist -f "today" -k "$API_KEY"
```

## Development

To utilize the environment defined in the Nix-based `flake.nix` file, you can drop into a shell with all the required dependencies to build this project using the following command:
```sh 
$ nix develop 
```

If you aren't using Nix for your development machine, well... Google around for how to develop in Rust then :)

## Roadmap

- [x] Write the beginnings of a README.md
- [x] Get a request to go through
- [x] Query tasks with filter
- [ ] Add task
- [ ] Complete task by ID
- [ ] Implement both JSON output and pretty-printed output
- [ ] Write documentation/man pages
- [ ] Write the finishings of a README.md
- [ ] Package for distribution in Cargo/Nixpkgs
