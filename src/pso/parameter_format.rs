use super::population_format::Swarm;

#[derive(Debug)]
/**
 Generics <T> constrains all elements whose type havs relativity with that of `cur_place`
 For each individual,
    x(t+1)=x(t)+v(t+1)
    v(t+1)=omage*v(t)+dv1(t)+dv2(t)
    C1,C2 is used within dv1,dv2
 */
pub struct Param{
    pub d:usize,
    pub swarm_size:usize,
    pub max_gen:usize,//use `usize` for the convienence of not changing type later
    pub omage:f32,//ratio of cur_velocity in next_velocity
    pub c1:f32,
    pub c2:f32,
    pub r_num:usize,//num of r that can be chosen
    pub r_prob_min:f32,//a subtle param, no real meaning
    pub eval_range:usize,//adjacent gen that considered when calculate r,used in min{cue_gen-eval_rang,0}
   
} 
pub enum ReturnType {
    BestOne((Vec<f32>,f32)),
    Swarm(Swarm),
    Nothing(()),
}
pub struct Mode{
    /**
    whether to generate plot when itering  
    every max_gen/5 generate one,plus finial result one
     */
    pub plot_flag:bool, 
    /**
    whether to plot search angle's statistics
     */
    pub plot_search_angle:bool,
    /**
    whether to save automatically  
    every max_gen/5 generate one,plus finial result one
     */    
    pub _ckpt_flag:bool,
    /**
    choose the model to run
     */     
    pub model:u8, 
    /**
    choose what to output  
    `0` for None;`1` for best particle:Particle;`2` for the whole swarm:Swarm 
     */  
    pub output:u8, 

}