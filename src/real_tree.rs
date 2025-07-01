use nalgebra::DMatrix;

pub struct WillowTree {
    /// The number of time steps
    pub N: usize,
    /// The number of spatial nodes at each time step
    pub m: usize,
    /// Time to end of tree
    pub T: f64,
    /// Time step size
    pub h: f64,
    /// The willow tree structure
    pub tree: DMatrix<f64>,
}
