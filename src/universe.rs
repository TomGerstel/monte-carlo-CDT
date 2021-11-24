use std::ops::Index;

struct Universe {
    slices: Vec<Timeslice>,
    order_four: Vec<Vertex>,
}

struct Timeslice {
    vertices: Vec<Vertex>,
}

struct Vertex {
    neighbours_up: Vec<VertexPos>,
    neighbours_down: Vec<VertexPos>,
}

struct VertexPos {
    time: Time,
    space: Space,
}

struct Time(usize);
struct Space(usize);

impl Index<Time> for Universe {
    type Output = Timeslice;

    fn index(&self, time: Time) -> &Self::Output {
        &self.slices[time.0]
    }
}

impl Index<VertexPos> for Universe {
    type Output = Vertex;

    fn index(&self, pos: VertexPos) -> &Self::Output {
        &self[pos.time][pos.space]
    }
}

impl Index<Space> for Timeslice {
    type Output = Vertex;

    fn index(&self, space: Space) -> &Self::Output {
        &self.vertices[space.0]
    }
}
