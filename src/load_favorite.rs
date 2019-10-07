use clap::{Arg, ArgMatches, App};
use ronor::{Sonos, PlayModes};
use super::{find_favorite_by_name, find_group_by_name, Result};

pub fn build() -> App<'static, 'static> {
  App::new("load-favorite")
    .about("Load the specified favorite in a group")
    .arg(Arg::with_name("PLAY").short("p").long("play")
           .help("Automatically start playback"))
    .arg(Arg::with_name("REPEAT").short("r").long("repeat"))
    .arg(Arg::with_name("REPEAT_ONE").short("o").long("repeat-one"))
    .arg(Arg::with_name("CROSSFADE").short("c").long("crossfade")
           .help("Do crossfade between tracks"))
    .arg(Arg::with_name("SHUFFLE").short("s").long("shuffle")
           .help("Shuffle the tracks"))
    .arg(Arg::with_name("FAVORITE").required(true))
    .arg(Arg::with_name("GROUP").required(true))
}

pub fn run(sonos: &mut Sonos, matches: &ArgMatches) -> Result<()> {
  with_authorization!(sonos, {
    with_favorite!(sonos, matches, favorite, {
      with_group!(sonos, matches, group, {
        let repeat = matches.is_present("REPEAT");
        let repeat_one = matches.is_present("REPEAT_ONE");
        let crossfade = matches.is_present("CROSSFADE");
        let shuffle = matches.is_present("SHUFFLE");
        Ok(sonos.load_favorite(&group, &favorite,
          matches.is_present("PLAY"),
          if repeat || repeat_one || crossfade || shuffle {
            Some(PlayModes { repeat, repeat_one, crossfade, shuffle })
          } else {
            None
          })?
        )
      })
    })
  })
}
