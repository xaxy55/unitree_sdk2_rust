//! SportClient — high-level sport-mode API for the Go2 robot.
//!
//! Provides a convenient method for every sport API command. All methods
//! currently log their invocation and return 0 (success) as stubs.
//!
//! > **Note:** Real implementation would use DDS RPC over CycloneDDS.

use super::sport_api::*;

/// Controls the Go2 robot's sport/locomotion modes.
///
/// Create with [`SportClient::new`], call [`init`](Self::init) once, then
/// use any of the 40+ command methods to control the robot.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::robot::go2::sport::SportClient;
///
/// let mut client = SportClient::new(false);
/// client.init();
/// assert_eq!(client.stand_up(), 0);
/// assert_eq!(client.move_cmd(0.5, 0.0, 0.0), 0);
/// assert_eq!(client.stop_move(), 0);
/// ```
pub struct SportClient {
    enable_lease: bool,
    timeout: f32,
}

impl SportClient {
    /// Create a new `SportClient`.
    ///
    /// Set `enable_lease` to `true` to request an exclusive control lease.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_client() -> SportClient {
        let mut c = SportClient::new(false);
        c.init();
        c
    }

    #[test]
    fn new_sets_defaults() {
        let client = SportClient::new(false);
        assert!(!client.enable_lease);
        assert_eq!(client.timeout, 10.0);
    }

    #[test]
    fn new_with_lease() {
        let client = SportClient::new(true);
        assert!(client.enable_lease);
    }

    #[test]
    fn set_timeout() {
        let mut client = SportClient::new(false);
        client.set_timeout(5.0);
        assert_eq!(client.timeout, 5.0);
    }

    #[test]
    fn basic_commands_return_zero() {
        let c = make_client();
        assert_eq!(c.damp(), 0);
        assert_eq!(c.balance_stand(), 0);
        assert_eq!(c.stop_move(), 0);
        assert_eq!(c.stand_up(), 0);
        assert_eq!(c.stand_down(), 0);
        assert_eq!(c.recovery_stand(), 0);
        assert_eq!(c.sit(), 0);
        assert_eq!(c.rise_sit(), 0);
        assert_eq!(c.hello(), 0);
        assert_eq!(c.stretch(), 0);
        assert_eq!(c.content(), 0);
        assert_eq!(c.heart(), 0);
        assert_eq!(c.scrape(), 0);
    }

    #[test]
    fn parameterized_commands_return_zero() {
        let c = make_client();
        assert_eq!(c.euler(0.1, 0.2, 0.3), 0);
        assert_eq!(c.move_cmd(1.0, 0.0, 0.5), 0);
        assert_eq!(c.speed_level(2), 0);
        assert_eq!(c.switch_joystick(true), 0);
        assert_eq!(c.pose(true), 0);
        assert_eq!(c.hand_stand(false), 0);
    }

    #[test]
    fn trick_commands_return_zero() {
        let c = make_client();
        assert_eq!(c.front_flip(), 0);
        assert_eq!(c.front_jump(), 0);
        assert_eq!(c.front_pounce(), 0);
        assert_eq!(c.dance1(), 0);
        assert_eq!(c.dance2(), 0);
        assert_eq!(c.left_flip(), 0);
        assert_eq!(c.back_flip(), 0);
    }

    #[test]
    fn gait_commands_return_zero() {
        let c = make_client();
        assert_eq!(c.free_walk(), 0);
        assert_eq!(c.free_bound(true), 0);
        assert_eq!(c.free_jump(true), 0);
        assert_eq!(c.free_avoid(true), 0);
        assert_eq!(c.classic_walk(true), 0);
        assert_eq!(c.walk_upright(true), 0);
        assert_eq!(c.cross_step(true), 0);
        assert_eq!(c.static_walk(), 0);
        assert_eq!(c.trot_run(), 0);
        assert_eq!(c.economic_gait(), 0);
        assert_eq!(c.switch_avoid_mode(), 0);
    }

    #[test]
    fn auto_recover() {
        let c = make_client();
        assert_eq!(c.auto_recover_set(true), 0);
        assert_eq!(c.auto_recover_get(), Ok(false));
    }
}
