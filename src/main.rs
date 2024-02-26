#[derive(Debug)]
struct Node{
    coor_x: u32,
    coor_y: u32,
    value: u32,
}

#[derive(Debug)]
struct Universe {
    size_x: u32,
    size_y: u32,
}

fn main() {
    // let canvas_x = 1000;
    // let canvas_y = 1000;
    let test_var = create_node(40, 103, 0);
    println!("{:?}", test_var);
}

fn create_node(coor_x:u32, coor_y:u32, value:u32) -> Node {
    Node {
        coor_x: coor_x,
        coor_y: coor_y,
        value: value,
    }
}

fn create_universe(size_x:u32, size_y:u32) -> Universe{
    Universe {
        size_x: size_x,
        size_y: size_y,
    }
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
}
