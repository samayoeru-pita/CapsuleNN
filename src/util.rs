pub mod util{
    pub fn dot(v1: &Vec<f32>, v2: &Vec<f32>) -> f32{
        let mut result: f32 = 0.0;
        assert!(v1.len() == v2.len(), "Vectors have different sizes.");
        for i in 0..v1.len(){
            result += v1[i] * v2[i];
        }
        result
    }
    pub fn length_sq(vec: &Vec<f32>) -> f32{
        let mut result: f32 = 0.0;
        for i in 0..vec.len(){
            result += vec[i] * vec[i];
        }
        result
    }
    pub fn length(vec: &Vec<f32>) -> f32{
        let mut result: f32 = 0.0;
        for i in 0..vec.len(){
            result += vec[i] * vec[i];
        }
        result.sqrt()
    }
}