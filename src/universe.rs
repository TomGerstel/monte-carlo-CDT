#[derive(Clone, Debug)]
pub struct Universe {
    triangles: Vec<Triangle>,
    order_four: Vec<usize>, // keeps a list of order 4 vertices, labelled by the top-left triangle
}

#[derive(Clone, Debug)]
struct Triangle {
    orientation: Orientation,
    time: usize,
    left: usize,
    right: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Orientation {
    Up,
    Down,
}

impl Universe {
    pub fn new(timespan: usize, triangle_count: usize) -> Self {
        assert!(
            triangle_count % (2 * timespan) == 0,
            "triangle count must be an integer multiple of 2 times the timespan"
        );

        let length = triangle_count / timespan;
        let mut triangles = Vec::with_capacity(triangle_count);
        for t in 0..timespan {
            for i in 0..length {
                let (orientation, time) = match i % 2 {
                    0 => (
                        Orientation::Up,
                        (((t + timespan - 1) % timespan) * length + i + 1),
                    ),
                    1 => (
                        Orientation::Down,
                        (((t + timespan + 1) % timespan) * length + i - 1),
                    ),
                    _ => panic!(
                        "input was likely invalid: timespan = {}, triangle_count = {}",
                        timespan, triangle_count
                    ),
                };
                let left = t * length + ((i + length - 1) % length);
                let right = t * length + ((i + length + 1) % length);
                triangles.push(Triangle {
                    orientation,
                    time,
                    left,
                    right,
                })
            }
        }
        let order_four = vec![];
        Universe {
            triangles,
            order_four,
        }
    }

    pub fn lengths(&self, origin: usize) -> Vec<usize> {
        // Look at the lengths of the timeslices starting from an origin
        // Do this by 'walking through each slice, and thereafter advancing
        // to the next slice until back to starting point.

        let triangle_count = self.triangles.len();
        let mut lengths = Vec::with_capacity(triangle_count);

        let mut marker = origin;
        loop {
            lengths.push(1);
            let t = lengths.len() - 1;

            // find a down triangle and mark it as origin of the slice
            let mut slice_origin = marker;
            slice_origin = loop {
                match self.triangles[slice_origin].orientation {
                    Orientation::Down => break slice_origin,
                    Orientation::Up => slice_origin = self.triangles[slice_origin].right,
                }
            };

            // walk through the slice and count the triangles
            // break loop if slice_origin is found
            // return if origin is found
            let mut slice_walker = self.triangles[slice_origin].right;
            'slice_walk: loop {
                if slice_walker == origin && t > 0 {
                    lengths.pop(); // a slice was counted double, remove it
                    return lengths;
                } else if slice_walker == slice_origin {
                    break 'slice_walk;
                } else {
                    slice_walker = self.triangles[slice_walker].right;
                    lengths[t] += 1;
                }
            }

            // move to the next slice
            marker = self.triangles[slice_origin].time;
        }
    }

    pub fn mcmc_step(&mut self, move_ratio: f32) {
        let is_flip = fastrand::f32() < move_ratio;
        if is_flip || self.order_four.is_empty() {
            let left = self.sample_triangle_flip();
            self.triangle_flip(left);
        } else {
            let shard_up = self.sample_shard_move();
            let dest_up = self.sample_dest(shard_up);
            self.shard_move(shard_up, dest_up);
        }
    }

    fn sample_dest(&self, shard: usize) -> usize {
        loop {
            let index = fastrand::usize(..self.triangles.len());
            let dest = match self.triangles[index].orientation {
                Orientation::Up => index,
                Orientation::Down => self.triangles[index].time,
            };
            if dest != shard {
                return dest;
            }
        }
    }

    fn sample_triangle_flip(&self) -> usize {
        loop {
            let attempt = fastrand::usize(..self.triangles.len());
            let right = self.triangles[attempt].right;
            match (
                self.triangles[attempt].orientation,
                self.triangles[right].orientation,
            ) {
                (Orientation::Up, Orientation::Down) => break attempt,
                (Orientation::Down, Orientation::Up) => break attempt,
                _ => {}
            }
        }
    }

    fn sample_shard_move(&self) -> usize {
        let index = fastrand::usize(..self.order_four.len());
        self.triangles[self.order_four[index]].right
    }

    fn triangle_flip(&mut self, left: usize) {
        // identify the relevant triangles
        let right = self.triangles[left].right;
        let left_nbr = self.triangles[left].time;
        let right_nbr = self.triangles[right].time;

        // flip the orientations
        self.flip_orientation(left);
        self.flip_orientation(right);

        // reassign neighbours
        self.triangles[left_nbr].time = right;
        self.triangles[right_nbr].time = left;
        self.triangles[left].time = right_nbr;
        self.triangles[right].time = left_nbr;

        // update order_four
        // this can be optimised, some of these only have the possibility of
        // either becoming an order 4, or no longer being one (and not both)
        self.filter_order_four_at(right);
        self.filter_order_four_at(left_nbr);
        self.filter_order_four_at(right_nbr);
        self.filter_order_four_at(self.triangles[left].left);
        self.filter_order_four_at(self.triangles[left_nbr].left);
        self.filter_order_four_at(self.triangles[right_nbr].left);
    }

