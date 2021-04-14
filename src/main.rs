// initialization
    // import dependancies
        use pathfinding::prelude::{astar}; // absdiff
        use std::fs::File;
        use std::io::prelude::*;
        use std::time::{SystemTime};
        use std::sync;
        use std::sync::mpsc;
        use threadpool::ThreadPool;
        extern crate num_cpus;
    // define constants
        // define player movement speeds
            // const WALKING_SPEED: f64 = 4.317;
            const SPRINT_SPEED: f64 = 5.612;
            // const SPRINT_JUMPING_SPEED: f64 = 7.0;
            const SPRINT_JUMPING_ROOFLESS_ICE_SPEED: f64 = 9.23;
            const SPRINT_JUMPING_ICE_SPEED: f64 = 16.9;

// function declarations

    fn distance_between_points(x1:i64, y1:i64, z1:i64, x2:i64, y2:i64, z2:i64) -> f64{
        (((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64).powf(1.0/2.0)
    }

    fn pathtype_travel_speed(pathtype: &str) -> Option<f64>{
        match pathtype {
            "normal" => Some(SPRINT_SPEED),
            "rail" => Some(SPRINT_SPEED),
            "iceroad" => Some(SPRINT_JUMPING_ICE_SPEED),
            "roofless iceroad" => Some(SPRINT_JUMPING_ROOFLESS_ICE_SPEED),
            _ => None
        }
    }

fn main() {

    // retrieve node-data json from file
        let mut file = File::open("Public-CivClassic-Nodes/nodes.json").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read to string");
        let node_data: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    // retrieve destinations from json
        let mut file = File::open("Public-CivClassic-Nodes/destinations.json").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read to string");
        let destinations: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    // multi-threaded pathfind to all destinations
        let pool = ThreadPool::new(num_cpus::get());
        let arc_node_data = sync::Arc::new(node_data);


        let (results_tx, results_rx) = mpsc::channel();
        let start_time = SystemTime::now();

        // pathfind to every destination
        for (node_id, dest_name) in destinations.as_object().unwrap(){

            let node_id = node_id.clone();
            let dest_name = dest_name.clone();

            let arc_node_data2 = arc_node_data.clone();
            let results_tx1 = results_tx.clone();
            pool.execute(move || {

                // declare closures

                    let get_neighbors = |node: &str| -> Vec<&String>{
                        let mut neighbors = Vec::new();

                        // loop through all keys in "connections"
                        for (neighbor, _val) in arc_node_data2[node]["connections"].as_object().unwrap(){
                            neighbors.push(neighbor)
                        }

                        neighbors
                    };

                    let pathtype_for_types = |type1: &str,type2: &str| -> String{
                        if type1 == type2 {
                            type1.to_string()
                        }else{
                            "normal".to_string()
                        }
                    };

                    let pathtype_between_nodes = |node1: &str, node2: &str| -> String{
                        pathtype_for_types(&arc_node_data2[node1]["pathType"].as_str().unwrap(), &arc_node_data2[node2]["pathType"].as_str().unwrap())
                    };

                    let distance_between_nodes = |node1: &str, node2: &str| -> f64{
                        distance_between_points(arc_node_data2[node1]["x"].as_i64().unwrap(),arc_node_data2[node1]["y"].as_i64().unwrap(),arc_node_data2[node1]["z"].as_i64().unwrap(), arc_node_data2[node2]["x"].as_i64().unwrap(),arc_node_data2[node2]["y"].as_i64().unwrap(),arc_node_data2[node2]["z"].as_i64().unwrap())
                    };

                    let cost_of_travel_between_nodes = |node1: &str, node2: &str| -> u64{
                        ((distance_between_nodes(node1, node2) / (pathtype_travel_speed(&pathtype_between_nodes(&node1, &node2)).unwrap())) * 1000.0) as u64
                    };

                    let get_successors = |node: &str| -> Vec<(&str, u64)>{
                        let neighbors = get_neighbors(node);
                        let mut successors = Vec::new();
                        for neighbor in neighbors{
                            successors.push((&neighbor[..], cost_of_travel_between_nodes(node, neighbor) as u64));
                        }
                        successors
                    };

                // declare destination nodeIDs
                    // starting point
                        let gensokyo_yuyu_hut = "0x0E4B93C652D6C3DF1DF377D1DFA1B33C";
                        // let bibelbonse = "0x410048838B8718F06D73AFCE00B9A8B1";
                    // possible destinations
                        // let gensokyo_cactus_farm = "0x00F936B150C8D7B0363B449FA45710F1";
                        // let pandoria_station_underground = "0x4C4D4CB26FEBE14BAC7A6E4887BB7375";
                        // let pandoria_station_surface = "0x101F51B31E1276E45FF42E0660198954";

                // pathfind
                    let starting_node = gensokyo_yuyu_hut;
                    let target = &node_id;

                    let h = |node: &str| -> u64{
                        let distance = distance_between_nodes(&node, &target);
                        (distance / SPRINT_JUMPING_ICE_SPEED * 1000.0) as u64
                    };

                    let result = astar(
                        // starting node
                            &starting_node, 
                        // get_successors()
                            |node|get_successors(node),
                        // h()
                            |node|h(node),
                        // success()
                            |node| node == target
                    );

                    match result {
                        Some(x) => {
                            let mut string_vec = Vec::new();
                            for item in x.0{
                                string_vec.push(String::from(item));
                            }
                            results_tx1.send((dest_name,Some((string_vec,x.1)))).unwrap()
                        },
                        None => results_tx1.send((dest_name,None)).unwrap(),
                    };
            });
        }
        drop(results_tx);
        
        for received in results_rx{
            println!("Destination: {}",received.0);
            match received.1 {
                Some(x) => {
                    println!("ETA: {} minutes",x.1 / 1000 / 60);
                    // println!("Route: {:?}",x.0);
                },
                None => println!("ETA: [no path found]"),
            };
        }

        let end_time = SystemTime::now();
        let duration = end_time
        .duration_since(start_time)
        .expect("Time went backwards");

        println!("=====");
        println!("Completed pathfinding to {} destinations in {:?} seconds",destinations.as_object().unwrap().len(), duration);
}