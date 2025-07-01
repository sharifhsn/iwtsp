# -*- coding: utf-8 -*-
"""
Improved nonlinear optimization using JuMP and Ipopt.
"""
using JuMP
using Ipopt

println("Done loading packages.")

# --- Problem Data ---
m = 30
q = [0.01558822190112092, 0.0216736949157998, 0.025263154654762213, 0.02794640775787956, 0.03013487069165903, 0.03200474600938565, 0.033649575134551336, 0.03512561663999733, 0.03646962148539734, 0.037707058932142296, 0.03885638269578825, 0.03993143616656965, 0.04094289824450984, 0.04189919785855301, 0.04280711691188369, 0.04280711691188369, 0.04189919785855301, 0.04094289824450984, 0.03993143616656965, 0.03885638269578825, 0.037707058932142296, 0.03646962148539734, 0.03512561663999733, 0.033649575134551336, 0.03200474600938565, 0.03013487069165903, 0.02794640775787956, 0.025263154654762213, 0.0216736949157998, 0.01558822190112092]
Z_bounds = [-3.0, -2.154813341554466, -1.7833838913613136, -1.5339167165110092, -1.3378573012010246, -1.1719610363878912, -1.0252988635663451, -0.8917604546545208, -0.7675197160795298, -0.6499696621466611, -0.5372051182121156, -0.42774236591438064, -0.320351913092292, -0.21394830480998098, -0.10750826831304774, 0.0, 0.10750826831304774, 0.21394830480998098, 0.320351913092292, 0.42774236591438064, 0.5372051182121156, 0.6499696621466611, 0.7675197160795298, 0.8917604546545208, 1.0252988635663451, 1.1719610363878912, 1.3378573012010246, 1.5339167165110092, 1.7833838913613136, 2.154813341554466, 3.0]
z_initial = [-2.5774066707772327, -1.9690986164578899, -1.6586503039361613, -1.435887008856017, -1.254909168794458, -1.0986299499771182, -0.958529659110433, -0.8296400853670254, -0.7087446891130955, -0.5935873901793883, -0.4824737420632481, -0.37404713950333635, -0.2671501089511365, -0.16072828656151436, -0.05375413415652387, 0.05375413415652387, 0.16072828656151436, 0.2671501089511365, 0.37404713950333635, 0.4824737420632481, 0.5935873901793883, 0.7087446891130955, 0.8296400853670254, 0.958529659110433, 1.0986299499771182, 1.254909168794458, 1.435887008856017, 1.6586503039361613, 1.9690986164578899, 2.5774066707772327]

# Define the lower and upper bounds for z
lb = Z_bounds[1:m]
ub = Z_bounds[2:m+1]

# --- JuMP Model ---
# Create a new JuMP model and specify the Ipopt solver
model = Model(Ipopt.Optimizer)

# Define the variables 'z' with their bounds and initial values
@variable(model, lb[i] <= z[i=1:m] <= ub[i], start = z_initial[i])

# Define the objective function
# min (sum(q_i * z_i^4) - 3)^2
@objective(model, Min, (sum(q[i] * z[i]^4 for i in 1:m) - 3.0)^2)

# Define the equality constraints
# sum(q_i * z_i) = 0
@constraint(model, mean_constraint, sum(q[i] * z[i] for i in 1:m) == 0)

# sum(q_i * z_i^2) = 1
@constraint(model, variance_constraint, sum(q[i] * z[i]^2 for i in 1:m) == 1)


println("Solving the optimization problem with JuMP and Ipopt...")
# Solve the model
optimize!(model)


# --- Results ---
println("\n--- Results ---")
println("Termination status: ", termination_status(model))
println("Primal status: ", primal_status(model))
println("Objective value: ", objective_value(model))

if termination_status(model) == MOI.LOCALLY_SOLVED
    solution_z = value.(z)
    println("\nOptimal z values:")
    println(solution_z)

    # You can also verify the constraints with the solution
    mean_val = sum(q .* solution_z)
    variance_val = sum(q .* solution_z.^2)
    kurtosis_val = sum(q .* solution_z.^4)
    println("\nVerification of constraints:")
    println("Mean: ", mean_val)
    println("Variance: ", variance_val)
    println("Sum of q*z^4 (related to kurtosis): ", kurtosis_val)
else
    println("\nCould not find an optimal solution.")
end