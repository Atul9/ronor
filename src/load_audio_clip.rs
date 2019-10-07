use clap::{Arg, ArgMatches, App};
use ronor::Sonos;
use std::process::exit;
use super::{find_player_by_name, Result};
use url::Url;

pub fn build() -> App<'static, 'static> {
  App::new("load-audio-clip")
    .about("Schedule an audio clip to play on a particular player")
    .arg(Arg::with_name("NAME").default_value("ronor clip")
           .short("n").long("name").takes_value(true))
    .arg(Arg::with_name("APP_ID").default_value("guru.blind").value_name("STRING")
           .short("i").long("app-id").takes_value(true))
    .arg(Arg::with_name("CLIP_TYPE").short("t").long("type").takes_value(true)
           .possible_values(&["Chime", "Custom"]))
    .arg(Arg::with_name("PRIORITY").short("p").long("priority").takes_value(true)
           .possible_values(&["Low", "High"]))
    .arg(Arg::with_name("VOLUME").short("v").long("volume").takes_value(true)
           .help("Volume in percent (0-100)"))
    .arg(Arg::with_name("HTTP_AUTHORIZATION")
           .short("a").long("http-authorization").takes_value(true).value_name("STRING")
           .help("HTTP Authorization string"))
    .arg(Arg::with_name("PLAYER").required(true)
           .help("Name of the player"))
    .arg(Arg::with_name("URL").required(true)
           .help("Location of the audio clip"))
}

pub fn run(sonos: &mut Sonos, matches: &ArgMatches) -> Result<()> {
  with_authorization!(sonos, {
    with_player!(sonos, matches, player, {
      let url = value_t!(matches, "URL", Url).unwrap();
      if url.has_host() {
        sonos.load_audio_clip(&player,
          matches.value_of("APP_ID").unwrap(),
          matches.value_of("NAME").unwrap(),
          match matches.value_of("CLIP_TYPE") {
            Some(s) => Some(s.parse::<>()?),
            None => None
          }, match matches.value_of("PRIORITY") {
            Some(s) => Some(s.parse::<>()?),
            None => None
          }, match matches.value_of("VOLUME") {
            Some(s) => Some(s.parse::<>()?),
            None => None
          }, matches.value_of("HTTP_AUTHORIZATION"), Some(&url)
        )?;
      } else {
        println!("The URL you provided does not look like Sonos will be able to reach it");
        exit(1);
      }
      Ok(())
    })
  })
}
