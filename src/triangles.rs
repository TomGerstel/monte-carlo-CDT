#[derive(Debug)]
struct Triangle {
    direction: Direction,
    left: usize,
    time: usize,
    right: usize
}

#[derive(Debug)]
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
                    (Direction::Up, (t+i + L) % N)
                } else {
                    (Direction::Down, (N + t+i - L) % N) // Add N to avoid subtraction overflow of usize
                };
                triangles.push(Triangle{
                    direction,
                    time,
                    left: t + ((L + i-1) % L), // Add L to avoid possible subtraction overflow of usize
                    right: t + ((i+1) % L)
                })
            }
        }
        return Universe{triangles};
    }

    // fn lengths(&self, origin: usize = 0) -> Vec<usize> {
    //     // Look at the lengths of the timeslices starting from an origin
    //     let lengths = Vector::new()
    // }
}

#[test]
fn test_triangle_universe(){
    let u0 = Universe::new(10, 100);
    println!("The universe: {:?}", u0.triangles);
}