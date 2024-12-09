use super::population_format::Particle;

pub struct Objective{
    //restrain particle's place
    //velocity constrain is 12% of this range,so not more definition
    pub place_constrain:Vec<(f32,f32)>,

    //this function should generate fitness only for one particle
    pub fitness: fn(&Particle) -> f32,
    //used to name file
    pub fitness_name:String,
}
