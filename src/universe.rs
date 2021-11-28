use fastrand::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Universe {
    vertices: Vec<Vertex>,
    lengths: Vec<usize>,
    order_four: Vec<VertexPos>,
}

#[derive(Debug)]
struct Vertex {
    position: VertexPos,
    neighbours_next: Vec<VertexPos>,
    neighbours_prev: Vec<VertexPos>,
}

#[derive(Copy, Clone, Debug)]
struct VertexPos {
    time: usize,
    space: usize,
}

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self.vertices[self.lengths.iter().take(pos.time).sum::<usize>() + pos.space]
    }
}

impl IndexMut<VertexPos> for Universe {
    fn index_mut(&mut self, pos: VertexPos) -> &mut Self::Output {
        &mut self.vertices[self.lengths.iter().take(pos.time).sum::<usize>() + pos.space]
    }
}

impl Universe {
    pub fn new(vertex_count: usize, timespan: usize) -> Self {
        assert!(
            vertex_count >= timespan,
            "given vertex count ({}) is too small, must be at least as big as the timespan ({})",
            vertex_count,
            timespan
        );

        let mut universe = Universe {
            vertices: vec![],
            lengths: vec![],
            order_four: vec![],
        };
        for t in 0..timespan {
            let pos_next = VertexPos::new((t + 1) % timespan, 0);
            let pos_prev = VertexPos::new((t + timespan - 1) % timespan, 0); // Ik weet vrij zeker dat je gewoon negatieve getallen kunt modulo'en
            let vertex = Vertex {
                position: VertexPos::new(t, 0),
                neighbours_next: vec![pos_next, pos_next],
                neighbours_prev: vec![pos_prev, pos_prev],
            };
            universe.vertices.push(vertex);
            universe.lengths.push(1);
        }

        let n_free = vertex_count - timespan;
        let pos_next = VertexPos::new(1, 0);
        let pos_prev = VertexPos::new(timespan - 1, 0);
        for s in 0..n_free {
            let pos = VertexPos::new(0, s + 1);
            let vertex = Vertex {
                position: pos,
                neighbours_next: vec![pos_next],
                neighbours_prev: vec![pos_prev],
            };
            universe.vertices.insert(1, vertex);
            universe.lengths[0] += 1;
            universe.order_four.push(pos);
            universe[pos_next].neighbours_prev.push(pos);
            universe[pos_prev].neighbours_next.push(pos);
        }
        universe
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn timespan(&self) -> usize {
        self.lengths.len()
    }

    fn random_vertex(&self) -> VertexPos {
        let index = fastrand::usize(..self.vertex_count());
        self.vertices[index].position
    }

    pub fn move_22(&mut self) {
        let pos = self.random_vertex();
        let len = self.lengths[pos.time];
        if fastrand::bool() {
            //First identify the four relevant vertices
            //In this case pos = pos_right
            let pos_left = VertexPos::new(pos.time, (pos.space + len - 1) % len);
            let pos_next_left = self[pos_left].neighbours_next[0];
            let pos_next_right = self[pos_left].neighbours_next[0];

            //Break the link
            self[pos].neighbours_next.remove(0);
            self[pos_next_left].neighbours_prev.pop();

            //Restore the link
            self[pos_left].neighbours_next.push(pos_next_right);
            self[pos_next_right].neighbours_prev.insert(0, pos)
        } else {
            //TODO: do the inverse
        }
        //TODO: update order_four
    }
}

impl VertexPos {
    fn new(time: usize, space: usize) -> Self {
        VertexPos { time, space }
    }
}
