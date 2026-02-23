//! SportClient - high-level sport-mode API for the Go2 robot.
//!
//! TODO: Real implementation would use DDS RPC over CycloneDDS.
//! All methods currently log their invocation and return 0 (success) as stubs.

use super::sport_api::*;

/// Controls the Go2 robot's sport/locomotion modes.
pub struct SportClient {
    enable_lease: bool,
    timeout: f32,
}

impl SportClient {
    /// Create a new `SportClient`.
    pub fn new(enable_lease: bool) -> Self {
        Self {
            enable_lease,
            timeout: 10.0,
        }
    }

    /// Set the RPC timeout in seconds.
    pub fn set_timeout(&mut self, timeout: f32) {
        self.timeout = timeout;
    }

    /// Initialize the client (connect to DDS).
    pub fn init(&mut self) {
        log::info!(
            "SportClient init: service={} version={} enable_lease={}",
            ROBOT_SPORT_SERVICE_NAME,
            ROBOT_SPORT_API_VERSION,
            self.enable_lease
        );
    }

    fn call_api(&self, api_id: i32, parameter: &str) -> i32 {
        log::info!(
            "SportClient call: api_id={} parameter={}",
            api_id,
            parameter
        );
        0
    }

    /// Damp all motors.
    pub fn damp(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_DAMP, "{}")
    }

    /// Enter balance stand mode.
    pub fn balance_stand(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_BALANCESTAND, "{}")
    }

    /// Stop all movement.
    pub fn stop_move(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_STOPMOVE, "{}")
    }

    /// Stand up.
    pub fn stand_up(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_STANDUP, "{}")
    }

    /// Stand down (lie down).
    pub fn stand_down(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_STANDDOWN, "{}")
    }

    /// Recovery stand from fallen state.
    pub fn recovery_stand(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_RECOVERYSTAND, "{}")
    }

    /// Set body orientation (Euler angles in radians).
    pub fn euler(&self, roll: f32, pitch: f32, yaw: f32) -> i32 {
        let p = format!(r#"{{"x":{roll},"y":{pitch},"z":{yaw}}}"#);
        self.call_api(ROBOT_SPORT_API_ID_EULER, &p)
    }

    /// Move with velocity (m/s for vx/vy, rad/s for vyaw).
    pub fn move_cmd(&self, vx: f32, vy: f32, vyaw: f32) -> i32 {
        let p = format!(r#"{{"x":{vx},"y":{vy},"z":{vyaw}}}"#);
        self.call_api(ROBOT_SPORT_API_ID_MOVE, &p)
    }

    /// Sit down.
    pub fn sit(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_SIT, "{}")
    }

    /// Rise from sit.
    pub fn rise_sit(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_RISESIT, "{}")
    }

    /// Set speed level.
    pub fn speed_level(&self, level: i32) -> i32 {
        let p = format!(r#"{{"value":{level}}}"#);
        self.call_api(ROBOT_SPORT_API_ID_SPEEDLEVEL, &p)
    }

    /// Hello gesture.
    pub fn hello(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_HELLO, "{}")
    }

    /// Stretch pose.
    pub fn stretch(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_STRETCH, "{}")
    }

    /// Switch joystick mode.
    pub fn switch_joystick(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_SWITCHJOYSTICK, &p)
    }

    /// Content mode.
    pub fn content(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_CONTENT, "{}")
    }

    /// Heart gesture.
    pub fn heart(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_HEART, "{}")
    }

    /// Pose mode.
    pub fn pose(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_POSE, &p)
    }

    /// Scrape action.
    pub fn scrape(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_SCRAPE, "{}")
    }

    /// Front flip.
    pub fn front_flip(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_FRONTFLIP, "{}")
    }

    /// Front jump.
    pub fn front_jump(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_FRONTJUMP, "{}")
    }

    /// Front pounce.
    pub fn front_pounce(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_FRONTPOUNCE, "{}")
    }

    /// Dance routine 1.
    pub fn dance1(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_DANCE1, "{}")
    }

    /// Dance routine 2.
    pub fn dance2(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_DANCE2, "{}")
    }

    /// Left flip.
    pub fn left_flip(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_LEFTFLIP, "{}")
    }

    /// Back flip.
    pub fn back_flip(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_BACKFLIP, "{}")
    }

    /// Hand stand.
    pub fn hand_stand(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_HANDSTAND, &p)
    }

    /// Free walk mode.
    pub fn free_walk(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_FREEWALK, "{}")
    }

    /// Free bound mode.
    pub fn free_bound(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_FREEBOUND, &p)
    }

    /// Free jump mode.
    pub fn free_jump(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_FREEJUMP, &p)
    }

    /// Free avoid mode.
    pub fn free_avoid(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_FREEAVOID, &p)
    }

    /// Classic walk mode.
    pub fn classic_walk(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_CLASSICWALK, &p)
    }

    /// Walk upright mode.
    pub fn walk_upright(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_WALKUPRIGHT, &p)
    }

    /// Cross step mode.
    pub fn cross_step(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_CROSSSTEP, &p)
    }

    /// Set auto-recovery flag.
    pub fn auto_recover_set(&self, flag: bool) -> i32 {
        let p = format!(r#"{{"value":{}}}"#, flag as i32);
        self.call_api(ROBOT_SPORT_API_ID_AUTORECOVERY_SET, &p)
    }

    /// Get auto-recovery flag. Returns `Ok(bool)` or `Err(api_error_code)`.
    pub fn auto_recover_get(&self) -> Result<bool, i32> {
        log::info!("SportClient call: api_id={} parameter={{}}", ROBOT_SPORT_API_ID_AUTORECOVERY_GET);
        // Stub: always returns false
        Ok(false)
    }

    /// Static walk mode.
    pub fn static_walk(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_STATICWALK, "{}")
    }

    /// Trot run mode.
    pub fn trot_run(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_TROTRUN, "{}")
    }

    /// Economic gait mode.
    pub fn economic_gait(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_ECONOMICGAIT, "{}")
    }

    /// Switch avoid mode.
    pub fn switch_avoid_mode(&self) -> i32 {
        self.call_api(ROBOT_SPORT_API_ID_SWITCHAVOIDMODE, "{}")
    }
}
