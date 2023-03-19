pub mod fire;

// the pub use in fire made it appear as if `shoot_lightning` was 
// defined in that module instead of `lightning_actions`
use fire::lightning_actions;

pub fn perform_action() {
    lightning_actions::shoot_lightning();
}
