extern crate sdl2;
// extern  crate gl;

// AI system +
    // Behaviour Tree (i.e. follow -> hide -> attack -> hide -> ...) +
        // Component system where we define behaviour and put into Behaviour Tree
#[allow(dead_code)]
pub enum BehaviourTreeNode<'a> {
    Action(Box<dyn Fn() -> BehaviourTreeResult + 'a>),
    Selector(Vec<BehaviourTreeNode<'a>>),
    Sequence(Vec<BehaviourTreeNode<'a>>),
}

#[allow(dead_code)]
pub enum BehaviourTreeResult {
    Success,
    Failure,
    Running,
}
#[allow(dead_code)]
impl<'a> BehaviourTreeNode<'a> {
    pub fn tick(&self) -> BehaviourTreeResult {
        match self {
            BehaviourTreeNode::Action(action) => action(),
            BehaviourTreeNode::Selector(nodes) => {
                for node in nodes {
                    match node.tick() {
                        BehaviourTreeResult::Success => return BehaviourTreeResult::Success,
                        BehaviourTreeResult::Running => return BehaviourTreeResult::Running,
                        BehaviourTreeResult::Failure => {}
                    }
                }
                BehaviourTreeResult::Failure
            }
            BehaviourTreeNode::Sequence(nodes) => {
                for node in nodes {
                    match node.tick() {
                        BehaviourTreeResult::Failure => return BehaviourTreeResult::Failure,
                        BehaviourTreeResult::Running => return BehaviourTreeResult::Running,
                        BehaviourTreeResult::Success => {}
                    }
                }
                BehaviourTreeResult::Success
            }
        }
    }
}