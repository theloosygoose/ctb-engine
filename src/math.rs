pub trait Sortable {
    fn sort(&mut self);
    
    fn rev_sort(&mut self);
}

pub fn mean(data: &Vec<f32>) -> f32 {
    
    let sum = data.iter().sum::<f32>() as f32;
    
    let count = data.len();

    sum / count as f32
}

pub fn std_dev(data: &Vec<f32>) -> f32 {
    let data_mean = mean(data);
    let count = data.len();

    let variance = data.iter().map(|val| {
        let diff = data_mean - (*val as f32);

        diff*diff
    })
    .sum::<f32>() / count as f32;

    variance.sqrt()
}

pub fn z_score(data: &Vec<f32>, data_std_dev: f32, data_mean: f32) -> Vec<f32> {
    let mut z_scores: Vec<f32> = vec![];
    
    data.iter().for_each(|val| {
        let z_score = (val - data_mean) / data_std_dev;

        z_scores.push(z_score);
    });
    
    z_scores
}
