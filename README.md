# Sonos smart speaker controller API and CLI

Linux: [![Build Status](https://travis-ci.org/mlang/ronor.svg?branch=master)](https://travis-ci.org/mlang/ronor)

This project implements (most of) the [Sonos control API] in a rust crate. It also provides a simple command-line tool which can be used in scripts.

You likely need a recent rust compiler.

Build with `cargo build`.

You unfortunately have to register a developer account on integration.sonos.com and create your own integration point. You also need to create your own redirection endpoint on the web. A minimalistic example script is provided in `static/sonos.php`.

Ideally, I'd like to find a way to make this part of it common, so that you dont have to register your own integration. However, I really dont know yet how to do this securely, input welcome.

With your integration information ready, just run `ronor init` and your client id, secret, and redirection url will be saved to `~/.config/ronor/`.

With that, you can authorize ronor to access households registered with your Sonos user account by running `ronor login`.

See `ronor help` for a list of available commands.

## Managing groups

Use the `modify-group` subcommand to manage grouping of logical players.

For example, imagine the following household of three players and no grouping.

```console
$ ronor inventory
Bad = Bad
Wohnzimmer = Wohnzimmer
Schlafzimmer = Schlafzimmer
```

That means, each player is the sole member of a group with the same name.

Now lets make a group of `Schlafzimmer` (bedroom) and `Bad` (Bathroom).

```console
$ ronor modify-group Schlafzimmer --add Bad
$ ronor inventory
Schlafzimmer + 1 = Schlafzimmer + Bad
Wohnzimmer = Wohnzimmer
```

To undo this group again, we simply do the following.

```console
$ ronor modify-group 'Schlafzimmer + 1' --remove Bad
$ ronor inventory
Bad = Bad
Wohnzimmer = Wohnzimmer
Schlafzimmer = Schlafzimmer
```

## Text to speech

For the text-to-speech functionality (`ronor speak`) you need `espeak` and `ffmpeg` installed. Simply pipe text to `STDIN` and it should arrive at the desired player.

Documentation is very scarce right now. Please refer to developer.sonos.com for documentation about the underlying API calls.

I am not a native english speaker, and just copying text from developer.sonos.com seems sort of wrong. PRs very welcome.

[Sonos control API]: https://developer.sonos.com/reference/control-api/
