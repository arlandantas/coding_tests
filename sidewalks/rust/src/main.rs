/*

7
1 4 2
2 4 3
3 4 1
4 5 1
5 6 2
5 7 4

4
1 2 1
2 3 2
3 4 3

*/

use std::{io, collections::HashMap};

#[derive(Debug)]
struct Route {
    origin: u32,
    current: u32,
    cost: u32,
    path: Vec<u32>,
}

fn get_best_route(origin: &u32, target: &u32, base_data: &BaseData) -> Route {
    let mut possibilities: Vec<Route> = Vec::new();
    let initial_route = Route { origin: origin.clone(), current: origin.clone(), cost: 0, path: vec![origin.clone()] };
    if origin == target {
        return initial_route;
    }

    let mut best_possibility: Option<Route> = None;

    possibilities.push(initial_route);
    while possibilities.len() > 0 {
        let possibility = possibilities.remove(0);
        if possibility.current == *target {
            if best_possibility.is_none() || possibility.cost < best_possibility.as_ref().unwrap().cost {
                best_possibility = Some(possibility);
                continue;
            }
        }
        let connections = base_data.building_connections.get(&possibility.current).unwrap();
        for (connection, cost) in connections {
            if possibility.path.contains(connection) {
                continue;
            }
            if connection != target && base_data.leaf_buildings.contains(connection) {
                continue;
            }
            let mut path = possibility.path.clone();
            path.push(connection.clone());
            let cost = possibility.cost + cost;
            let new_possibility = Route { origin: possibility.origin, current: *connection, cost, path };
            possibilities.push(new_possibility);
        }
    }

    return best_possibility.expect("Failed to find best route");
}

fn count_movements(base_data: &BaseData) {
    let leaf_paths: HashMap<u32, HashMap<u32, Route>> = HashMap::new();
    for leaf in &base_data.leaf_buildings {
        for leaf2 in &base_data.leaf_buildings {
            if leaf == leaf2 { continue; }
            let route = get_best_route(leaf, leaf2, base_data);
            print!("Leafs: {} - {}: ", leaf, leaf2);
            print_full_path(&route.path);
        }
    }
}

fn main() {
    let base_data = read_static_input();

    count_movements(&base_data);
    
    // println!("Input: {:#?}", base_data);
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn read_usize() -> u32 {
    return read_string().parse().expect("Failed to parse input to number");
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Path {
    origin: u32,
    target: u32,
    cost: u32,
}

type BuildingConnections = HashMap<u32, HashMap<u32, u32>>;

#[derive(Debug)]
struct BaseData {
    building_count: u32,
    leaf_buildings: Vec<u32>,
    building_connections: BuildingConnections
}

fn read_static_input() -> BaseData {
    // 1 2 1
    // 2 3 2
    // 3 4 3
    // let paths = vec![
    //     Path { origin: 1, target: 2, cost: 1 },
    //     Path { origin: 2, target: 3, cost: 5 },
    //     Path { origin: 3, target: 4, cost: 3 },
    //     Path { origin: 2, target: 5, cost: 1 },
    //     Path { origin: 5, target: 3, cost: 1 },
    // ];

    let paths = vec![
        Path { origin: 1, target: 4, cost: 2 },
        Path { origin: 2, target: 4, cost: 3 },
        Path { origin: 3, target: 4, cost: 1 },
        Path { origin: 4, target: 5, cost: 1 },
        Path { origin: 5, target: 6, cost: 2 },
        Path { origin: 5, target: 7, cost: 4 }
    ];
    let mut building_connections: BuildingConnections = HashMap::new();
    let mut building_count = 0;
    for path in &paths {
        add_connection(&path, &mut building_connections);
        if path.origin > building_count {
            building_count = path.origin;
        }
        if path.target > building_count {
            building_count = path.target;
        }
    }
    return BaseData {
        building_count,
        leaf_buildings: get_leaf_buildings(&building_connections),
        building_connections,
    }
}

fn get_leaf_buildings(building_connections: &BuildingConnections) -> Vec<u32> {
    let mut leaf_buildings = vec![];
    for (building, connections) in building_connections {
        if connections.len() == 1 {
            leaf_buildings.push(building.clone());
        }
    }
    return leaf_buildings;
}

fn add_connection(path: &Path, building_connections: &mut BuildingConnections) {
    if !building_connections.contains_key(&path.origin) {
        building_connections.insert(path.origin, HashMap::new());
    }
    let origin_connections = building_connections.get_mut(&path.origin).unwrap();
    if !origin_connections.contains_key(&path.target) {
        origin_connections.insert(path.target, path.cost);
    }

    if !building_connections.contains_key(&path.target) {
        building_connections.insert(path.target, HashMap::new());
    }
    let target_connections = building_connections.get_mut(&path.target).unwrap();
    if !target_connections.contains_key(&path.origin) {
        target_connections.insert(path.origin, path.cost);
    }
}

fn read_input() -> BaseData {
    let building_count = read_usize();
    let mut building_connections: BuildingConnections = HashMap::new();

    for _ in 0..building_count-1 {
        let line = read_string();
        let mut line_split = line.split(" ");
        let origin = line_split.next().unwrap().parse::<u32>().unwrap();
        let target = line_split.next().unwrap().parse::<u32>().unwrap();
        let cost = line_split.next().unwrap().parse::<u32>().unwrap();
        let path = Path { origin, target, cost };
        add_connection(&path, &mut building_connections);
    }

    let leaf_buildings = get_leaf_buildings(&building_connections);

    return BaseData { building_count, building_connections, leaf_buildings };
}

fn print_full_path(path: &Vec<u32>) {
    if path.len() == 0 {
        return;
    }
    let mut iterator = path.iter();
    print!("Full path: {}", iterator.next().unwrap());
    for current in iterator {
        print!(" -> {}", current);
    }
    println!("");
}
