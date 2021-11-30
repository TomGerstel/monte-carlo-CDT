use slotmap::*;

#[derive(Debug)]
pub struct Universe {
    vertices: Vec<DefaultKey>,
    sm: SlotMap<DefaultKey, Vertex>,
}

#[derive(Debug, Default, Clone)]
struct Vertex {
    prev: Vec<DefaultKey>,
    next: Vec<DefaultKey>,
    left: Option<DefaultKey>,
    right: Option<DefaultKey>,
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

        Universe { vertices, sm }
    }

    pub fn vertex_count(&self) -> usize {
        self.sm.len()
    }

    pub fn mcmc_step(&mut self, move_ratio: f32) {
        let move_22 = fastrand::f32() < move_ratio;
        let inv_move = fastrand::bool();
        if move_22 {
            let key = self.key_22();
            if inv_move {
                self.move_22_a(key);
            } else {
                self.move_22_b(key);
            }
        } else if inv_move {
            let key = self.key_42();
            self.move_42(key);
        } else {
            let key = self.key_random();
            self.move_24(key);
        }
    }

    fn key_random(&self) -> DefaultKey {
        let index = fastrand::usize(..self.vertices.len());
        self.vertices[index]
    }

    fn key_22(&self) -> DefaultKey {
        loop {
            let key = self.key_random();
            if self.sm[key].next.len() >= 2 {
                break key;
            }
        }
    }

    fn key_42(&self) -> DefaultKey {
        loop {
            let key = self.key_random();
            if self.sm[key].next.len() == 1 && self.sm[key].prev.len() == 1 {
                break key;
            }
        }
    }
    pub fn move_22_a(&mut self, key: DefaultKey) {
        // labels are as in Fig 6 of https://arxiv.org/abs/1203.3591
        let k1 = self.sm[key].left.unwrap();
        let k2 = key;
        let k3 = *self.sm[key].next.first().unwrap();
        let k4 = *self.sm[key].next.get(1).unwrap();

        // break old link
        self.sm[k2].next.remove(0);
        self.sm[k3].prev.pop();

        // create new link
        self.sm[k1].next.push(k4);
        self.sm[k4].prev.insert(0, k1);
    }

    pub fn move_22_b(&mut self, key: DefaultKey) {
        // labels are as in Fig 6 of https://arxiv.org/abs/1203.3591
        let k1 = key;
        let k2 = self.sm[key].right.unwrap();
        let k3 = *self.sm[key].next.get(self.sm[key].next.len() - 2).unwrap();
        let k4 = *self.sm[key].next.last().unwrap();

        // break old link
        self.sm[k1].next.pop();
        self.sm[k4].prev.remove(0);

        // create new link
        self.sm[k2].next.insert(0, k3);
        self.sm[k3].prev.push(k2);
    }

    pub fn move_24(&mut self, key: DefaultKey) {
        // labels are as in Fig 6 of https://arxiv.org/abs/1203.3591
        let k1 = *self.sm[key].prev.last().unwrap();
        let k2 = *self.sm[key].next.last().unwrap();
        let k3 = key;
        let k4 = self.sm[key].right.unwrap();

        // create new vertex and get its key
        let k5 = self.sm.insert(Vertex {
            prev: vec![k1],
            next: vec![k2],
            left: Some(k3),
            right: Some(k4),
        });

        // find indices of k1, k2 where to insert k5
        let ind_k1 = self.sm[k1].next.iter().position(|&k| k == k3).unwrap();
        let ind_k2 = self.sm[k2].prev.iter().position(|&k| k == k3).unwrap();

        // link up the new vertex
        self.sm[k1].next.insert(ind_k1 + 1, k5);
        self.sm[k2].prev.insert(ind_k2 + 1, k5);
        self.sm[k3].right = Some(k5); // old link is automatically broken here
        self.sm[k4].left = Some(k5); // old link is automatically broken here

        // add vertex to list
        self.vertices.push(k5);
    }

    pub fn move_42(&mut self, key: DefaultKey) {
        // labels are as in Fig 6 of https://arxiv.org/abs/1203.3591
        let k1 = *self.sm[key].prev.first().unwrap();
        let k2 = *self.sm[key].next.first().unwrap();
        let k3 = self.sm[key].left.unwrap();
        let k4 = self.sm[key].right.unwrap();
        let k5 = key;

        // find indices of k1, k2 where k5 is connectec
        let ind_k1 = self.sm[k1].next.iter().position(|&k| k == k5).unwrap();
        let ind_k2 = self.sm[k2].prev.iter().position(|&k| k == k5).unwrap();

        // edit links
        self.sm[k1].next.remove(ind_k1 + 1);
        self.sm[k2].prev.remove(ind_k2 + 1);
        self.sm[k3].right = Some(k4);
        self.sm[k4].left = Some(k3);

        // remove vertex from list
        self.sm.remove(k5);
    }
}
