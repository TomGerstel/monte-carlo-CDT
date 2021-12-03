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

    fn move_22(&mut self, position: usize) {
        // Perform a 2,2 move at position 'pos', if position is not suitable nothing is changed
        let neighbour: usize = self.triangles[position].right;
        if self.triangles[position].direction == Direction::Up && self.triangles[neighbour].direction == Direction::Down {
            // Up - Down pair, so change to Down - Up
            self.triangles[position].direction = Direction::Down;
            self.triangles[neighbour].direction = Direction::Up;
            let below: usize = self.triangles[position].time; // Neighbour in the previous time-slice
            self.triangles[position].time = self.triangles[neighbour].time;
            self.triangles[neighbour].time = below;
        } else if self.triangles[position].direction == Direction::Down && self.triangles[neighbour].direction == Direction::Up {
            // Down - Up pair, so change to Up - Down
            self.triangles[position].direction = Direction::Up;
            self.triangles[neighbour].direction = Direction::Down;
            let above: usize = self.triangles[position].time; //Neighbour in the next time-slice
            self.triangles[position].time = self.triangles[neighbour].time;
            self.triangles[neighbour].time = above;
        }
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
fn test_move_22() {
    let mut universe = Universe::new(10, 100);
    assert_eq!(universe.triangles[10].time, 20);
    universe.move_22(10);
    assert_eq!(universe.triangles[10].time, 1);
    assert_eq!(universe.triangles[11].time, 20);
    universe.move_22(11);
    assert_eq!(universe.triangles[11].time, 20);
}