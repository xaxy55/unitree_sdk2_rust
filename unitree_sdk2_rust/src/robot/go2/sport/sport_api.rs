//! Sport API service name, version, and method ID constants.
//!
//! These constants map sport-mode RPC methods to their numeric IDs
//! used in the DDS request/response protocol.

/// Sport service name used in DDS topic registration.
pub const ROBOT_SPORT_SERVICE_NAME: &str = "sport";
/// Sport API version string.
pub const ROBOT_SPORT_API_VERSION: &str = "1.0.0.1";

// ---- Basic locomotion ----

/// Damp all motors (emergency soft stop).
pub const ROBOT_SPORT_API_ID_DAMP: i32 = 1001;
/// Enter balance stand mode.
pub const ROBOT_SPORT_API_ID_BALANCESTAND: i32 = 1002;
/// Stop all movement.
pub const ROBOT_SPORT_API_ID_STOPMOVE: i32 = 1003;
/// Stand up from lying position.
pub const ROBOT_SPORT_API_ID_STANDUP: i32 = 1004;
/// Lie down from standing position.
pub const ROBOT_SPORT_API_ID_STANDDOWN: i32 = 1005;
/// Recovery stand after a fall.
pub const ROBOT_SPORT_API_ID_RECOVERYSTAND: i32 = 1006;
/// Set body Euler angles (roll, pitch, yaw).
pub const ROBOT_SPORT_API_ID_EULER: i32 = 1007;
/// Move with velocity command (vx, vy, vyaw).
pub const ROBOT_SPORT_API_ID_MOVE: i32 = 1008;
/// Sit down.
pub const ROBOT_SPORT_API_ID_SIT: i32 = 1009;
/// Rise from sitting position.
pub const ROBOT_SPORT_API_ID_RISESIT: i32 = 1010;

// ---- Speed and control ----

/// Set speed level (0-2).
pub const ROBOT_SPORT_API_ID_SPEEDLEVEL: i32 = 1015;
/// Hello wave gesture.
pub const ROBOT_SPORT_API_ID_HELLO: i32 = 1016;
/// Stretch pose.
pub const ROBOT_SPORT_API_ID_STRETCH: i32 = 1017;
/// Content/happy gesture.
pub const ROBOT_SPORT_API_ID_CONTENT: i32 = 1020;

// ---- Dance and tricks ----

/// Dance routine 1.
pub const ROBOT_SPORT_API_ID_DANCE1: i32 = 1022;
/// Dance routine 2.
pub const ROBOT_SPORT_API_ID_DANCE2: i32 = 1023;
/// Switch joystick control mode.
pub const ROBOT_SPORT_API_ID_SWITCHJOYSTICK: i32 = 1027;
/// Pose mode.
pub const ROBOT_SPORT_API_ID_POSE: i32 = 1028;
/// Scrape/paw gesture.
pub const ROBOT_SPORT_API_ID_SCRAPE: i32 = 1029;
/// Front flip.
pub const ROBOT_SPORT_API_ID_FRONTFLIP: i32 = 1030;
/// Front jump.
pub const ROBOT_SPORT_API_ID_FRONTJUMP: i32 = 1031;
/// Front pounce.
pub const ROBOT_SPORT_API_ID_FRONTPOUNCE: i32 = 1032;
/// Heart gesture.
pub const ROBOT_SPORT_API_ID_HEART: i32 = 1036;

// ---- Gait modes ----

/// Static walk gait.
pub const ROBOT_SPORT_API_ID_STATICWALK: i32 = 1061;
/// Trot running gait.
pub const ROBOT_SPORT_API_ID_TROTRUN: i32 = 1062;
/// Economic (energy-saving) gait.
pub const ROBOT_SPORT_API_ID_ECONOMICGAIT: i32 = 1063;

// ---- Advanced modes (2xxx range) ----

