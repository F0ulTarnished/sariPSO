use crate::pso::population_format::Particle;
#[allow(dead_code)]
pub fn rosenbrock(particle:&Particle)->f32{
    let mut sum = 0.0;
    let x=&particle.cur_place;
    let d = x.len();
    
    // Compute the Rosenbrock function value
    for i in 0..d - 1 {
        let xi = x[i];
        let xi1 = x[i + 1];
        let xi_square=xi.powi(2);
        sum += 100.0 * (xi1 - xi_square).powi(2) + (1.0-xi-xi+xi_square);
    }
    sum
}

