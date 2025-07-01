import ipyopt
from numpy import ndarray as np_array
import numpy as np

class OptimizationProblem:
    def __init__(self, q, z_bounds):
        """
        q: weights (q_i)
        z_bounds: list of tuples [(Z_0, Z_1), (Z_1, Z_2), ...] for bounds
        """
        self.q = np.array(q)
        self.m = len(q)
        self.z_bounds = z_bounds
        
    def objective(self, x):
        """Objective function: [sum(q_i * z_i^4) - 3]^2"""
        fourth_moment = np.sum(self.q * x**4)
        return (fourth_moment - 3.0)**2
    
    def gradient(self, x):
        """Gradient of objective function"""
        fourth_moment = np.sum(self.q * x**4)
        return 8 * self.q * x**3 * (fourth_moment - 3.0)
    
    def constraints(self, x):
        """Constraint functions g(x) = 0"""
        g1 = np.sum(self.q * x)  # first moment = 0
        g2 = np.sum(self.q * x**2) - 1.0  # second moment = 1
        return np.array([g1, g2])
    
    def jacobian(self, x):
        """Jacobian of constraints"""
        # Jacobian is 2 x m matrix
        jac = np.zeros((2, self.m))
        jac[0, :] = self.q  # gradient of g1
        jac[1, :] = 2 * self.q * x  # gradient of g2
        return jac.flatten()  # ipyopt expects flattened jacobian
    
    def solve(self, x0=None):
        """Solve the optimization problem"""
        if x0 is None:
            # Initialize with feasible point (approximately)
            x0 = np.random.randn(self.m)
            # Normalize to approximately satisfy second moment constraint
            x0 = x0 / np.sqrt(np.sum(self.q * x0**2))
        
        # Variable bounds
        xl = np.array([bound[0] for bound in self.z_bounds])
        xu = np.array([bound[1] for bound in self.z_bounds])
        
        # Constraint bounds (both equality constraints)
        gl = np.array([0.0, 0.0])
        gu = np.array([0.0, 0.0])
        
        # Number of variables and constraints
        n = self.m
        m_con = 2
        
        # Create ipyopt problem with all required parameters
        problem = ipyopt.Problem(
            n=n,
            x_l=xl,
            x_u=xu,
            m=m_con,
            g_l=gl,
            g_u=gu,
            sparsity_indices_jac_g=([], []),
            sparsity_indices_h=([], []),
            eval_f=self.objective,
            eval_grad_f=self.gradient,
            eval_g=self.constraints,
            eval_jac_g=self.jacobian
        )
        
        # Solve
        x_opt, info, _ = problem.solve(x0)
        return x_opt, info

def main():
    print("Solving constrained optimization problem with ipyopt")
    
    # Example problem setup
    m = 4  # number of variables
    q = np.array([0.25, 0.25, 0.25, 0.25])  # equal weights
    
    # Box constraints: z_i in [Z_{i-1}, Z_i]
    z_bounds = [(-2, -1), (-1, 0), (0, 1), (1, 2)]
    
    # Create and solve problem
    opt_problem = OptimizationProblem(q, z_bounds)
    
    try:
        solution, info = opt_problem.solve()
        
        print(f"Optimization successful: {info['status_msg']}")
        print(f"Optimal solution: {solution}")
        print(f"Objective value: {opt_problem.objective(solution)}")
        print(f"Constraint violations: {opt_problem.constraints(solution)}")
        
        # Verify constraints
        first_moment = np.sum(q * solution)
        second_moment = np.sum(q * solution**2)
        fourth_moment = np.sum(q * solution**4)
        
        print(f"\nVerification:")
        print(f"First moment (should be 0): {first_moment}")
        print(f"Second moment (should be 1): {second_moment}")
        print(f"Fourth moment: {fourth_moment}")
        print(f"Objective: {(fourth_moment - 3)**2}")
        
    except Exception as e:
        print(f"Optimization failed: {e}")

if __name__ == "__main__":
    main()