use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Universe {
    slices: Vec<TimeSlice>,
    order_four: Vec<VertexPos>,
}

#[derive(Debug)]
struct TimeSlice {
    vertices: Vec<Vertex>,
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

impl Index<Time> for Universe {
    type Output = TimeSlice;

    fn index(&self, time: Time) -> &Self::Output {
        &self.slices[time.0]
    }
}

impl IndexMut<Time> for Universe {
    fn index_mut(&mut self, time: Time) -> &mut Self::Output {
        &mut self.slices[time.0]
    }
}

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self[pos.time][pos.space]
    }
}

impl IndexMut<VertexPos> for Universe {
    fn index_mut(&mut self, pos: VertexPos) -> &mut Self::Output {
        &mut self[pos.time][pos.space]
    }
}

impl Index<Space> for TimeSlice {
    type Output = Vertex;

    fn index(&self, space: Space) -> &Self::Output {
        &self.vertices[space.0]
    }
}

impl IndexMut<Space> for TimeSlice {
    fn index_mut(&mut self, space: Space) -> &mut Self::Output {
        &mut self.vertices[space.0]
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
            let slice = TimeSlice {
                vertices: vec![vertex],
            };
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
            universe[Time(0)].vertices.push(vertex);

            let pos = VertexPos {
                time: Time(0),
                space: Space(s + 1),
            };
            universe.order_four.push(pos);
            universe[Time(1)][Space(0)].neighbours_prev.push(pos);
            universe[Time(max_time - 1)][Space(0)]
                .neighbours_next
                .push(pos);
        }
        universe
    }
}