    fn shard_move(&mut self, shard_up: usize, dest_up: usize) {
        // identify the relevant triangles
        let shard_down = self.triangles[shard_up].time;
        let dest_down = self.triangles[dest_up].time;

        // identify the shard's neighbours
        let shard_nbr_left_up = self.triangles[shard_up].left;
        let shard_nbr_right_up = self.triangles[shard_up].right;
        let shard_nbr_left_down = self.triangles[shard_down].left;
        let shard_nbr_right_down = self.triangles[shard_down].right;

        // reassign neighbours around original location (close the gap left behind by the shard)
        self.triangles[shard_nbr_left_up].right = shard_nbr_right_up;
        self.triangles[shard_nbr_right_up].left = shard_nbr_left_up;
        self.triangles[shard_nbr_left_down].right = shard_nbr_right_down;
        self.triangles[shard_nbr_right_down].left = shard_nbr_left_down;

        // identify the neighbours near the destination
        let dest_nbr_up = self.triangles[dest_up].right;
        let dest_nbr_down = self.triangles[dest_down].right;

        // reassign neighbours around new shard location (insert the shard in its new location)
        self.triangles[dest_up].right = shard_up;
        self.triangles[dest_nbr_up].left = shard_up;
        self.triangles[dest_down].right = shard_down;
        self.triangles[dest_nbr_down].left = shard_down;

        // update the neighbours of the shard itself
        self.triangles[shard_up].right = dest_nbr_up;
        self.triangles[shard_up].left = dest_up;
        self.triangles[shard_down].right = dest_nbr_down;
        self.triangles[shard_down].left = dest_down;

        // update order_four
        // this can be optimised, some of these only have the possibility of
        // either becoming an order 4, or no longer being one (and not both)
        self.filter_order_four_at(shard_nbr_left_up);
        self.filter_order_four_at(shard_nbr_right_up);
        self.filter_order_four_at(dest_up);
        self.filter_order_four_at(shard_up);
        self.filter_order_four_at(dest_nbr_up);
    }

    fn flip_orientation(&mut self, label: usize) {
        self.triangles[label].orientation = match self.triangles[label].orientation {
            Orientation::Down => Orientation::Up,
            Orientation::Up => Orientation::Down,
        }
    }

    fn is_order_four_at(&self, label: usize) -> bool {
        self.triangles[label].orientation == Orientation::Up
            && self.triangles[self.triangles[label].right].orientation == Orientation::Up
            && self.triangles[self.triangles[label].time].right
                == self.triangles[self.triangles[label].right].time
    }

    fn filter_order_four_at(&mut self, label: usize) {
        self.order_four.retain(|&l| l != label);
        if self.is_order_four_at(label) {
            self.order_four.push(label);
        }
    }
}
/*
#[test]
fn test_universe_lengths() {
    let T = 7;
    let N = 72;

    let u0 = Universe::new(T, N);
    assert_eq!(u0.lengths(), vec![N / T; T]);
}

#[test]
fn test_moves01() {
    let mut universe = Universe::new(10, 100);

    // First check the flip move
    assert_eq!(universe.triangles[10].time, 20);
    universe.do_move(10, 1);
    assert_eq!(universe.triangles[10].time, 1);
    assert_eq!(universe.triangles[1].time, 10);
    assert_eq!(universe.triangles[11].time, 20);
    assert_eq!(universe.triangles[20].time, 11);

    // Then check the shard-move move
    universe.do_move(11, 1);
    assert_eq!(universe.triangles[11].time, 20);
    assert_eq!(universe.triangles[11].left, 1);
    assert_eq!(universe.triangles[11].right, 2);
    assert_eq!(universe.triangles[20].left, 10);
    assert_eq!(universe.triangles[20].right, 12);
    assert_eq!(universe.triangles[21].left, 29);
}

#[test]
fn test_moves02() {
    let mut universe = Universe::new(10, 100);

    // Check shard move to triangle with opposite orientation
    universe.do_move(10, 1);
    universe.do_move(11, 10);
    assert_eq!(universe.triangles[11].time, 20);
    assert_eq!(universe.triangles[11].left, 1);
    assert_eq!(universe.triangles[11].right, 2);
    assert_eq!(universe.triangles[20].left, 10);
    assert_eq!(universe.triangles[20].right, 12);
    assert_eq!(universe.triangles[21].left, 29);
}
 */
