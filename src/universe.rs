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
    time: usize,
    space: usize,
}

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self.slices[pos.time][pos.space]
    }
}

impl IndexMut<VertexPos> for Universe {
    fn index_mut(&mut self, pos: VertexPos) -> &mut Self::Output {
        &mut self.slices[pos.time][pos.space]
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
            slices: vec![],
            order_four: vec![],
        };
        for t in 0..timespan {
            let pos_next = VertexPos::new((t + 1) % timespan, 0);
            let pos_prev = VertexPos::new((t + timespan - 1) % timespan, 0);
            let vertex = Vertex {
                neighbours_next: vec![pos_next, pos_next],
                neighbours_prev: vec![pos_prev, pos_prev],
            };
            let slice = vec![vertex];
            universe.slices.push(slice);
        }

        let n_free = vertex_count - timespan;
        let pos_next = VertexPos::new(1, 0);
        let pos_prev = VertexPos::new(timespan - 1, 0);
        for s in 0..n_free {
            let vertex = Vertex {
                neighbours_next: vec![pos_next],
                neighbours_prev: vec![pos_prev],
            };
            universe.slices[0].push(vertex);

            let pos = VertexPos::new(0, s + 1);
            universe.order_four.push(pos);
            universe[pos_next].neighbours_prev.push(pos);
            universe[pos_prev].neighbours_next.push(pos);
        }
        universe
    }

    pub fn vertex_count(&self) -> usize {
        self.slices.iter().map(|x| x.len()).sum()
    }

    pub fn timespan(&self) -> usize {
        self.slices.len()
    }
}

impl VertexPos {
    fn new(time: usize, space: usize) -> Self {
        VertexPos { time, space }
    }
}
