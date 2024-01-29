pub struct TargetState;

pub fn determine_target_state() {}

// enum AnimationState {
//     Idle,
//     Running,
//     Airborne,
// }

pub struct AnimationStateNode {
    file: String,
    repeat: bool,
}

pub struct AnimationStateTransition {}

pub struct AnimationStateGraph {
    nodes: Vec<AnimationStateNode>,
    transitions: Vec<AnimationStateTransition>,
}
