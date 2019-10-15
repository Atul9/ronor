use clap::{Arg, ArgMatches, App};
use ronor::Sonos;
use super::{Result, ArgMatchesExt};

pub const NAME: &str = "load-home-theater-playback";

pub fn build() -> App<'static, 'static> {
  App::new(NAME)
    .about("Signal a player to switch to its TV input (optical or HDMI)")
    .arg(super::household_arg())
    .arg(Arg::with_name("PLAYER").required(true)
           .help("Name of the player"))
}

pub fn run(sonos: &mut Sonos, matches: &ArgMatches) -> Result<()> {
  with_authorization!(sonos, {
    let household = matches.household(sonos)?;
    let targets = sonos.get_groups(&household)?;
    let player = matches.player(&targets.players)?;
    Ok(sonos.load_home_theater_playback(&player)?)
  })
}
