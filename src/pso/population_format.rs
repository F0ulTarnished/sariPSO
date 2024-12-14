use core::f32;
use std::{error::Error, f32::consts::PI};
use chrono::Local;
use rand::Rng;
use plotters::prelude::*;
/**
Each particle's place&velocity is vec,so pop_vec is 2-d vec
*/
#[derive(Debug)]
pub struct Swarm {
    pub swarm_size: usize,

    pub gb: Vec<f32>,
    pub gb_val: f32,
    pub particles: Box<Vec<Particle>>,
}

/**
Note:This struct can't be generate outside of `struct Population`
all items are vec
`k_info` is uesd to update `late_info`
*/
#[derive(Debug)]
pub struct Particle {
    pub cur_place: Vec<f32>,    //x(t)
    pub cur_velocity: Vec<f32>, //v(t)

    pub pb: Vec<f32>, //personal best place
    pub pb_val: f32,

    pub k_info:(usize,bool)//.0 update in `fn determin_r; .1 update in `fn update_obj`
}

impl Particle {
    #[inline]
    pub fn create_particle(
        d: usize,
        place_constrian: &Vec<(f32, f32)>, //in inform of (min,max)
    ) -> Particle {

        //generate Particle
        let mut rng = rand::thread_rng();
        let mut place: Vec<f32> = Vec::new();
        let mut velocity: Vec<f32> = Vec::new();
        for i in 0..d {
            let upper = place_constrian[i].1;
            let lower = place_constrian[i].0;
            let difference = (upper - lower) * 0.15; //speed will be constrianed within 12% of the range
            place.push(rng.gen_range(lower..upper));
            velocity.push(rng.gen_range(-difference..difference));
        }
        Particle {
            cur_place: place.clone(),
            cur_velocity: velocity,

            pb: place,
            pb_val: f32::MAX, //will be update before choose gb

            k_info:(0,false)
        }
    }
}
impl Swarm {
    /**
     Initialze a swarm
     @param d:dimension
     @param swarm_size:needless to say
     @param place_constrain:varible range with respect to each dim
     */
    pub fn new(
        d: usize,
        swarm_size: usize,
        place_constrain: &Vec<(f32,f32)>,
    ) -> Self {
        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..swarm_size {
            let new_particle=Particle::create_particle(d, place_constrain);
            particles.push(new_particle);
        }
        let gb=particles[0].cur_place.clone();
        Swarm{
            swarm_size:swarm_size,
            gb:gb,
            gb_val:f32::MAX,
            particles:Box::new(particles),
        }
    }
    /**
     update all particles' objective value in the swarm
     Note:not only change pb,but gb
     @param fitness:the fitness function
     */
    pub fn update_obj<F>(&mut self,fitness:F)
    where F:Fn(&Particle)->f32,{
        let particles=&mut self.particles;

        for i in 0..self.swarm_size{
            let obj_val=fitness(&particles[i]);
            //check if better than pb
            if obj_val<particles[i].pb_val{
                particles[i].pb=particles[i].cur_place.clone();
                particles[i].pb_val=obj_val;
                //update k_info
                particles[i].k_info.1=true;
                //check if better than gb
                if obj_val<self.gb_val{
                    self.gb=particles[i].cur_place.clone();
                    self.gb_val=obj_val;
                }
            }
        }
    }
    /**
    update objective of one particle,
    used for async update objectice
     */
    pub fn async_update_obj<F>(&mut self,index:usize,fitness:F)
    where F:Fn(&Particle)->f32,{
        let particle=&mut self.particles[index];
        let obj_val=fitness(particle);
        if obj_val<particle.pb_val{
            particle.pb=particle.cur_place.clone();
            particle.pb_val=obj_val;
            //update k_info
            particle.k_info.1=true;
            //check if better than gb
            if obj_val<self.gb_val{
                self.gb=particle.cur_place.clone();
                self.gb_val=obj_val;
            }
        }
    }
    pub fn plot_figure(&self,fitness_name:&str, output_path: &str,cur_gen:usize) -> Result<(), Box<dyn Error>> {
        let mut data:Vec<f32>=Vec::new();
        let mut min_value:f32=f32::MAX;
        let mut max_value:f32=f32::MIN;
        for k in 0..self.swarm_size{
            let val=self.particles[k].pb_val;
            let max_flag=val>max_value;
            let min_flag=val<min_value;
            min_value=(min_flag as i8 as f32)*val+(!min_flag as i8 as f32)*min_value ;
            max_value=(max_flag as i8 as f32)*val+(!max_flag as i8 as f32)*max_value ;
            data.push(val);
        }
        let time=Local::now().format("%Y%m%d_%H%M%S").to_string();
        let file_name=format!("{}/{}_gen_{}_{}.png",output_path,fitness_name,cur_gen,time);
        
        let root = BitMapBackend::new(&file_name, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
    
        
        let caption=format!("Scatter Plot Gen:{}",cur_gen);
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..data.len(), 0.0..max_value/2.0)?;
    
        chart.configure_mesh().draw()?;
    

        chart.draw_series(
            data.iter()
                .enumerate()
                .map(|(x, y)| Circle::new((x, *y), 5, RED.filled())),
        )?;
    
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(0, min_value), (data.len(), min_value)],
            &BLUE,
        )))?;
    
        Ok(())
    }
    /**
    To update search angle,which can evaluate ability of exporation 
    Later used in plot_search_angle,resolution in 2*pi/80 per bar
     */
    pub fn update_search_angle(&self,search_angle_info:&mut[usize;80]){
        //randomly choose tow dimension, no difference to select tow
        let d1=5;
        let d2=7;
        for particle in &*self.particles{
            let r_sin=particle.cur_place[d2];
            let r_cos=particle.cur_place[d1];
            let angle=f32::atan2(r_sin, r_cos);
            let index=((angle + PI) / (PI / 40.0)) as usize;
            if index!=80{
                search_angle_info[index]+=1;
            }
            else if index==80 {
                search_angle_info[0]+=1;
            }
            else {
                debug_assert!(index<80,"angle:{} out of range",angle);
            }
        }
    }
    /**
    Plot figure of seach angle of two dimension,statistic
     */
    pub fn plot_search_angle(&self,search_angle_info:&[usize;80],fitness_name:&str, output_path: &str)-> Result<(), Box<dyn std::error::Error>>{
        let time=Local::now().format("%Y%m%d_%H%M%S").to_string();
        let file_name=format!("{}/{}_{}.png",output_path,fitness_name,time);
        let root = BitMapBackend::new(&file_name, (800, 600)).into_drawing_area();
            root.fill(&WHITE)?;
        
        let y_max=search_angle_info.iter().max().unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("Search Angle Histogram", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..80 as usize, 0..*y_max)?;

        
        chart.configure_mesh()
            .x_labels(5) 
            .y_labels(10) 
            .x_desc("Angle")
            .y_desc("Frequency")
            .x_label_formatter(&|x| {
                
                //let pi:f32 = std::f32::consts::PI;
                match x {
                    0 => "0".to_string(),
                    20 => "π/2".to_string(),
                    40 => "π".to_string(),
                    60 => "3π/2".to_string(),
                    80 => "2π".to_string(),
                    _  => "".to_string(), 
            }
            })
            .draw()?;

        chart.draw_series(
            search_angle_info.iter().enumerate().map(|(i, &value)| {
                Rectangle::new(
                    [(i, 0), (i + 1, value)],
                    BLUE.filled(),
                )
            })
        )?;
    Ok(())
    }
}
