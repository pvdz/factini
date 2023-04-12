#[derive(Debug, Clone)]
pub struct Story {
  // Maps to config.nodes
  pub story_node_index: usize,
  // Map to config.nodes, Parts specific to this Story.
  pub part_nodes: Vec<usize>,
  // Map to config.nodes, quests specific to this Story.
  pub quest_nodes: Vec<usize>,
}
