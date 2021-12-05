#[derive(Debug)]
struct Triangle {
    direction: Direction,
    left: usize,
    time: usize,
    right: usize
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down
}

#[derive(Debug)]
struct Universe {
    triangles: Vec<Triangle>
}

impl Universe {
    fn new(T: usize, N: usize) -> Self {
        // Initialise a new universe with T timeslices
        // and L=T/N (rounded down) triangles per slice
        // as a flat universe (thus a cylindrical geometry)
        let L = N / T;
        let N = L * T;  // Update N to actual amount of triangles
        let mut triangles: Vec<Triangle> = Vec::new();
        for t in 0..T {
            for i in 0..L {
                let (direction, time) = if (t + i) % 2 == 0 {
                    (Direction::Up, (N + t*L+i - L) % N) // Add N to avoid subtraction overflow of usize
                } else {
                    (Direction::Down, (t*L+i + L) % N) 
                };
                triangles.push(Triangle{
                    direction,
                    time,
                    left: t*L + ((L + i-1) % L), // Add L to avoid possible subtraction overflow of usize
                    right: t*L + ((i+1) % L)
                })
            }
        }
        return Universe{triangles};
    }

    fn lengths(&self) -> Vec<usize> {
        // Look at the lengths of the timeslices starting from an origin
        // Do this by 'walking through each slice, and thereafter advancing
        // to the next slice until back to starting point.
        
        let start_key = 0; // TODO: Maybe tranform origin to function parameter?
        let mut lengths: Vec<usize> = Vec::new();
        let N = self.triangles.len();
        if N == 0 { // Check if not empty
            return Vec::new()
        }
        
        let mut key = start_key;
        for t in 0..(N/2 + 1) { // Likely much less time-slices than the maximum N/2+1, loop returns when last time-slices is covered
            lengths.push(1);
            let slice_start_key = key;
            let mut down_triangle: Option<usize> = None; // Find the first down-triangle for later advancement
            while {
                if down_triangle == None && self.triangles[key].direction == Direction::Down {
                    down_triangle = Some(key);
                }
                key = self.triangles[key].right; // Advance walk to right
                if t != 0 && key == start_key { // Return when back to the very beginning
                    lengths.pop();
                    return lengths;
                }
                key != slice_start_key
            } {
                lengths[t] += 1;
            }

            if let Some(down_key) = down_triangle { // Advance to next timeslice based on the first down triangles upper neighbour
                key = self.triangles[down_key].time;
            } else {
                dbg!("For some reason the searched timeslice had not down triangles, which should be impossible.");
                return Vec::new();
            }
        }
        dbg!("Triangulation walk took longer than should be possible, something went wrong in the search.");
        return Vec::new();
    }

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
        let (new_neighbour_left, new_time_left) = if {
            self.triangles[new_position].direction == self.triangles[old_position].direction
        } {
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
    }
}

#[test]
fn test_universe_lengths() {
    let T = 7;
    let N = 72;

    let u0 = Universe::new(T, N);
    assert_eq!(u0.lengths(), vec![N/T; T]);
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