/// Left flip.
pub const ROBOT_SPORT_API_ID_LEFTFLIP: i32 = 2041;
/// Back flip.
pub const ROBOT_SPORT_API_ID_BACKFLIP: i32 = 2043;
/// Hand stand.
pub const ROBOT_SPORT_API_ID_HANDSTAND: i32 = 2044;
/// Free walk mode (autonomous).
pub const ROBOT_SPORT_API_ID_FREEWALK: i32 = 2045;
/// Free bound mode.
pub const ROBOT_SPORT_API_ID_FREEBOUND: i32 = 2046;
/// Free jump mode.
pub const ROBOT_SPORT_API_ID_FREEJUMP: i32 = 2047;
/// Free obstacle avoidance mode.
pub const ROBOT_SPORT_API_ID_FREEAVOID: i32 = 2048;
/// Classic walk mode.
pub const ROBOT_SPORT_API_ID_CLASSICWALK: i32 = 2049;
/// Walk upright (bipedal) mode.
pub const ROBOT_SPORT_API_ID_WALKUPRIGHT: i32 = 2050;
/// Cross step mode.
pub const ROBOT_SPORT_API_ID_CROSSSTEP: i32 = 2051;
/// Enable/disable automatic fall recovery.
pub const ROBOT_SPORT_API_ID_AUTORECOVERY_SET: i32 = 2054;
/// Query automatic fall recovery status.
pub const ROBOT_SPORT_API_ID_AUTORECOVERY_GET: i32 = 2055;
/// Switch obstacle avoidance mode.
pub const ROBOT_SPORT_API_ID_SWITCHAVOIDMODE: i32 = 2058;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_constants() {
        assert_eq!(ROBOT_SPORT_SERVICE_NAME, "sport");
        assert_eq!(ROBOT_SPORT_API_VERSION, "1.0.0.1");
    }

    #[test]
    fn api_ids_are_unique() {
        let ids = [
            ROBOT_SPORT_API_ID_DAMP,
            ROBOT_SPORT_API_ID_BALANCESTAND,
            ROBOT_SPORT_API_ID_STOPMOVE,
            ROBOT_SPORT_API_ID_STANDUP,
            ROBOT_SPORT_API_ID_STANDDOWN,
            ROBOT_SPORT_API_ID_RECOVERYSTAND,
            ROBOT_SPORT_API_ID_EULER,
            ROBOT_SPORT_API_ID_MOVE,
            ROBOT_SPORT_API_ID_SIT,
            ROBOT_SPORT_API_ID_RISESIT,
            ROBOT_SPORT_API_ID_SPEEDLEVEL,
            ROBOT_SPORT_API_ID_HELLO,
            ROBOT_SPORT_API_ID_STRETCH,
            ROBOT_SPORT_API_ID_CONTENT,
            ROBOT_SPORT_API_ID_DANCE1,
            ROBOT_SPORT_API_ID_DANCE2,
            ROBOT_SPORT_API_ID_SWITCHJOYSTICK,
            ROBOT_SPORT_API_ID_POSE,
            ROBOT_SPORT_API_ID_SCRAPE,
            ROBOT_SPORT_API_ID_FRONTFLIP,
            ROBOT_SPORT_API_ID_FRONTJUMP,
            ROBOT_SPORT_API_ID_FRONTPOUNCE,
            ROBOT_SPORT_API_ID_HEART,
            ROBOT_SPORT_API_ID_STATICWALK,
            ROBOT_SPORT_API_ID_TROTRUN,
            ROBOT_SPORT_API_ID_ECONOMICGAIT,
            ROBOT_SPORT_API_ID_LEFTFLIP,
            ROBOT_SPORT_API_ID_BACKFLIP,
            ROBOT_SPORT_API_ID_HANDSTAND,
            ROBOT_SPORT_API_ID_FREEWALK,
            ROBOT_SPORT_API_ID_FREEBOUND,
            ROBOT_SPORT_API_ID_FREEJUMP,
            ROBOT_SPORT_API_ID_FREEAVOID,
            ROBOT_SPORT_API_ID_CLASSICWALK,
            ROBOT_SPORT_API_ID_WALKUPRIGHT,
            ROBOT_SPORT_API_ID_CROSSSTEP,
            ROBOT_SPORT_API_ID_AUTORECOVERY_SET,
            ROBOT_SPORT_API_ID_AUTORECOVERY_GET,
            ROBOT_SPORT_API_ID_SWITCHAVOIDMODE,
        ];
        let mut sorted = ids.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len(), "API IDs must be unique");
    }
}
