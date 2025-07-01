The following is documentation for the Rooc file format for defining optimization problems.

# Table of contents



-   [Objective function](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_objective_function)

-   [Constraints](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_constraints)

-   [Variables and compound variables](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_variable)

-   [Data](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_data)

-   Blocks

    -   [Expansion blocks](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_expansion_blocks)

    -   [Scoped expansion blocks](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_scoped_blocks)

-   [Domains](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_domains)

-   [Functions and tuples](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_functions_and_tuples)

-   [Other things](https://rooc.specy.app/docs/rooc/rooc-syntax#rooc_others)



# Objective function



The objective function can be one of either "min" or "max", after the keyword, you can define whichever expression you wish to optimize.  

Once the model is compiled, it will be ran through the solver and find the optimal solution which fits the constraints.  

Some solvers allow you to also work on finding a satisfiable solution, which is a solution that fits the constraints, not caring about the objective function. in that case, instead of writing the min/max keyword and objective function, you can use the "solve" keyword.



    min x + y / 2



# Constraints



The formal model can follow a list of constraints, you can use one of <=, >=, =, <, > comparisons. Some solvers like the simplex, do not allow for strict inequalities <, >.  

The special keyword "for" can be used at the end of a constraint to create a constraint for each element that you iterate over.  

You can also give a name to constraints, so that you can more easily recognise them in the output. To do that, just write the name of the constraint followed by a colon ":". (eg. "myConstraint: x + y = 2"), you can also treat the constraint name as a compound variable and use the iteration syntax.



    something: y >= x + 2

    const_i: x * i <= i for i in 0..5



# Variables and compound variables



The language employs two execution environments: a formal model and a compiled model. Within the formal model, you can define three types of variables:



-   **Compound variables:** These variables have names determined _after_ compilation.

-   **Constant variables:** These variables have their values replaced during compilation (e.g., substituting a symbolic name with a concrete number).

-   **Standard variables:** These variables retain their names and values after compilation.



### Compound Variables



A compound variable is identified by an underscore (`_`) in its name. The portion of the name preceding the first underscore serves as a prefix. The remaining parts (split by underscores) are treated as an expression that must evaluate to a string, number, or node.



For example, in the compound variable `x_i`, `x` is the prefix, and `i` is a variable whose value will be used during compilation to construct the final variable name. If `i` has the value `3`, the compiled variable name would be `x_3`.



Compound variables are particularly useful when iterating over a list and dynamically generating variable names based on each element's value. See the example below for further clarification.



You can use curly braces `{}` to enclose more complex expressions within the compound variable name. If the expression is a simple number or a single variable name, the curly braces can be omitted. For instance, `data_{i + 1}` allows for more complex index calculations, while `item_1` or `value_count` are simpler examples.



### Escaping Compound Variable Names



If you need to use a variable name that _looks_ like a compound variable but should be treated literally, you can escape the name using a backslash (`\`). For example, `\x_hello` will be interpreted as the literal variable name `x_hello`, preventing the evaluation of `hello`.



    min 1

    s.t.

        x_i + \x_hello <= 1

        x_{i + 1}_i <= 2

    where

        let i = 3



will be compiled to



    min 1

    s.t.

        x_3 + x_hello <= 1

        x_4_3 <= 2



# Data



Following the constraint definitions, you can define data within the `where` section. This data is then available for use throughout your model.



The ROOC language supports various data types, including arrays, matrices, graphs, strings, numbers, and boolean values. Furthermore, you can use expressions, function calls, and other computational constructs within the `where` section.



To define a named constant, use the `let` keyword followed by the constant's name and its value. For example:



    let A = [1, 2, 3]

    let B = [

        [1, 2, 3],

        [4, 5, 6]

    ]

    let G = Graph {

        A -> [ C, B:2 ],

        B -> [ A, C:-3 ],

        C

    }

    let lengthOfA = len(A)

    let someString = "hello"

    let someBool = true



# Expansion blocks



Expansion blocks are a special type of expression macro, used to preprocess other expressions. A common example is the `avg` block, which takes a comma-separated list of expressions and expands it calculate the arithmetic average of those expressions.



    avg{ x, y, z }



This will be compiled to:



    (x + y + z) / 3



There are different kinds of expansion blocks, you can find them in the documentation.



# Scoped expansion blocks



There are also special kinds of expansion blocks, which have a _scope_ attached to it.  

In the normal expansion blocks, you need to manually specify the different expressions, separating them with a comma. The Scoped expansion blocks, you specify a template (which is the expression inside of the block), and an iteration scope (eg. iterating over a list).  

Together with compound variables and scoped expansion blocks, you can do things like creating a summation over a list or range.  

As an example, here is creating a summation of x\_u, where u is a number from 0 to 3 (3 excluded)



    sum(u in 0..3) { x_u }



will be compiled to:



    x_0 + x_1 + x_2



there are different scoped expansion blocks that can be used to expand the expressions, you can find them in the documentation.



# Domains



After the data you can define in which domain each variable will be part of, those variables are the ones that will remain after the compilation is finished. The domain knowledge will then be used by solvers.  

  

Every variable that will end up in the compiled model must be defined, you can use the "for" iteration like in the constraints to define compound variables.  

The domains are "Real", "NonNegativeReal", "Boolean" and "IntegerRange".  

You can define a minimum and maximum value for each domain except for the "Boolean" domain. They are required for the "IntegerRange" domain, and optional for Real (which defaults to -inf and inf) and NonNegativeReal (which defaults to 0 and inf).



    y, x_u as IntegerRange(0,20) for u in 0..5



# Functions and tuples



The ROOC langage has a set of builtin functions that can be used to manipulate data.  

Those functions can be run anywhere in the model or data section.  

  

The language also has support for tuples and tuples destructuring, you can destructure a tuple or array by writing the name of the variables inside a parenthesis "(a,b,c)". Some builtin values are destructurable, like arrays, tuples and graph edges.



    sum((el, idx) in enumerate([10, 20,30])) { x_idx * el}



will be compiled to



    x_0 * 10 + x_1 * 20 + x_2 * 30



# Other things



You can write comments in the model by using the "//" or "/\* \*/" syntax, a model is structured (in this order) by the objective function, constraints, data and domains.



    min sum(i in A) { x_i }

    s.t.

        /*

            here you can put your constraints

        */

        avg(i in A) { x_i } <= 2

        x_i >= minimum for i in A

    where

        // here your constants.

        let A = [1, 2, 3]

        let minimum = 1

    define

        // and here the domain of the variables

        x_i as Real for i in A



# Rooc runtime



Inside Rooc, you can use the functions defined inside of the runtime to perform operations on the data. There are 3 kinds of functions:



-   [Normal functions](https://rooc.specy.app/docs/rooc/rooc-runtime#functions): They accept an input and return an output

-   [Block functions](https://rooc.specy.app/docs/rooc/rooc-runtime#block_functions): They have a block where you can write an expression, and the block function will do some operation over the block

-   [Block scoped functions](https://rooc.specy.app/docs/rooc/rooc-runtime#block_scoped_functions): Like block functions, but they have an iteration scope. Inside of the scope you can define how to iterate over the data, and it will be available in the block for you to use.



## Functions



They are functions that accept parameters and return a value, you can use them inside blocks, assignments or in expressions  



    x * len(A) <= 2



    len(of_iterable: Any[]): PositiveInteger



Returns the length of the iterable



    enumerate(of_iterable: Any[]): (Any, PositiveInteger)[]



Enumerates the iterable, returning a tuple of the element and its index



    enum(of_iterable: Any[]): (Any, PositiveInteger)[]



Enumerates the iterable, returning a tuple of the element and its index



    edges(of_graph: Graph): GraphEdge[]



Returns the edges of a graph



    E(of_graph: Graph): GraphEdge[]



Returns the edges of a graph



    nodes(of_graph: Graph): GraphNode[]



Returns the nodes of a graph



    V(of_graph: Graph): GraphNode[]



Returns the nodes of a graph



    neigh_edges(of_node: GraphNode): GraphEdge[]



Returns the neighbour edges of a node



    N(of_node: GraphNode): GraphEdge[]



Returns the neighbour edges of a node



    neigh_edges_of(of_node_name: String, in_graph: Graph): GraphEdge[]



Returns the neighbour edges of a node name in a graph



    N_of(of_node_name: String, in_graph: Graph): GraphEdge[]



Returns the neighbour edges of a node name in a graph



    range(from: Integer, to: Integer, to_inclusive: Boolean): Integer[]



Returns an iterable of integers from \`from\` to \`to\` (inclusive or exclusive)



    zip(arg1: Any[], arg2: Any[]): (Any, Any)[]



Zips two or more iterables together, returning a tuple of the elements, the length of the resulting iterable is the length of the shortest iterable



    difference(first: Any[], second: Any[]): Any[]



Returns the difference of two iterables



    union(first: Any[], second: Any[]): Any[]



Returns the union of two iterables



    intersection(first: Any[], second: Any[]): Any[]



Returns the intersection of two iterables



## Block functions



They are blocks which have one or more expressions separated by a comma, they will use those expressions to perform a transformation, like the avg (average) block  



    avg {x_1, x_2, x_3}



    min{ }



Computes the inner expression as the minimum of all elements



    max{ }



Computes the inner expression as the maximum of all elements



    avg{ }



Computes the inner expression as the average of all elements



## Block scoped functions



They are function blocks, it has as parameters one or more iterators over iterable data, they will declare a variable (or more using tuples destructuring) for each iterator and then execute the block.  

If there are more than one iterators, they will behave as nested iterators, where the first iterator is the outermost one  



    sum(i in 0..len(A), el in A[i]) { x_i * el }



    sum(){ }



Expands the inner expression into a sum of all elements



    prod(){ }



Expands the inner expression into a product of all elements



    min(){ }



Expands the inner expression into the minimum of all elements



    max(){ }



Expands the inner expression into the maximum of all elements



    avg(){ }



Expands the inner expression into the average of all elements



# Execution pipes



Other than the model, you can define an execution pipeline that you can customise to do what you need.  

There are some presets you can choose from, but in general, each pipe step has an input and produces an output, each step of the pipeline will be executed one after the other in the order they are defined, and each result will be shown as the output.



Name:



**Compiler**



Input:



`String`



Output:



`Parser`



Compiles the code



Name:



**Pre Model**



Input:



`Parser`



Output:



`Pre Model`



Generates a model from the compiler output



Name:



**Model**



Input:



`Pre Model`



Output:



`Model`



Run the Pre Model to generate the static model



Name:



**Linear model**



Input:



`Model`



Output:



`Linear Model`



Transforms the model into a linear model



Name:



**Standard linear model**



Input:



`Linear Model`



Output:



`Standard Linear Model`



Transforms the linear model into a model in standard form



Name:



**Tableau for simplex**



Input:



`Standard Linear Model`



Output:



`Tableau`



Transforms the standard linear model into a tableau that can be used in the simplex algorithm, it creates artificial variables to find the initial basis



Name:



**Real solver**



Input:



`Linear Model`



Output:



`Real Solution`



Runs a real variable solver to find the optimal solution, the variables must be real or non negative real



Name:



**Simplex solver with steps**



Input:



`Tableau`



Output:



`Optimal Tableau with Steps`



Runs the simplex algorithm to find the optimal solution and returns the tableau at each step



Name:



**Binary solver**



Input:



`Linear Model`



Output:



`Binary Solution`



Runs a binary variable solver to find the optimal solution, the variables must be binary



Name:



**Integer binary solver**



Input:



`Linear Model`



Output:



`Integer Binary Solution`



Runs a binary and integer variable solver to find the optimal solution, the variables must be binary or integer



Name:



**MILP solver**



Input:



`Linear Model`



Output:



`MILP Solution`



Runs a solver that allows for real, integer and binary variables to find the optimal solution.



Name:



**Auto solver**



Input:



`Linear Model`



Output:



`MILP Solution`



Automatically picks the best solver to run for this model



Name:



**HiGHS solver**



Input:



`Linear Model`



Output:



`MILP Solution`



A high performance MILP solver, using a linear model



Name:



**HiGHS solver (Cplex LP)**



Input:



`String`



Output:



`MILP Solution`



A high performance MILP solver, using the Cplex LP format



Name:



**To Cplex LP**



Input:



`Linear Model`



Output:



`String`



Converts a linear model to the Cplex LP format



# Solvers



ROOC allows you to use different kinds of solvers depending on the problem you are trying to solve.  

  

The problems you write might have different types of variables (Real, Integer, Binary), and not all solvers can use all of them.  

You can quickly select which solver you want to use by choosing the "preset" next to the "run" button, or create your own pipeline.  

Here are the main requirements and differences for each solver:



-   **HiGHS solver**: It uses the [HiGHS](https://highs.dev/) solver, the most complete and performant solver in ROOC. It allows you to solve any MILP problem, so problems with any kind of variables. (Recommended)

-   **Auto solver**: It picks the most appopriate solver depending on the problem you are trying to solve, it is the default.

-   **MILP solver**: It uses the [microlp](https://github.com/Specy/microlp/) solver, a simple solver for MILP problems (like HiGHS), slower than HiGHS.

-   **Real solver**: It uses the [Clarabel](https://clarabel.org/stable/) solver, a solver for models that use only Real or NonNegativeReal variables.

-   **Binary solver**: It uses the [Copper](https://github.com/ffminus/copper) solver, a constraint programming solver that can solve problems with binary variables. All coefficients must be integers (they will be rounded otherwise).

-   **Integer & binary solver**: It uses the [Copper](https://github.com/ffminus/copper) solver, a constraint programming solver that can solve problems with binary and integer variables. All coefficients must be integers (they will be rounded otherwise).

-   **Simplex solver Step by Step**: It uses the internal rooc solver to show how to solve a linear problem by using the simplex method step by step. It might be useful to understand how the simplex method works, but is not reccomended to actually solve problems as it might have errors and is slow.



Alternatively, if you don't want to use the ROOC syntax to create the model, you can use the [CPLEX LP](https://www.ibm.com/docs/en/icos/22.1.2?topic=cplex-lp-file-format-algebraic-representation) format, which is a standard format to represent linear problems. And then you can choose the HiGHS CPLEX solver to solve it by using the HiGHS solver.



You can also write the model using the ROOC syntax, to then convert it to the CPLEX LP format, this way you can use any other solver that supports the CPLEX LP format.



# Examples



Here are some classic examples of optimization problems that can be solved using ROOC.



## Diet Problem



The diet problem is a classic optimization problem where the goal is to find the optimal diet that meets the nutritional requirements at the lowest cost.



    /*

        This the diet problem, minimize the cost of the diet while 

        staying between the limits of each nutrient

    */

    min sum((cost, i) in enumerate(C)) { cost * x_i }

    s.t.  

    

        min_{nutrient[j]}: //the diet must have at least of nutrient j

            sum(i in 0..f) { a[i][j] * x_i} >= Nmin[j] for j in 0..len(Nmin)

        

        max_{nutrient[j]}: //the diet must have at most of nutrient j

            sum(i in 0..f) { a[i][j] * x_i } <= Nmax[j] for j in 0..len(Nmax)

    

    where    

        // Cost of chicken, rice, avocado

        let C = [1.5, 0.5, 2.0]

        // Min and max of: protein, carbs, fats

        let nutrient = ["protein", "carbs", "fats"]

        let Nmin = [50, 200, 0] 

        let Nmax = [150, 300, 70]

        // Min and max servings of each food    

        let Fmin = [1, 1, 1] 

        let Fmax = [5, 5, 5]

        let a = [

            //protein, carbs, fats        

            [30, 0, 5], // Chicken

            [2, 45, 0], // Rice

            [2, 15, 20] // Avocado    

        ]

        // Number of foods

        let f = len(a)

        // Number of nutrients

        let n = len(Nmax)

    define

        //bound the amount of each serving of food i

        x_i as NonNegativeReal(Fmin[i], Fmax[i]) for i in 0..n



## Dominating Set Problem



In the dominating set problem, the goal is to find the smallest set of nodes in a graph such that every node in the graph is either in the set or adjacent to a node in the set, as in, the nodes are either dominant or adjacent to a dominant node (dominated).



    //minimize the number of selected nodes

    min sum(u in nodes(G)) { x_u }

    s.t. 

        // the variable "_" will simply ignore the value

        x_v + sum((_, u) in neigh_edges(v)) { x_u } >= 1 for v in nodes(G)

    where

        let G = Graph {

            A -> [B, C, D, E, F],

            B -> [A, E, C, D, J],

            C -> [A, B, D, E, I],

            D -> [A, B, C, E, H],

            E -> [A, B, C, D, G],

            F -> [A, G, J],

            G -> [E, F, H],

            H -> [D, G, I],

            I -> [C, H, J],

            J -> [B, F, I]

        }

    define

        x_u, x_v as Boolean for v in nodes(G), (_, u) in neigh_edges(v)

    



## Knapsack Problem



In the knapsack problem, you are given a set of items, each with a weight and a value, and a knapsack with a maximum capacity. The goal is to maximize the total value of the items in the knapsack without exceeding the capacity.



    //maximize the value of the bag

    max sum((value, i) in enumerate(values)) { value * x_i }

    s.t.

        //make sure that the selected items do not go over the bag's capacity

        sum((weight, i) in enumerate(weights)) { weight * x_i } <= capacity

    where

        let weights = [10, 60, 30, 40, 30, 20, 20, 2]

        let values = [1, 10, 15, 40, 60, 90, 100, 15]

        let capacity = 102

    define

        x_i as Boolean for i in 0..len(weights)

    



## Knapsack Problem



The same knapsack problem as before, but creating variables for the weight and cost, together with a named constraint to see the final weight of the bag.



    //maximize the value of the bag

    max value

    s.t.

        //make sure that the selected items do not go over the bag's capacity

        weight: weight <= capacity

    

        weight = sum((w, i) in enumerate(weights)) { w * x_i } 

        value  = sum((v, i) in enumerate(values)) { v * x_i }

    where

        let weights = [10, 60, 30, 40, 30, 20, 20, 2]

        let values = [1, 10, 15, 40, 60, 90, 100, 15]

        let capacity = 102

    define

        x_i as Boolean for i in 0..len(weights)

        weight, value as NonNegativeReal



## Simple machining problem



Imagine you run a small factory that makes two types of products: A and B. Each product requires time on two machines, Machine 1 and Machine 2. Your goal is to maximize profit, but you’re limited by how many hours each machine is available each day.



    //how much each product will earn you

    max sum((v, i) in enum(value)) { x_i * v }

    subject to

        //the machines need to be within the maximum machining time

        sum((time, j) in enum(machiningTime[i])){ x_j * time } <= timeLimit[i] for i in 0..len(value)

    where 

        let value = [10, 15]

        let timeLimit = [8, 6]

        let machiningTime = [

            [1, 2], // how much time machine A needs to make product A and B

            [2, 1]  // same but for machine B

        ]

    define 

        x_i as NonNegativeReal for i in 0..len(value)



T
