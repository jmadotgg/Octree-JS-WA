use rand::Rng;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct OctreeNode {
    center: Point,
    size: f32,
    points: Vec<Point>,
    children: Option<[Box<OctreeNode>; 8]>,
}

impl OctreeNode {
    fn new(center: Point, size: f32) -> Self {
        OctreeNode {
            center,
            size,
            points: Vec::new(),
            children: None,
        }
    }

    fn insert(&mut self, point: Point) {
        if self.size <= 0.1 {
            self.points.push(point);
            return;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        let octant = self.get_octant(point);
        self.children.as_mut().unwrap()[octant].insert(point);
    }

    fn subdivide(&mut self) {
        let half_size = self.size / 2.0;
        let x = self.center.x;
        let y = self.center.y;
        let z = self.center.z;

        let mut children = [
            Box::new(OctreeNode::new(
                Point {
                    x: x - half_size,
                    y: y - half_size,
                    z: z - half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x - half_size,
                    y: y - half_size,
                    z: z + half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x - half_size,
                    y: y + half_size,
                    z: z - half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x - half_size,
                    y: y + half_size,
                    z: z + half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x + half_size,
                    y: y - half_size,
                    z: z - half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x + half_size,
                    y: y - half_size,
                    z: z + half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x + half_size,
                    y: y + half_size,
                    z: z - half_size,
                },
                half_size,
            )),
            Box::new(OctreeNode::new(
                Point {
                    x: x + half_size,
                    y: y + half_size,
                    z: z + half_size,
                },
                half_size,
            )),
        ];

        for point in &self.points {
            let octant = self.get_octant(*point);
            children[octant].insert(*point);
        }

        self.points.clear();
        self.children = Some(children);
    }

    fn get_octant(&self, point: Point) -> usize {
        let x = if point.x >= self.center.x { 1 } else { 0 };
        let y = if point.y >= self.center.y { 1 } else { 0 };
        let z = if point.z >= self.center.z { 1 } else { 0 };
        (x << 2) | (y << 1) | z
    }
    fn query_point(&self, point: Point) -> Option<Point> {
        if self.size <= 0.1 {
            // This is a leaf node, check if the point exists in its points
            for &p in &self.points {
                if p.x == point.x && p.y == point.y && p.z == point.z {
                    return Some(p);
                }
            }
        } else if let Some(children) = &self.children {
            // This node is not a leaf, so traverse the appropriate child node
            let octant = self.get_octant(point);
            return children[octant].query_point(point);
        }

        None
    }
}

#[wasm_bindgen]
pub fn run_wa(size: u32) -> Option<String> {
    let mut root = OctreeNode::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        10.0,
    );

    let mut rng = rand::thread_rng();
    for _ in 0..size {
        root.insert(Point {
            x: rng.gen_range(0..size) as f32,
            y: rng.gen_range(0..size) as f32,
            z: rng.gen_range(0..size) as f32,
        })
    }

    let point = Point {
        x: 0.444 * size as f32,
        y: 0.666 * size as f32,
        z: 0.888 * size as f32,
    };

    root.insert(point);
    let result = root.query_point(point);

    Some(format!("{result:?}"))

    // Now the octree is filled with example data. You can perform various operations on it as needed.
    // You may want to implement additional methods for querying, traversing, or testing its speed.
}
