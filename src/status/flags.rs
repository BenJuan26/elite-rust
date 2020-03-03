#![allow(dead_code)]

const DOCKED: u32 = 0x00000001;
const LANDED: u32 = 0x00000002;
const LANDING_GEAR_DOWN: u32 = 0x00000004;
const SHIELDS_UP: u32 = 0x00000008;
const SUPERCRUISE: u32 = 0x00000010;
const FLIGHT_ASSIST_OFF: u32 = 0x00000020;
const HARDPOINTS_DEPLOYED: u32 = 0x00000040;
const IN_WING: u32 = 0x00000080;
const LIGHTS_ON: u32 = 0x00000100;
const CARGO_SCOOP_DEPLOYED: u32 = 0x00000200;
const SILENT_RUNNING: u32 = 0x00000400;
const SCOOPING_FUEL: u32 = 0x00000800;
const SRV_HANDBRAKE: u32 = 0x00001000;
const SRV_TURRET: u32 = 0x00002000;
const SRV_UNDER_SHIP: u32 = 0x00004000;
const SRV_DRIVE_ASSIST: u32 = 0x00008000;
const FSD_MASS_LOCKED: u32 = 0x00010000;
const FSD_CHARGING: u32 = 0x00020000;
const FSD_COOLDOWN: u32 = 0x00040000;
const LOW_FUEL: u32 = 0x00080000;
const OVERHEATING: u32 = 0x00100000;
const HAS_LAT_LONG: u32 = 0x00200000;
const IS_IN_DANGER: u32 = 0x00400000;
const BEING_INTERDICTED: u32 = 0x00800000;
const IN_MAIN_SHIP: u32 = 0x01000000;
const IN_FIGHTER: u32 = 0x02000000;
const IN_SRV: u32 = 0x04000000;
const IN_ANALYSIS_MODE: u32 = 0x08000000;
const NIGHT_VISION: u32 = 0x10000000;
const ALTITUDE_FROM_AVERAGE_RADIUS: u32 = 0x20000000;
const FSD_JUMP: u32 = 0x40000000;
const SRV_HIGH_BEAM: u32 = 0x80000000;

const GUI_FOCUSNONE: u32 = 0;
const GUI_FOCUS_INTERNAL_PANEL: u32 = 1;
const GUI_FOCUS_EXTERNAL_PANEL: u32 = 2;
const GUI_FOCUS_COMMS_PANEL: u32 = 3;
const GUI_FOCUS_ROLE_PANEL: u32 = 4;
const GUI_FOCUS_STATION_SERVICES: u32 = 5;
const GUI_FOCUS_GALAXY_MAP: u32 = 6;
const GUI_FOCUS_SYSTEM_MAP: u32 = 7;
const GUI_FOCUS_ORRERY: u32 = 8;
const GUI_FOCUS_FSS_MODE: u32 = 9;
const GUI_FOCUS_SAA_MODE: u32 = 10;
const GUI_FOCUS_CODEX: u32 = 11;

const GUI_FOCUS_LEFT: u32 = GUI_FOCUS_EXTERNAL_PANEL;
const GUI_FOCUS_RIGHT: u32 = GUI_FOCUS_INTERNAL_PANEL;
const GUI_FOCUS_TOP: u32 = GUI_FOCUS_COMMS_PANEL;
const GUI_FOCUS_BOTTOM: u32 = GUI_FOCUS_ROLE_PANEL;

#[derive(Debug, Default)]
pub struct Flags {
    pub docked: bool,
    pub landed: bool,
    pub landing_gear_down: bool,
    pub shields_up: bool,
    pub supercruise: bool,
    pub flight_assist_off: bool,
    pub hardpoints_deployed: bool,
    pub in_wing: bool,
    pub lights_on: bool,
    pub cargo_scoop_deployed: bool,
    pub silent_running: bool,
    pub scooping_fuel: bool,
    pub srv_handbrake: bool,
    pub srv_turret: bool,
    pub srv_under_ship: bool,
    pub srv_drive_assist: bool,
    pub fsd_mass_locked: bool,
    pub fsd_charging: bool,
    pub fsd_cooldown: bool,
    pub low_fuel: bool,
    pub overheating: bool,
    pub has_lat_long: bool,
    pub is_in_danger: bool,
    pub being_interdicted: bool,
    pub in_main_ship: bool,
    pub in_fighter: bool,
    pub in_srv: bool,
    pub in_analysis_mode: bool,
    pub night_vision: bool,
    pub altitude_from_average_radius: bool,
    pub fsd_jump: bool,
    pub srv_high_beam: bool,
}

impl From<u32> for Flags {
    fn from(raw: u32) -> Self {
        Flags{
            docked: raw & DOCKED > 0,
            landed: raw & LANDED > 0,
            landing_gear_down: raw & LANDING_GEAR_DOWN > 0,
            shields_up: raw & SHIELDS_UP > 0,
            supercruise: raw & SUPERCRUISE > 0,
            flight_assist_off: raw & FLIGHT_ASSIST_OFF > 0,
            hardpoints_deployed: raw & HARDPOINTS_DEPLOYED > 0,
            in_wing: raw & IN_WING > 0,
            lights_on: raw & LIGHTS_ON > 0,
            cargo_scoop_deployed: raw & CARGO_SCOOP_DEPLOYED > 0,
            silent_running: raw & SILENT_RUNNING > 0,
            scooping_fuel: raw & SCOOPING_FUEL > 0,
            srv_handbrake: raw & SRV_HANDBRAKE > 0,
            srv_turret: raw & SRV_TURRET > 0,
            srv_under_ship: raw & SRV_UNDER_SHIP > 0,
            srv_drive_assist: raw & SRV_DRIVE_ASSIST > 0,
            fsd_mass_locked: raw & FSD_MASS_LOCKED > 0,
            fsd_charging: raw & FSD_CHARGING > 0,
            fsd_cooldown: raw & FSD_COOLDOWN > 0,
            low_fuel: raw & LOW_FUEL > 0,
            overheating: raw & OVERHEATING > 0,
            has_lat_long: raw & HAS_LAT_LONG > 0,
            is_in_danger: raw & IS_IN_DANGER > 0,
            being_interdicted: raw & BEING_INTERDICTED > 0,
            in_main_ship: raw & IN_MAIN_SHIP > 0,
            in_fighter: raw & IN_FIGHTER > 0,
            in_srv: raw & IN_SRV > 0,
            in_analysis_mode: raw & IN_ANALYSIS_MODE > 0,
            night_vision: raw & NIGHT_VISION > 0,
            altitude_from_average_radius: raw & ALTITUDE_FROM_AVERAGE_RADIUS > 0,
            fsd_jump: raw & FSD_JUMP > 0,
            srv_high_beam: raw & SRV_HIGH_BEAM > 0,
        }
    }
}
