use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::Arc;
use futures::lock::Mutex;
use crate::chat::Chat;
use crate::space::Space;
use crate::tile_set::DEFAULT_TILE_SET;
use crate::vec2::Vec2;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Node {
    position: Vec2,
    heuristic: usize,
}

impl Node {
    fn new(position: Vec2, heuristic: usize) -> Self {
        Node {
            position,
            heuristic,
        }
    }

    pub(crate) async fn find_shortest_path(
        map: &Vec<Vec<Space>>,
        monster_start_position: Vec2,
        player_start_position: Vec2,
        mut chat_clone: &mut Arc<Mutex<Chat>>
    ) -> Vec2 {
        // create a map to store the cost of reaching each position
        let mut cost_map = HashMap::new();

        // create the open set as a priority queue. You can use a BinaryHeap for this
        let mut open_set = BinaryHeap::<Node>::new();

        // create a closed set to keep track of visited nodes and initialise it with the monster's starting position
        let mut closed_set = HashSet::new();
        closed_set.insert(monster_start_position);

        // initialise the cost to reach the starting position as zero.
        cost_map.insert(monster_start_position, 0);

        // add the initial position (monster's starting position) to the open set
        open_set.push(Node::new(monster_start_position, 0));

        // loop until the open set is empty
        while let Some(current_monster) = open_set.pop() {

            // check if we've reached the player's position
            if current_monster.position == player_start_position {
                // reconstruct and return the path
                chat_clone.lock().await.process_debug_message("current node pos eq pl", 4);
                return Node::reconstruct_path(monster_start_position, player_start_position, &cost_map, map, chat_clone).await;
            }

            // mark the current node/start node (start monster position) as visited
            closed_set.insert(current_monster.position);

            // explore neighbours of the current node
            for neighbour_position in Node::get_neighbours(chat_clone, current_monster.position, map, player_start_position, monster_start_position).await {

                if !closed_set.contains(&neighbour_position) {

                    let tile = map[neighbour_position.y][neighbour_position.x];
                    let movement_cost = tile.travel_cost;
                    let tentative_cost = cost_map[&current_monster.position] + movement_cost;

                    // if this is the first time visiting the neighbour or the new cost is lower, update the cost map
                    if !cost_map.contains_key(&neighbour_position) || tentative_cost <= cost_map[&neighbour_position] {

                        // Calculate the heuristic (you can use the Space values here).
                        let heuristic_cost = Node::calculate_heuristic(chat_clone, neighbour_position, player_start_position, tentative_cost).await;

                        let priority = tentative_cost + heuristic_cost;

                        // Update the cost map.
                        cost_map.insert(neighbour_position, tentative_cost);

                        // create the neighbour node and add it to the open set.
                        let neighbour_node = Node::new(neighbour_position, priority);
                        open_set.push(neighbour_node);
                    }
                }
            }
        }

        chat_clone.lock().await.process_debug_message(&format!("New monster position not found: {}", cost_map.len()), 2);

        // if no path is found, return the monster's current position.
        monster_start_position
    }

    async fn get_neighbours(mut chat_clone: &mut Arc<Mutex<Chat>>, current_node_position: Vec2, map: &Vec<Vec<Space>>, player_pos: Vec2, monster_pos: Vec2) -> Vec<Vec2> {
        let mut neighbours = Vec::new();

        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = current_node_position.x as i32 + dx;
            let new_y = current_node_position.y as i32 + dy;

            if new_x >= 0 && new_x < map[0].len() as i32 && new_y >= 0 && new_y < map.len() as i32 {
                let tile = &map[new_y as usize][new_x as usize];
                let tile_pos = Vec2::new(new_x as usize, new_y as usize);

                if tile.is_traversable  {
                    neighbours.push(tile_pos);
                }

                if tile_pos == player_pos {
                    neighbours.push(tile_pos);
                }

                if tile_pos == monster_pos {
                    neighbours.push(tile_pos);
                }
            }
        }

        neighbours
    }

    async fn calculate_heuristic(
        mut chat_clone: &mut Arc<Mutex<Chat>>,
        node_position: Vec2,
        player_position: Vec2,
        tentative_cost: usize
    ) -> usize {
        // calculate the Manhattan distance (L1 distance) between the two positions
        let dx = (player_position.x as isize - node_position.x as isize).abs() as usize;
        let dy = (player_position.y as isize - node_position.y as isize).abs() as usize;
        let manhattan_distance = dx+dy;

        chat_clone.lock().await.process_debug_message(&format!("plyr pos: {:?},\nnode pos: {:?}", player_position, node_position), 7);

        // return the heuristic cost as the Manhattan distance
        chat_clone.lock().await.process_debug_message(&format!("Manhattan d: {}, tentative_cost: {}", manhattan_distance, tentative_cost), 1);

        manhattan_distance
    }

    async fn reconstruct_path(
        monster_position: Vec2,
        player_position: Vec2,
        cost_map: &HashMap<Vec2, usize>,
        map: &Vec<Vec<Space>>,
        mut chat_clone: &mut Arc<Mutex<Chat>>
    ) -> Vec2 {
        let mut current_position = player_position;
        let mut path = Vec::new();

        // start from the player's position and work backward
        while current_position != monster_position {

            // find neighboring positions
            let neighbours = Node::get_neighbours(&mut chat_clone, current_position, map, player_position, monster_position).await;

            // initialise min_cost with a high value
            let mut min_cost = usize::MAX;
            let mut next_position = current_position;

            for neighbour in neighbours {
                if let Some(cost) = cost_map.get(&neighbour) {
                    if *cost < min_cost {
                        min_cost = *cost;
                        next_position = neighbour;
                    }
                }
            }

            current_position = next_position;
            path.push(current_position);
        }

        // remove initial player pos
        path.pop();

        // the path is currently in reverse order, so reverse it
        path.reverse();

        if let Some(pathing_data) = path.first() {
            *pathing_data
        } else {
            monster_position
        }
    }
}
