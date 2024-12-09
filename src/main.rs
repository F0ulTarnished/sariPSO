#[allow(unused_imports)]
use problem::{rosenbrock::rosenbrock, rotated_rf::rotated_rf,happycat::happycat};
use pso::{objective_format::Objective, parameter_format::{Mode, Param, ReturnType}, sari_pso::sari_pso_alg};

mod pso;
mod problem;

fn main() {
    let obj=Objective{
        place_constrain:vec![(-50.0,50.0);10],
        fitness:rotated_rf,
        fitness_name:"HappyCat".to_owned()
    };

    //set parameter
    let param=Param{
        d:obj.place_constrain.len(),
        swarm_size:500,
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
        plot_flag:false,
        plot_search_angle:true,
        _ckpt_flag:true,
        model:1,
        output:1,
    };
    let best_solution=sari_pso_alg(&param, &obj, &mode);
    match best_solution.unwrap(){
        ReturnType::Nothing(_)=>println!("PSO completed"),
        ReturnType::BestOne((vec,v))=>{
            println!("Best soluton:{:?}",vec);
            println!("Best fitness:{:?}",v);
            }
            ReturnType::Swarm(swarm)=>{
            println!("Best soluton:{:?}",swarm.gb);
            println!("Best fitness:{:?}",swarm.gb_val);    
            }
        }
}
