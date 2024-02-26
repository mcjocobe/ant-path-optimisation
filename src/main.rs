#[derive(Debug)]
struct Node {
    coor_x: u32,
    coor_y: u32,
    value: u32,
}

fn main() {
    // let canvas_x = 1000;
    // let canvas_y = 1000;
    let test_var = create_random_node();
    println!("{:?}", test_var);
}

fn create_random_node() -> Node {
    Node {
        coor_x: 40,
        coor_y: 103,
        value: 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn point_exist() {
        let test_node = crate::create_random_node();
        assert_eq!(test_node.coor_x, 40);
    }
}
