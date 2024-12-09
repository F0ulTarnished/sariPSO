use std:: sync::Mutex;
use rand::Rng;
use rand_distr::{Beta, Distribution, StandardNormal};
use super::{parameter_format::Param, population_format::Particle, sari_pso::LateInfo};
//----------------------------------------------------------------------------
/**
 This varible will be reused whin one iteration.Store in static to avoid redundant calculation
 Only calculate once in one iteration,when first particle is calculated.
 */
static CUMULATIVE_SUM:Mutex<Vec<f32>>=Mutex::new(Vec::new());
/**
 This function will determine r
 */
pub fn determine_r(particle: &mut Particle,cur_gen:usize,param:&Param,late_info:&LateInfo,first_flag:bool,model:u8)->f32{
    let mut rng=rand::thread_rng();
    let r_num=param.r_num;
    //if not the first gen
    if cur_gen!=1{
        if first_flag{
            //Count n_k & pn_k of the thesis,cal q_k
            let all_update_ratio=get_all_update_ratio(late_info, r_num);
            //cal prob_k
            let all_prob_k=get_all_prob_k(param.r_prob_min, &all_update_ratio);
            //cal cumulative sum of prob_k,and update it in static
            update_cumulative_sum(&all_prob_k);     
        }
        //determine k
        let r_0:f32;
        match model{
            3=>{
                if get_aggressive_flag(late_info, param.eval_range){
                    let beta=Beta::new(5.0, 2.0).unwrap();
                    r_0=beta.sample(&mut rng);
                }
                else {
                    r_0=rng.gen_range(0.0..=1.0);
                }
            }
            _=>{r_0=rng.gen_range(0.0..=1.0)}
        }
        let mut k= CUMULATIVE_SUM.lock().unwrap().binary_search_by(|&x| {
            if x > r_0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }).err().unwrap();
        debug_assert!(k!=r_num,"k:{},r_num:{}",k,r_num);
        k=k.clamp(0, r_num-1);
        //update k_info in 
        particle.k_info.0=k;
        //return
        k as f32/(r_num-1)as f32
    }
    else if cur_gen==1{
        let k =rng.gen_range(0..r_num);
        //update k_info in 
        particle.k_info.0=k;
        k as f32/(r_num-1)as f32
    }
    else {
        println!("Fail to generate correct r");
        f32::MAX
    }
}
fn get_aggressive_flag(late_info:&LateInfo,eval_range:usize)->bool{
    let mut pb_update_rec=vec![0;eval_range];
    let pb_update=&late_info.opt_update_pb_count;
    for i in 0..eval_range{
        for ele in pb_update{
            pb_update_rec[i]+=ele[i];
        }
    }
    let mut more_update_count=0;
    for i in 1..eval_range{
        let more_flag=pb_update_rec[i]>pb_update_rec[i-1];
        more_update_count+=more_flag as usize;
    }
    more_update_count>(eval_range/2)
}
/**
Returns the update ratio of k which is used in determining r,(k in 0..=r_num-1)  
*/
fn get_all_update_ratio(late_info:&LateInfo,r_num:usize)->Vec<f32>{
    let mut all_update_ratio:Vec<f32>=Vec::with_capacity(r_num);
    for k in 0..r_num {
        let use_sum:usize=late_info.opt_use_count[k].iter().sum();
        let update_sum:usize=late_info.opt_update_pb_count[k].iter().sum();
        let update_ratio=(update_sum as f32/use_sum as f32).max(0.0);
        debug_assert!(!update_ratio.is_nan(),"NaN found,update_sum:{},use_sum:{}",update_sum,use_sum);
        all_update_ratio.push(update_ratio);
    }
    all_update_ratio
}
/**
Returns the probability of k which is used in determining r,(k in 0..=r_num-1)  
param `pm` comes from `param:&Param`
*/
fn get_all_prob_k(pm:f32,all_update_ratio:&Vec<f32>)->Vec<f32>{
    let r_num=all_update_ratio.len();
    let mut all_prob_k:Vec<f32>=Vec::with_capacity(r_num);

    let sum_update_ratio:f32=all_update_ratio.iter().sum();
    for k in 0..r_num{
        let ratio_ratio=all_update_ratio[k]/sum_update_ratio;
        let prob_k=pm+(1.0-pm*r_num as f32)*ratio_ratio;
        all_prob_k.push(prob_k);
    }
    all_prob_k
    
}
#[inline(always)]
fn update_cumulative_sum(all_prob_k:&Vec<f32>){
    let mut cumulative_sum:Vec<f32>=all_prob_k.clone();
    for k in 1..cumulative_sum.len(){
        cumulative_sum[k]+=cumulative_sum[k-1];
    }
    *CUMULATIVE_SUM.lock().unwrap()=cumulative_sum;
}
//----------------------------------------------------------------------------
/**
 update velocity directly in sturct
 */
