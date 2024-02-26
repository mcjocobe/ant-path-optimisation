#[derive(Debug)]
struct Node {
    coor_x: i32,
    coor_y: i32,
    value: i32,
}

#[derive(Debug)]
struct Universe {
    size_x: i32,
    size_y: i32,
}

fn main() {
    // let canvas_x = 1000;
    // let canvas_y = 1000;
    let test_var = create_node(40, 103, 0);
    println!("{:?}", test_var);
}

fn create_node(coor_x: i32, coor_y: i32, value: i32) -> Node {
    Node {
        coor_x: coor_x,
        coor_y: coor_y,
        value: value,
    }
}

fn create_universe(size_x: i32, size_y: i32) -> Universe {
    Universe {
        size_x: size_x,
        size_y: size_y,
    }
}

fn is_node_in_universe(node: Node, universe: Universe) -> bool {
    if node.coor_x <= universe.size_x && node.coor_y <= universe.size_y {
        true
    } else {
        false
    }
}

fn distance_between_nodes(node_one: Node, node_two: Node) -> f32 {
    let result =
        (node_one.coor_x - node_two.coor_x).pow(2) + (node_one.coor_y - node_two.coor_y).pow(2);
    let f_result = result as f32;
    f_result.sqrt()
}

#[cfg(test)]
mod tests {
    #[test]
    fn universe_exists() {
        let test_universe = crate::create_universe(100, 200);
        assert_eq!(test_universe.size_x, 100);
    }

    #[test]
    fn point_exists() {
        let test_node = crate::create_node(40, 103, 0);
        assert_eq!(test_node.coor_x, 40);
    }

    #[test]
    fn is_point_in_universe() {
        let test_universe = crate::create_universe(100, 200);
        let test_node = crate::create_node(40, 103, 0);
        assert!(crate::is_node_in_universe(test_node, test_universe))
    }

    #[test]
    fn is_point_outside_universe() {
        let test_universe = crate::create_universe(100, 200);
        let test_node = crate::create_node(40, 203, 0);
        assert_eq!(crate::is_node_in_universe(test_node, test_universe), false)
    }

    #[test]
    fn compute_distance_between_nodes() {
        let test_node_one = crate::create_node(40, 130, 0);
        let test_node_two = crate::create_node(40, 150, 0);
        assert_eq!(
            crate::distance_between_nodes(test_node_one, test_node_two),
            20.0
        )
    }
}
