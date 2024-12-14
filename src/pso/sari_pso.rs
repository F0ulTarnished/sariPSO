use indicatif::{MultiProgress, ProgressBar};

use super::{objective_format::Objective, parameter_format::{Mode, Param, ReturnType}, population_format::{Particle, Swarm}, pso_utils::{determine_r, restrain_place, update_place, sari_update_velocity,classic_update_velocity}};

//frame og sariPSO
/**
 The alg will generate swarm within itself,you can choose what to output
 */
pub fn sari_pso_alg(param:&Param,obj:&Objective,mode:&Mode)->Option<ReturnType>{
    //Initialze swarm,pb,gb
    let mut swarm=Swarm::new(param.d, param.swarm_size, &obj.place_constrain);
    swarm.update_obj(obj.fitness);
    //info needed in determine_r
    let mut late_info=LateInfo::new(param.r_num, param.eval_range);
    let mut search_angle_info:[usize;80]=[0;80];
    //iteration
    let muti_bar=MultiProgress::new();
    let gen_bar=muti_bar.add(ProgressBar::new(param.max_gen as u64));       //display cur_gen/max_gen
    let flag_check_interval=param.max_gen.wrapping_div(5);
    for cur_gen in 1..=param.max_gen{
        gen_bar.inc(1);                                                //increment gen_bar
        let gen_ratio=cur_gen as f32/param.max_gen as f32;              //display   index/swarm_size
        let index_bar=muti_bar.add(ProgressBar::new(param.swarm_size as u64));

        for index in 0..param.swarm_size{
            let r=determine_r(&mut swarm.particles[index],cur_gen,param,&late_info,index==0,mode.model);
            debug_assert!(!r.is_nan(),"r is NaN");
            match mode.model {
                
                0=>{
                    classic_update_velocity(&mut swarm.particles[index], param, &swarm.gb,gen_ratio );
                }
                _=>{
                    sari_update_velocity(&mut swarm.particles[index], param, r,&swarm.gb,gen_ratio );
                }
            }
            
            update_place(&mut swarm.particles[index]);
            restrain_place(&mut swarm.particles[index], &obj.place_constrain);
            index_bar.inc(1);                                           //increment index_bar
            match mode.model {
                2=>swarm.async_update_obj(index, obj.fitness),
                _=>{}
            }
        }
        //update objective value,pb,gb
        match mode.model {
            2=>{}
            _=>swarm.update_obj(obj.fitness),
            
        }
        //swarm.update_obj(obj.fitness);
        swarm.update_search_angle(&mut search_angle_info);
        late_info.update_info(&swarm.particles, param.eval_range);

        index_bar.finish_and_clear();                                            //index_bar finished
        
        //whether to plot
        if cur_gen%flag_check_interval==0{
            if mode.plot_flag{
                let err = swarm.plot_figure(&obj.fitness_name, "figure", cur_gen);
                println!("{:?}",err);
            }
            
        }
        
    }
    gen_bar.finish_and_clear();                                                  //gen_bar finished
    if mode.plot_search_angle {
        let err=swarm.plot_search_angle(&search_angle_info, &obj.fitness_name, "angle_figure");
        println!("{:?}",err);
    }
    //choose what to output
    match mode.output {
        1=>Some(ReturnType::BestOne((swarm.gb,swarm.gb_val))),
        2=>Some(ReturnType::Swarm(swarm)),
        _=>Some(ReturnType::Nothing(())),

   }
}
/**
 Stores info of last min{eval_range,cur_gen-1} iterations
 In form of [[k_th info of iters;eval_range];r_num],i.e. rows are indexed by k,cols indexed by iteration.So one row has all info for one k
 specifically,stores `Opt_k use time` and `Opt_k update pb time` of each iteration with respect to each k(from 0 to r_num-1)
 Why each iteration ,not sum? To facilite sum new sum 
 */
#[derive(Debug)]
pub struct LateInfo{
    pub cur_iter:usize,//current iteration in the vec![0;eval_range],update in the last of the iteration
    pub opt_use_count:Vec<Vec<usize>>,          //update in `fn determine_r`
    pub opt_update_pb_count:Vec<Vec<usize>>,
}
impl LateInfo{
    /**
     By default,all info would be 0,since there's no iteration before the first and that 0 dosen't affect sum
     */
    pub fn new(r_num:usize,eval_range:usize)->Self{
        let opt_use_count:Vec<Vec<usize>>=vec![vec![0;eval_range];r_num];
        let opt_update_pb_count:Vec<Vec<usize>>=vec![vec![0;eval_range];r_num];
        LateInfo{
            cur_iter:0,
            opt_use_count,
            opt_update_pb_count,
        }
    }
    /**
     update late_info
     */
    pub fn update_info(&mut self,particles:&Vec<Particle>,eval_range:usize){
        let cur_iter=self.cur_iter;
        for particle in particles{
            let (k,flag)=particle.k_info;
            self.opt_use_count[k][cur_iter]+=1;
            self.opt_update_pb_count[k][cur_iter]+=flag as usize;
        }
        //change cur_iter for next iter
        self.cur_iter=(self.cur_iter+1)%eval_range;
    }
}