pub fn sari_update_velocity(particle:&mut Particle,param:&Param,r:f32,gb:&Vec<f32>,gen_ratio:f32){
    //prepare ellipsoid center and its norm
    let coe1=0.5*param.c1;
    let (x1,norm1)=get_ellipsoid_center(&particle.pb, &particle.cur_place, coe1);
    let coe2=0.5*param.c2;
    let (x2,norm2)=get_ellipsoid_center(gb, &particle.cur_place, coe2);
    //prepare dv, will be 0 if x is 0,since x=0 means the ellipsoid degrades into a point
    let dv1:Vec<f32>;
    let dv2:Vec<f32>;
    if norm1!=0.0{
    //get dv1
    let sub_dv1=get_sub_dv(particle, param, &x1, norm1);
    dv1=get_dv(r, &x1, &sub_dv1);
    }else {
        dv1=vec![0.0;param.d]
    }
    if norm2!=0.0{
    //get dv2
    let sub_dv2=get_sub_dv(particle, param, &x2, norm2);
    dv2=get_dv(r, &x2, &sub_dv2);
    }else {
        dv2=vec![0.0;param.d]
    }

    //new velocity
    let mut velocity:Vec<f32>=Vec::with_capacity(param.d);
    let omage=param.omage-gen_ratio*0.5;
    for k in 0..param.d{
        let ele=omage*particle.cur_velocity[k]+dv1[k]+dv2[k];
        velocity.push(ele);
        
    }
    particle.cur_velocity=velocity;
}
fn get_dv(r:f32,x:&Vec<f32>,sub_dv:&Vec<f32>)->Vec<f32>{
    //get x^T*x
    let x_product:f32=x.iter().map(|x|x*x).sum();
    //get x and sub_dv product
    let x_sub_dv:f32=x.iter().zip(sub_dv.iter()).map(|(x,y)|x*y).sum();
    //coe resulted by above
    let coe=(1.0-r)*x_sub_dv/x_product;
    debug_assert!(!coe.is_nan(),"NaN found,r:{},x_sub_dv:{},x_product:{}",r,x_sub_dv,x_product);
    //calculate
    let mut dv:Vec<f32>=Vec::with_capacity(sub_dv.len());
    for k in 0..sub_dv.len(){
        let ele=r*sub_dv[k]+coe*x[k];
        debug_assert!(!ele.is_nan(),"dv has NaN,r:{},sub_dv:{},coe:{},x:{}",r,sub_dv[k],coe,x[k]);
        dv.push(ele);
    }
    dv
}
#[inline]
fn get_sub_dv(_particle:&mut Particle,param:&Param,x:&Vec<f32>,norm:f32)->Vec<f32>{
    //prepare ri~U(0,1)
    let mut rng=rand::thread_rng();
    let r_coe:f32=rng.gen_range(0.0..=1.0);
    //prepare n1..nd~N(0,1) and its norm
    let mut norm_distributed:Vec<f32>=Vec::with_capacity(param.d);
    let mut norm_normal=0.0;
    for _k in 0..param.d{
        let ele:f32=StandardNormal.sample(&mut rand::thread_rng());
        norm_distributed.push(ele);
        norm_normal+=ele*ele;
    }
    norm_normal=norm_normal.sqrt();
    //calculate
    let mut sub_dv:Vec<f32>=Vec::with_capacity(param.d);
    let coe=norm*(r_coe.powf(1.0/param.d as f32))/norm_normal;
    for k in 0..param.d{
        let ele=x[k]+coe*norm_distributed[k];
        debug_assert!(!ele.is_nan(),"sub_dv has NaN");
        sub_dv.push(ele);
    }
    sub_dv

}
#[inline(always)]
fn get_ellipsoid_center(best:&Vec<f32>,cur_place:&Vec<f32>,coe:f32)->(Vec<f32>,f32){
    let len=cur_place.len();
    let mut x:Vec<f32>=Vec::with_capacity(len);
    let mut norm=0.0;
    for k in 0..len{
        let scalar=coe*(best[k]-cur_place[k]);
        norm+=scalar*scalar;
        x.push(scalar);
    }
    norm=norm.sqrt();
    (x,norm)
}

//----------------------------------------------------------------------------
/**
 update place directly in struct
 */
#[inline(always)]
pub fn update_place(particle:&mut Particle){
    for k in 0..particle.cur_place.len(){
        particle.cur_place[k]+=particle.cur_velocity[k];
    }
}
//----------------------------------------------------------------------------
/**
 restrain particle's place within constrain,reflect method
 */
pub fn restrain_place(particle: &mut Particle, place_constrain: &[(f32, f32)]) {
    for ((place, velocity), &(min, max)) in particle
        .cur_place
        .iter_mut()
        .zip(particle.cur_velocity.iter_mut())
        .zip(place_constrain.iter())
    {
        // flag of overbound
        let under_min = min - *place;
        let over_max = *place - max;

        // reflect if overbound
        *place += under_min * (under_min > 0.0) as i32 as f32
            - over_max * (over_max > 0.0) as i32 as f32;
        *velocity *= 1.0 - 1.1 * ((under_min > 0.0) as i32 as f32 + (over_max > 0.0) as i32 as f32);
        //trivial restrain on velocity
        *velocity=velocity.clamp(min*0.15, max*0.15);
    }
}
//----------------------------------------------------------------------------
/**
save swarm
 */
//----------------------------------------------------------------------------
pub fn classic_update_velocity(particle:&mut Particle,param:&Param,gb:&Vec<f32>,gen_ratio:f32){
    let mut rng=rand::thread_rng();
    //prepare dv
    let mut dv1:Vec<f32>=Vec::new();
    let mut dv2:Vec<f32>=Vec::new();
    for k in 0..param.d{
        let r1=rng.gen_range(0.0..=1.0);
        let r2=rng.gen_range(0.0..=1.0);
        let ele1=param.c1*r1*(particle.pb[k]-particle.cur_place[k]);
        let ele2=param.c2*r2*(gb[k]-particle.cur_place[k]);
        dv1.push(ele1);
        dv2.push(ele2);
    }
    //get velocity
    let coe=param.omage-gen_ratio*0.5;
    let mut v:Vec<f32>=Vec::new();
    for k in 0..param.d{
        let ele=particle.cur_velocity[k]*coe+dv1[k]+dv2[k];
        v.push(ele);
    }
    particle.cur_velocity=v;
    
}
