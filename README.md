# Self-Adaptive Rotational Invariant Particle Swarm Optimization
alias.sariPSO
This model comes from "A Self-adaptive Rotationally Invariant Particle Swarm Optimization for Global Optimization"\(T.Dong & H.Wang et al.)
Plus my modification
## Feature of sariPSO
- Rotational invariance: This allows PSO with robust performance for rotated problem.
- Self-adaptive: It would choose search area according history statistics, allowing more suitable search stratage.
## Envoriment
- Window 11
- rust 1.87
- mscv
## Usage
### Set problem
- where: cd /problem, then create file "$PROBLEM_NAME.rs", and add it to "mod.rs" in the same folder
- how: You can refer to existing problem.rs.First, copy the whole content in "rosenbrock.rs" into your problem file.
  Then, define your problem by changing the `fn rosenbrock()`. Note here, problem function take one particle per
  caculating time, and you can use elements in particle by "dot".\(Details of `struct Particle`,see /src/population_format.rs)
  Last,add your problem to /src/main.rs
### Customize parameter
Three class of parameters in struct
- `struct Objectinv`: define your problem's place constrain, declare your fitness function and denote your problem'name. Also,dimension of your problem is defined as `place_constrain.len()`
- `struct Parameter`: other than `swarm_size` and `max_gen` see /src/parameter_format.rs for more details.
- `struct mode`: It defines the PSO's behavior, see /src/parameter_format.rs for more details.
### Preset parameter
Directly replace corresponding thins in main.rs
For rotated rosenbrock function,use sariPSO, output particles' distribution figure, best particle and its fitness
```
let obj=Objective{
        place_constrain:vec![(-50.0,50.0);10],
        fitness:rotated_rf,
        fitness_name:"HappyCat".to_owned()
    };

    //set parameter
    let param=Param{
        d:obj.place_constrain.len(),
        swarm_size:1000,
        max_gen:1000,//use `usize` for the convienence of not changing type later
        omage:0.9,//ratio of cur_velocity in next_velocity
        c1:2.05,
        c2:2.05,
        r_num:5,//num of r that can be chosen
        r_prob_min:0.02,//a subtle param, no real meaning
        eval_range:10,//adjacent gen that considered when calculate r,used in min{cue_gen-eval_rang,0}
    };
    //set mode
    let mode=Mode{
        plot_flag:true,
        plot_search_angle:true,
        _ckpt_flag:true,
        model:1,
        output:1,
    };
```
### Complie and run
Please use release version to run, debug has issue of performance,casue persumed in rand.  
Complie and run
```
cargo run --release
```
### Result
- Best particle's info will be printed in terminal.
- Particles' distribution figure is in /figure.\(This folder must exist\)
- Search angle statistics' histogram is in /angle_figure.\(This folder must exist\)
