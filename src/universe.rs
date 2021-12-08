use std::collections::HashSet;
// use fastrand::Rng;

#[derive(Clone, Debug)]     
pub struct Universe {
    triangles: Vec<Triangle>,
    // TODO: No duplicate labels for order_four wanted, so HashSet is probably more suitable
    order_four: HashSet<usize>, // keeps a list of order 4 vertices, labelled by the top-left triangle
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
        let order_four = HashSet::new();
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
        let mut lengths = Vec::with_capacity(triangle_count); // TODO: can't this length be much shorter at least /2

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
            }; // TODO: This wastes a few walks for every slice just on finding the marker

            // walk through the slice and count the triangles
            // break loop if slice_origin is found
            // return if origin is found
            let mut slice_walker = self.triangles[slice_origin].right;
            'slice_walk: loop {
                if slice_walker == origin && t > 0 {
                    lengths.pop(); // a slice was counted double, remove it
                    return lengths;
                } else if slice_walker == slice_origin {
                    break 'slice_walk; // TODO: isn't the default to break out of inner loop?
                } else {
                    slice_walker = self.triangles[slice_walker].right;
                    lengths[t] += 1;
                }
            }

            // move to the next slice
            marker = self.triangles[slice_origin].time;
        }
    }

    pub fn mcmc_step(&mut self, move_ratio: f64) {
        if self.order_four.is_empty() || (fastrand::f64() < move_ratio) {
            // println!("Sampling Triangle Flip");
            let left = self.sample_triangle_flip();
            // println!("Performing Triangle Flip");
            self.triangle_flip(left);
        } else {
            // println!("Sampling Shard Move");
            let shard_up = self.sample_shard_move();
            let dest_up = self.sample_dest(shard_up);
            // println!("Performing Shard Move ({:} -> {:})", shard_up, dest_up);
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
        // Note: In random triangulation the probablity of having opposite orientation
        // on the right is 50%, so on average 2 searches are necessary, alternative is 
        // to keep a list of possible pairs, but this is likely less efficient.
        loop {
            let attempt = fastrand::usize(..self.triangles.len());
            let right = self.triangles[attempt].right;
            if self.triangles[attempt].orientation != self.triangles[right].orientation {
                return attempt;
            }
        }
    }

    fn sample_shard_move(&self) -> usize {
        let index = fastrand::usize(..self.order_four.len());
        self.triangles[*self.order_four.iter().nth(index).unwrap()].right
    }

    fn swap_orientation(&mut self, left: usize, right: usize) {
        let left_orientation = self.triangles[left].orientation;
        self.triangles[left].orientation = self.triangles[right].orientation;
        self.triangles[right].orientation = left_orientation;
    }

    fn triangle_flip(&mut self, left: usize) {
        // identify the relevant triangles
        let right = self.triangles[left].right;
        let left_nbr = self.triangles[left].time;
        let right_nbr = self.triangles[right].time;

        // flip the orientations
        self.swap_orientation(left, right);

        // reassign neighbours
        self.triangles[left_nbr].time = right;
        self.triangles[right_nbr].time = left;
        self.triangles[left].time = right_nbr;
        self.triangles[right].time = left_nbr;

        // update order_four
        // this can be optimised, some of these only have the possibility of
        // either becoming an order 4, or no longer being one (and not both)
        
        match self.triangles[left].orientation { // Check orientation after flip
            Orientation::Up => { // So this is the original down-up
                self.add_if_order_four(left_nbr);
                self.add_if_order_four(self.triangles[left].left);
                self.order_four.remove(&right);
                self.order_four.remove(&self.triangles[left_nbr].left);
            },
            Orientation::Down => { // So this is the original up-down
                self.add_if_order_four(right);
                self.add_if_order_four(self.triangles[right_nbr].left);
                self.order_four.remove(&right_nbr);
                self.order_four.remove(&self.triangles[left].left);
            }
        }

        // // Here the all the have the option of being either added see comment, also this must still check all 4 triangles
        // self.filter_order_four_at(right); // up-down: add
        // self.filter_order_four_at(right_nbr); // up-down: remove
        // self.filter_order_four_at(self.triangles[left].left); // up-down: remove
        // self.filter_order_four_at(self.triangles[right_nbr].left); // up-down: add

        // self.filter_order_four_at(right); // down-up: remove
        // self.filter_order_four_at(left_nbr); // down-up: add
        // self.filter_order_four_at(self.triangles[left].left); // down-up: add
        // self.filter_order_four_at(self.triangles[left_nbr].left); // down-up: remove
    }

    fn shard_move(&mut self, shard_up: usize, dest_up: usize) {
        // If move is to original position, do nothing
        let shard_nbr_left_up = self.triangles[shard_up].left;
        if dest_up == shard_nbr_left_up {
            return
        }

        // identify the relevant triangles
        let shard_down = self.triangles[shard_up].time;
        let dest_down = self.triangles[dest_up].time;

        // identify the shard's neighbours
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

        
        let dest_order4 = !self.order_four.insert(dest_up); // Add dest_up as order 4, and check if it already was
        // assert!(self.is_order_four_at(dest_up), "Dest Up");
        let shard_order4 = if dest_order4 { // Add shard_up as order 4 if dest_up already was or remove if not, and check if itself already was
            // assert!(self.is_order_four_at(shard_up), "Shard Up");
            !self.order_four.insert(shard_up)
        } else {
            self.order_four.remove(&shard_up)
        };
        if !shard_order4 { // Remove shard_nbr_left_up if shard_up was not already order 4
            self.order_four.remove(&shard_nbr_left_up);
            // assert!(!self.is_order_four_at(shard_nbr_left_up), "Shard Neighbour Left Up");
        } else {
            // assert!(self.is_order_four_at(shard_nbr_left_up), "Shard Neighbour Left Up");
        }
        

        // // These are combined more cleverly above
        // self.filter_order_four_at(dest_up); // Order 4: unknown -> yes
        // self.filter_order_four_at(shard_up); // Order 4: unknown -> iff dest_up was order 4
        // self.filter_order_four_at(shard_nbr_left_up); // Order 4: yes -> iff shard_up was order 4        
        
        // self.filter_order_four_at(shard_nbr_right_up); // I think this doesn't change
        // self.filter_order_four_at(dest_nbr_up); // I think this one doesn't change as well
    }

    // fn flip_orientation(&mut self, label: usize) {
    //     self.triangles[label].orientation = match self.triangles[label].orientation {
    //         Orientation::Down => Orientation::Up,
    //         Orientation::Up => Orientation::Down,
    //     }
    // }

    pub fn check_all_order_four(&self) -> bool {
        for label in 0..self.triangles.len() {
            if self.order_four.contains(&label) != self.is_order_four_at(label) {
                return false
            }
        }
        true
    }

    pub fn check_order_four_list(&self) -> bool{
        for label in self.order_four.iter() {
            if !self.is_order_four_at(*label) {
                return false;
            }
        }
        true
    }

    fn add_if_order_four(&mut self, label: usize) {
        if self.is_order_four_at(label) {
            self.order_four.insert(label);
        }
    }
    
    fn is_order_four_at(&self, label: usize) -> bool {
        self.triangles[label].orientation == Orientation::Up
            && self.triangles[self.triangles[label].right].orientation == Orientation::Up
            && self.triangles[self.triangles[label].time].right
                == self.triangles[self.triangles[label].right].time
    }

    // fn filter_order_four_at(&mut self, label: usize) {
    //     self.order_four.retain(|&l| l != label);
    //     if self.is_order_four_at(label) {
    //         self.order_four.push(label);
    //     }
    // }
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
