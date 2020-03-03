#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

use elite;

lazy_static! {
    static ref DATA_DIR: PathBuf = get_data_dir();
}

fn get_data_dir() -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("data");

    path
}

#[test]
fn it_returns_star_system() {
    assert_eq!(elite::star_system::get_from_path(DATA_DIR.as_path()).unwrap(), String::from("Sol"));
}

#[test]
fn it_returns_status() {
    let status = elite::status::get_from_path(DATA_DIR.as_path()).unwrap();
    assert_eq!(status.pips[1], 8);
    assert_eq!(status.timestamp, String::from("2020-03-03T05:03:21Z"));

    assert!(status.flags.docked);
    assert!(!status.flags.landed);
    assert!(status.flags.landing_gear_down);
    assert!(status.flags.shields_up);
    assert!(!status.flags.supercruise);
    assert!(!status.flags.flight_assist_off);
    assert!(!status.flags.hardpoints_deployed);
    assert!(!status.flags.in_wing);
    assert!(!status.flags.lights_on);
    assert!(!status.flags.cargo_scoop_deployed);
    assert!(!status.flags.silent_running);
    assert!(!status.flags.scooping_fuel);
    assert!(!status.flags.srv_handbrake);
    assert!(!status.flags.srv_turret);
    assert!(!status.flags.srv_under_ship);
    assert!(!status.flags.srv_drive_assist);
    assert!(status.flags.fsd_mass_locked);
    assert!(!status.flags.fsd_charging);
    assert!(!status.flags.fsd_cooldown);
    assert!(!status.flags.low_fuel);
    assert!(!status.flags.overheating);
    assert!(status.flags.has_lat_long);
    assert!(!status.flags.is_in_danger);
    assert!(!status.flags.being_interdicted);
    assert!(status.flags.in_main_ship);
    assert!(!status.flags.in_fighter);
    assert!(!status.flags.in_srv);
    assert!(!status.flags.in_analysis_mode);
    assert!(!status.flags.night_vision);
    assert!(!status.flags.altitude_from_average_radius);
    assert!(!status.flags.fsd_jump);
    assert!(!status.flags.srv_high_beam);
}
