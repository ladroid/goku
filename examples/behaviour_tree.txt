// enum Status {
//     Success,
//     Failure,
//     Running,
// }

// struct ActionNode {
//     action: Box<dyn FnMut() -> Status>,
// }

// impl ActionNode {
//     fn new<F: 'static>(action: F) -> Self
//     where
//         F: FnMut() -> Status,
//     {
//         Self {
//             action: Box::new(action),
//         }
//     }

//     fn run(&mut self) -> Status {
//         (self.action)()
//     }
// }

// struct ConditionalNode {
//     condition: Box<dyn FnMut() -> bool>,
//     success_node: Box<dyn Node>,
//     failure_node: Box<dyn Node>,
// }

// impl ConditionalNode {
//     fn new<F: 'static>(condition: F, success_node: Box<dyn Node>, failure_node: Box<dyn Node>) -> Self
//     where
//         F: FnMut() -> bool,
//     {
//         Self {
//             condition: Box::new(condition),
//             success_node,
//             failure_node,
//         }
//     }
// }

// trait Node {
//     fn run(&mut self) -> Status;
// }

// impl Node for ActionNode {
//     fn run(&mut self) -> Status {
//         self.run()
//     }
// }

// impl Node for ConditionalNode {
//     fn run(&mut self) -> Status {
//         if (self.condition)() {
//             self.success_node.run()
//         } else {
//             self.failure_node.run()
//         }
//     }
// }

// struct EnemyBehaviorTree {
//     root_node: Box<dyn Node>,
// }

// impl EnemyBehaviorTree {
//     fn new(root_node: Box<dyn Node>) -> Self {
//         Self { root_node }
//     }

//     fn run(&mut self) {
//         self.root_node.run();
//     }
// }

// fn patrol(enemy: &mut GameObject, patrol_area: &Rect) -> Status {
//     let mut status = Status::Running;

//     // Move towards the right until we reach the right end of the patrol area
//     if enemy.x < patrol_area.x() + patrol_area.width() as i32 {
//         enemy.x += 1;
//     } else {
//         status = Status::Success;
//     }

//     status
// }

// fn chase(enemy: &mut GameObject, target: &Rect) -> Status {
//     let mut status = Status::Running;

//     // Move towards the target until we reach it
//     if enemy.collider.x() < target.x() {
//         enemy.x += 1;
//     } else if enemy.collider.x() > target.x() {
//         enemy.x -= 1;
//     } else {
//         status = Status::Success;
//     }

//     status
// }