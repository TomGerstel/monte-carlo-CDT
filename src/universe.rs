#[derive(Clone, Debug)]
pub struct Universe {
    triangles: Vec<Triangle>,
    allow_move: Vec<usize>, // keeps a list of order 4 vertices, labelled by the top-left triangle
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
        let allow_move = vec![];
        Universe {
            triangles,
            allow_move,
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
            let t = lengths.len();

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
            let mut slice_walker = slice_origin;
            loop {
                slice_walker = self.triangles[slice_walker].right;
                if slice_walker == origin {
                    if t > 1 {
                        lengths.pop(); // a slice was counted double, remove it
                        return lengths;
                    }
                } else if slice_walker == slice_origin {
                    break;
                } else {
                    lengths[t] += 1;
                }
            }

            // move to the next slice
            if self.triangles[slice_origin].orientation == Orientation::Up {
                marker = self.triangles[slice_origin].time;
            } else {
                panic!("triangles are not properly connected");
            }
        }
    }

    pub fn mcmc_step(&mut self, move_ratio: f32) {
        dbg!(self.clone());
        for l in self.allow_move.clone() {
            if !self.is_allow_move_at(l) {
                println!(
                    "triangle {} is marked as an allow_move, but this is false!!!",
                    l
                );
            }
        }
        //dbg!(self.allow_move.clone());
        let is_flip = fastrand::f32() < move_ratio;
        if is_flip || self.allow_move.is_empty() {
            let left = self.sample_triangle_flip();
            println!("flip {}", left);
            self.triangle_flip(left);
        } else {
            let shard_up = self.sample_shard_move();
            let dest_up = self.sample_up();
            println!("move {} to {}", shard_up, dest_up);
            self.shard_move(shard_up, dest_up);
        }
    }

    fn sample_up(&self) -> usize {
        let index = fastrand::usize(..self.triangles.len());
        match self.triangles[index].orientation {
            Orientation::Up => index,
            Orientation::Down => self.triangles[index].time,
        }
    }

    fn sample_triangle_flip(&self) -> usize {
        let mut i = 0;
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
            i += 1;
            if i > 100 {
                dbg!(self.clone());
                panic!();
            }
        }
    }

    fn sample_shard_move(&self) -> usize {
        let index = fastrand::usize(..self.allow_move.len());
        self.triangles[self.allow_move[index]].right
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

        // update allow_move
        self.filter_allow_move_at(left);
        self.filter_allow_move_at(right);
        self.filter_allow_move_at(left_nbr);
        self.filter_allow_move_at(right_nbr);
        self.filter_allow_move_at(self.triangles[left].left);
        self.filter_allow_move_at(self.triangles[right].right);
        self.filter_allow_move_at(self.triangles[self.triangles[left].left].time);
        self.filter_allow_move_at(self.triangles[self.triangles[right].right].time);
        self.filter_allow_move_at(self.triangles[left_nbr].left);
        self.filter_allow_move_at(self.triangles[left_nbr].right);
        self.filter_allow_move_at(self.triangles[right_nbr].left);
        self.filter_allow_move_at(self.triangles[right_nbr].right);
    }

    fn shard_move(&mut self, shard_up: usize, dest_up: usize) {
        if self.triangles[shard_up].orientation != Orientation::Up {
            dbg!(self.clone());
            panic!();
        }
        if self.triangles[dest_up].orientation != Orientation::Up {
            dbg!(self.clone());
            panic!();
        }
        // identify the relevant triangles
        let shard_down = self.triangles[shard_up].time;
        let dest_down = self.triangles[dest_up].time;

        // identify the relevant neighbours
        let shard_nbr_left_up = self.triangles[shard_up].left;
        let shard_nbr_right_up = self.triangles[shard_up].right;
        let shard_nbr_left_down = self.triangles[shard_down].left;
        let shard_nbr_right_down = self.triangles[shard_down].right;
        let dest_nbr_up = self.triangles[dest_up].right;
        let dest_nbr_down = self.triangles[dest_down].right;

        // reassign neighbours around original location (close the gap left behind by the shard)
        self.triangles[shard_nbr_left_up].right = shard_nbr_right_up;
        self.triangles[shard_nbr_right_up].left = shard_nbr_left_up;
        self.triangles[shard_nbr_left_down].right = shard_nbr_right_down;
        self.triangles[shard_nbr_right_down].left = shard_nbr_left_down;

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

        // update allow_move
        self.filter_allow_move_at(shard_nbr_left_up);
        self.filter_allow_move_at(shard_nbr_right_up);
        self.filter_allow_move_at(dest_up);
        self.filter_allow_move_at(shard_up);
        self.filter_allow_move_at(dest_nbr_up);
    }

    fn flip_orientation(&mut self, label: usize) {
        self.triangles[label].orientation = match self.triangles[label].orientation {
            Orientation::Down => Orientation::Up,
            Orientation::Up => Orientation::Down,
        }
    }

    fn is_allow_move_at(&self, label: usize) -> bool {
        self.triangles[label].orientation == Orientation::Up
            && self.triangles[self.triangles[label].right].orientation == Orientation::Up
            && self.triangles[self.triangles[label].time].right
                == self.triangles[self.triangles[label].right].time
        //&& self.triangles[label].right != label
        //&& self.triangles[self.triangles[label].time].right != self.triangles[label].time
    }

    fn filter_allow_move_at(&mut self, label: usize) {
        self.allow_move.retain(|&l| l != label);
        if self.is_allow_move_at(label) {
            self.allow_move.push(label);
        }
    }
    /*
    fn do_move(&mut self, position: usize, new_position: usize) {
        let neighbour: usize = self.triangles[position].right;
        if self.triangles[position].direction == Direction::Up {
            if self.triangles[neighbour].direction == Direction::Down {
                self.flip_triangles(neighbour, position) // Up - Down pair, so change to Down - Up
            } else {
                self.move_shard(position, neighbour, new_position) // Up - Up, so move shard
            }
        } else {
            if self.triangles[neighbour].direction == Direction::Up {
                self.flip_triangles(position, neighbour) // Down - Up pair, so change to Up - Down
            } else {
                self.move_shard(position, neighbour, new_position) // Down - Down, so move shard
            }
        }
    }

    fn move_shard(&mut self, old_position: usize, old_neighbour_right: usize, new_position: usize) {
        // Remove shard from old position
        // Set connections between direct neighbours
        let old_neighbour_left = self.triangles[old_position].left;
        self.triangles[old_neighbour_right].left = old_neighbour_left;
        self.triangles[old_neighbour_left].right = old_neighbour_right;
        // Find timelike neighbour/other part of the shard
        let old_position_time: usize = self.triangles[old_position].time;
        // Set connections between neighbours of timelike neighbour
        let old_time_right = self.triangles[old_position_time].right;
        let old_time_left = self.triangles[old_position_time].left;
        self.triangles[old_time_right].left = old_time_left;
        self.triangles[old_time_left].right = old_time_right;

        // Add shard at new position (to the right of the shard at new_position)
        // Select similarly oriented part of the shard to original position
        let (new_neighbour_left, new_time_left) =
            if { self.triangles[new_position].direction == self.triangles[old_position].direction }
            {
                (new_position, self.triangles[new_position].time)
            } else {
                (self.triangles[new_position].time, new_position)
            };
        // Get neighbours of the old shard
        let new_neighbour_right = self.triangles[new_neighbour_left].right;
        let new_time_right = self.triangles[new_time_left].right;
        // Set connections to new inserted shard
        self.triangles[new_neighbour_left].right = old_position;
        self.triangles[new_time_left].right = old_position_time;
        self.triangles[new_neighbour_right].left = old_position;
        self.triangles[new_time_right].left = old_position_time;
        // Set connections on new inserted shard
        self.triangles[old_position].left = new_neighbour_left;
        self.triangles[old_position].right = new_neighbour_right;
        self.triangles[old_position_time].left = new_time_left;
        self.triangles[old_position_time].right = new_time_right;
    }

    fn flip_triangles(&mut self, toup: usize, todown: usize) {
        // Flip triangle pair toup, todown: upward and downward respectively
        // Triangles must be beside one other, and toup must be down and todown up.
        self.triangles[toup].direction = Direction::Up;
        self.triangles[todown].direction = Direction::Down;
        // Interchange timelike neighbours
        let above: usize = self.triangles[toup].time;
        let below: usize = self.triangles[todown].time;
        self.triangles[toup].time = below;
        self.triangles[todown].time = above;
        self.triangles[above].time = todown;
        self.triangles[below].time = toup;
    } */
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
