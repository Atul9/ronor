use clap::{Arg, ArgMatches, App};
use ronor::Sonos;
use std::process::{Command, Stdio};
use crate::{Result, ResultExt, ArgMatchesExt};
use url::Url;

pub const NAME: &str = "speak";

pub fn build() -> App<'static, 'static> {
  App::new(NAME)
    .about("Send synthetic speech to a player")
    .arg(crate::household_arg())
    .arg(Arg::with_name("LANGUAGE").short("l").long("language").takes_value(true)
           .default_value("en"))
    .arg(Arg::with_name("WORDS_PER_MINUTE").short("s").long("speed").takes_value(true)
           .default_value("250"))
    .arg(Arg::with_name("VOLUME").short("v").long("volume").takes_value(true)
           .default_value("75"))
    .arg(Arg::with_name("PLAYER").required(true)
           .help("Name of the player"))
}

pub fn run(sonos: &mut Sonos, matches: &ArgMatches) -> Result<()> {
  let household = matches.household(sonos)?;
  let targets = sonos.get_groups(&household)?;
  let player = matches.player(&targets.players)?;
  let mut args = vec![ String::from("-w")
                     , String::from("/dev/stdout")
                     , String::from("--stdin")];
  if let Some(language) = matches.value_of("LANGUAGE") {
    args.extend(vec![String::from("-v"), language.to_string()]);
  }
  if let Some(wpm) = matches.value_of("WORDS_PER_MINUTE") {
    args.extend(vec![String::from("-s"), wpm.to_string()]);
  }
  if let Some(volume) = matches.value_of("VOLUME") {
    let volume = volume.parse::<u8>()? * 2;
    args.extend(vec![String::from("-a"), volume.to_string()]);
  }
  let espeak = Command::new("espeak")
    .args(args)
    .stdout(Stdio::piped()).spawn()
    .chain_err(|| "Failed to spawn speech synthesizer")?;
  if let Some(stdout) = espeak.stdout {
    let ffmpeg = Command::new("ffmpeg")
      .args(&["-i", "-", "-v", "fatal", "-b:a", "96k", "-f", "mp3", "-"])
      .stdin(stdout).stdout(Stdio::piped()).spawn()
      .chain_err(|| "Failed to spawn audio encoder")?;
    if let Some(stdout) = ffmpeg.stdout {
      let curl = Command::new("curl")
        .args(&["--upload-file", "-", "https://transfer.sh/espeak.mp3"])
        .stdin(stdout).output()
        .chain_err(|| "Failed to spawn uploader")?;
      if curl.status.success() {
        let url = Url::parse(&String::from_utf8_lossy(&curl.stdout))?;
        let _clip = sonos.load_audio_clip(&player,
          "guru.blind",
          "ping",
          None,
          None,
          None,
          None,
          Some(&url)
        )?;
      } else {
        return Err("curl failed".into());
      }
    }
  }
  Ok(())
}