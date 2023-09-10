use crate::space::Space;
use crate::vec2::Vec2;
use rand::Rng;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::Map;

pub(crate) struct CostMapData {
    cost_map: HashMap<Vec2, usize>,
    found_player: bool,
}

impl CostMapData {
    fn new(cost_map: HashMap<Vec2, usize>, found_player: bool) -> Self {
        Self {
            cost_map,
            found_player,
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Pathfinding {
    position: Vec2,
    heuristic: usize,
}

impl Pathfinding {
    fn new(position: Vec2, heuristic: usize) -> Self {
        Pathfinding {
            position,
            heuristic,
        }
    }

    pub(crate) fn wander(current_monster_position: Vec2, map: &Map) -> Vec2 {
        // define the directions (up, down, left, right)
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        loop {
            // generate a random number between 0 and 3 (inclusive)
            let random_index = rand::thread_rng().gen_range(0..5);

            // make it possible to not move
            if random_index == 0 {
                return current_monster_position
            }

            // otherwise, pick a random direction to wander
            let (dx, dy) = directions[random_index-1]; // Subtract 1 to match the array index
            let new_x = current_monster_position.x as i32 + dx;
            let new_y = current_monster_position.y as i32 + dy;
            let potential_position = Vec2::new(new_x as usize, new_y as usize);

            if map[potential_position.y][potential_position.x].is_traversable {
                return potential_position;
            }
        }
    }

    pub(crate) async fn find_shortest_path(
        map: &Map,
        monster_start_position: Vec2,
        player_start_position: Vec2,
        radius: usize,
    ) -> Vec2 {
        // we build the cost map based on the type of tiles we want to accept as traversable, toggle bool for monsters
        let mut cost_map_data = Pathfinding::build_cost_map(
            &map,
            monster_start_position,
            player_start_position,
            false,
            radius,
        )
        .await;
        if cost_map_data.found_player {
            // found player whilst not ignoring other monster's looking for the player
            return Pathfinding::reconstruct_path(
                monster_start_position,
                player_start_position,
                &cost_map_data.cost_map,
                map,
                false,
            )
            .await;
        }

        // if we're here, than the monster couldn't find a traversable path to the player
        // we should still move the monster as far towards the player as it can traverse
        let mut new_cost_map = Pathfinding::build_cost_map(
            &map,
            monster_start_position,
            player_start_position,
            true,
            radius,
        )
        .await;
        Pathfinding::reconstruct_path(
            monster_start_position,
            player_start_position,
            &new_cost_map.cost_map,
            map,
            true,
        )
        .await
    }

    async fn build_cost_map(
        map: &Map,
        monster_start_position: Vec2,
        player_start_position: Vec2,
        ignore_monsters: bool,
        radius: usize,
    ) -> CostMapData {
        // create a map to store the cost of reaching each position
        let mut cost_map = HashMap::new();

        // create the open set as a priority queue. You can use a BinaryHeap for this
        let mut open_set = BinaryHeap::<Pathfinding>::new();

        // create a closed set to keep track of visited nodes and initialise it with the monster's starting position
        let mut closed_set = HashSet::new();
        closed_set.insert(monster_start_position);

        // initialise the cost to reach the starting position as zero.
        cost_map.insert(monster_start_position, 0);

        // add the initial position (monster's starting position) to the open set
        open_set.push(Pathfinding::new(monster_start_position, 0));

        // loop until the open set is empty
        while let Some(current_monster) = open_set.pop() {
            // check if we've reached the player's position
            if current_monster.position == player_start_position {
                // reconstruct and return the cost map
                return CostMapData::new(cost_map, true);
            }

            // mark the current node/start node (start monster position) as visited
            closed_set.insert(current_monster.position);

            let mut neighbours = Vec::<Vec2>::new();

            // explore neighbours of the current node
            if ignore_monsters {
                neighbours = Pathfinding::get_traversable_neighbours(
                    current_monster.position,
                    map,
                    player_start_position,
                    monster_start_position,
                    ignore_monsters,
                )
                .await
            } else {
                neighbours = Pathfinding::get_traversable_neighbours(
                    current_monster.position,
                    map,
                    player_start_position,
                    monster_start_position,
                    ignore_monsters,
                )
                .await
            }

            for neighbour_position in neighbours {
                if !closed_set.contains(&neighbour_position) {
                    let tile = map[neighbour_position.y][neighbour_position.x];
                    let movement_cost = tile.travel_cost;
                    let tentative_cost = cost_map[&current_monster.position] + movement_cost;

                    // if this is the first time visiting the neighbour or the new cost is lower, update the cost map
                    if !cost_map.contains_key(&neighbour_position)
                        || tentative_cost <= cost_map[&neighbour_position]
                    {
                        // legit neighbour entry, with a lower value than previous so count towards the radius of the monster searching for the player
                        // calculate the distance between the current position and the player's position
                        let distance_to_player = Pathfinding::calculate_distance(current_monster.position, player_start_position);

                        // check if the distance exceeds the desired radius
                        if distance_to_player > radius {
                            break;
                        }

                        // calculate the heuristic
                        let heuristic_cost = Pathfinding::calculate_heuristic(
                            neighbour_position,
                            player_start_position,
                        )
                        .await;

                        let priority = tentative_cost + heuristic_cost;

                        // update the cost map
                        cost_map.insert(neighbour_position, tentative_cost);

                        // create the neighbour node and add it to the open set
                        let neighbour_node = Pathfinding::new(neighbour_position, priority);
                        open_set.push(neighbour_node);
                    }
                }
            }
        }

        return CostMapData::new(HashMap::<Vec2, usize>::new(), false);
    }

    fn calculate_distance(monster_position: Vec2, player_position: Vec2) -> usize {
        // calculate the Manhattan distance (L1 distance) between the two positions
        let dx = (player_position.x as isize - monster_position.x as isize).abs() as usize;
        let dy = (player_position.y as isize - monster_position.y as isize).abs() as usize;
        let manhattan_distance = dx + dy;

        return manhattan_distance;
    }

    async fn calculate_heuristic(node_position: Vec2, player_position: Vec2) -> usize {
        // calculate the Manhattan distance (L1 distance) between the two positions
        let dx = (player_position.x as isize - node_position.x as isize).abs() as usize;
        let dy = (player_position.y as isize - node_position.y as isize).abs() as usize;
        let manhattan_distance = dx + dy;
        manhattan_distance
    }

    async fn reconstruct_path(
        monster_position: Vec2,
        player_position: Vec2,
        cost_map: &HashMap<Vec2, usize>,
        map: &Map,
        ignore_monsters: bool,
    ) -> Vec2 {
        let mut current_position = player_position;
        let mut path = Vec::new();

        // start from the player's position and work backward
        while current_position != monster_position {
            // find neighboring positions
            let mut neighbours = Vec::<Vec2>::new();

            if !ignore_monsters {
                neighbours = Pathfinding::get_traversable_neighbours(
                    current_position,
                    map,
                    player_position,
                    monster_position,
                    ignore_monsters,
                )
                .await;
            } else {
                neighbours = Pathfinding::get_traversable_neighbours(
                    current_position,
                    map,
                    player_position,
                    monster_position,
                    ignore_monsters,
                )
                .await;
            }

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

            if path.contains(&current_position) {
                break;
            }

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

    async fn get_traversable_neighbours(
        current_node_position: Vec2,
        map: &Map,
        player_pos: Vec2,
        monster_pos: Vec2,
        ignore_monsters: bool,
    ) -> Vec<Vec2> {
        let mut neighbours = Vec::new();

        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = current_node_position.x as i32 + dx;
            let new_y = current_node_position.y as i32 + dy;

            if new_x >= 0 && new_x < map[0].len() as i32 && new_y >= 0 && new_y < map.len() as i32 {
                let tile = &map[new_y as usize][new_x as usize];
                let tile_pos = Vec2::new(new_x as usize, new_y as usize);

                if tile.is_traversable {
                    neighbours.push(tile_pos);
                }

                if ignore_monsters {
                    if tile.is_monster {
                        neighbours.push(tile_pos);
                    }
                }

                // for when the player has been found
                if tile_pos == player_pos {
                    neighbours.push(tile_pos);
                }

                // for when the monster has been found
                if tile_pos == monster_pos {
                    neighbours.push(tile_pos);
                }
            }
        }

        neighbours
    }
}