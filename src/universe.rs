use slotmap::*;

#[derive(Debug)]
pub struct Universe {
    vertices: Vec<DefaultKey>,
    sm: SlotMap<DefaultKey, Vertex>,
    order_four: Vec<DefaultKey>,
}

#[derive(Debug, Default, Clone)]
struct Vertex {
    //position: VertexPos,
    prev: Vec<DefaultKey>,
    next: Vec<DefaultKey>,
    left: Option<DefaultKey>,
    right: Option<DefaultKey>,
    //neighbours_prev: Vec<VertexPos>,
}

impl Universe {
    pub fn new(timespan: usize) -> Self {
        assert!(timespan > 1);

        // create slotmap and empty vertex vector
        let mut sm = SlotMap::with_capacity(timespan);
        let vertices = (0..timespan)
            .map(|_| Vertex::default())
            .map(|v| sm.insert(v))
            .collect::<Vec<_>>();

        // assign neighbours to vertices
        vertices.windows(3).for_each(|window| {
            let prev = window[0];
            let curr = window[1];
            let next = window[2];

            sm[curr] = Vertex {
                prev: vec![prev; 2],
                next: vec![next; 2],
                left: Some(curr),
                right: Some(curr),
            };
        });

        // patch the ends together
        sm[*vertices.first().unwrap()] = Vertex {
            prev: vec![*vertices.last().unwrap(); 2],
            next: vec![*vertices.get(1).unwrap(); 2],
            left: Some(*vertices.first().unwrap()),
            right: Some(*vertices.first().unwrap()),
        };
        sm[*vertices.last().unwrap()] = Vertex {
            prev: vec![*vertices.get(timespan - 2).unwrap(); 2],
            next: vec![*vertices.first().unwrap(); 2],
            left: Some(*vertices.last().unwrap()),
            right: Some(*vertices.last().unwrap()),
        };

        Universe {
            vertices,
            sm,
            order_four: vec![],
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.sm.len()
    }

    pub fn random_order_four(&self) -> DefaultKey {
        let index = fastrand::usize(..self.order_four.len());
        self.order_four[index]
    }
}
