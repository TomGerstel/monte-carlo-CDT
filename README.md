
- Topic theme: 2D Quantum Gravity using Causal Dynamical Triangulations. Challenges include finding meaningful observables and interpreting them.
- Topic goal: Create a MCMC that generates triangulations according to the desired distribution. Measure observables such as timeslice sizes, lightcone structure, curvature, etc. Potentially study effects of switching to a toroidal topology.
- Model: Either a cylindrical or toroidal 2D CDT QG. Parameters include the coupling lambda, number of time slices T, number (or distribution) of triangles. For the cylindrical model also the spatial size of the initial time slice is an input parameter.
- Implementation: A MCMC model exists for our use case. The potential for direct sampling algorithms is also considered.
- Parameter range: This depends on our implementation, we will mainly investigate this as we go. 
- Data & code management: We use this GitHub repository to store our code, report and data. The data we record will likely have to be the entire triangulation for various timesteps, the MC timestep of the measurement, as well as all input parameters for the given simulation.
- Data analysis: Data analysis will mainly consist of finite-size scaling approaches. Error analysis can be done using batching.