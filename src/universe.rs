use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Universe {
    slices: Vec<Vec<Vertex>>,
    order_four: Vec<VertexPos>,
}

#[derive(Debug)]
struct Vertex {
    neighbours_next: Vec<VertexPos>,
    neighbours_prev: Vec<VertexPos>,
}

#[derive(Copy, Clone, Debug)]
struct VertexPos {
    time: Time,
    space: Space,
}

#[derive(Copy, Clone, Debug)]
struct Time(usize);

#[derive(Copy, Clone, Debug)]
struct Space(usize);

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self.slices[pos.time.0][pos.space.0]
    }
}

impl IndexMut<VertexPos> for Universe {
    fn index_mut(&mut self, pos: VertexPos) -> &mut Self::Output {
        &mut self.slices[pos.time.0][pos.space.0]
    }
}

impl Universe {
    pub fn new(n_vertex: usize, max_time: usize) -> Self {
        assert!(
            n_vertex >= max_time,
            "given vertex number ({}) is too small, must be at least as big as the max time ({})",
            n_vertex,
            max_time
        );

        let mut universe = Universe {
            slices: vec![],
            order_four: vec![],
        };
        for t in 0..max_time {
            let pos_next = VertexPos {
                time: Time((t + 1) % max_time),
                space: Space(0),
            };
            let pos_prev = VertexPos {
                time: Time((t + max_time - 1) % max_time),
                space: Space(0),
            };
            let vertex = Vertex {
                neighbours_next: vec![pos_next, pos_next],
                neighbours_prev: vec![pos_prev, pos_prev],
            };
            let slice = vec![vertex];
            universe.slices.push(slice);
        }

        let n_free = n_vertex - max_time;
        let pos_next = VertexPos {
            time: Time(1),
            space: Space(0),
        };
        let pos_prev = VertexPos {
            time: Time(max_time - 1),
            space: Space(0),
        };
        for s in 0..n_free {
            let vertex = Vertex {
                neighbours_next: vec![pos_next],
                neighbours_prev: vec![pos_prev],
            };
            universe.slices[0].push(vertex);

            let pos = VertexPos {
                time: Time(0),
                space: Space(s + 1),
            };
            universe.order_four.push(pos);
            universe[pos_next].neighbours_prev.push(pos);
            universe[pos_prev].neighbours_next.push(pos);
        }
        universe
    }
}
