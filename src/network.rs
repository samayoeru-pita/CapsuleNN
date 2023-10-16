pub mod capsule{
    pub struct Capsule{
        value: Vec<f32>
    }
}
pub mod network{
    use super::capsule::Capsule;
    pub struct Network{
    }
    impl Network {
        pub fn new() -> Network{
            let capsules: Vec<Capsule> = vec![];
            Network{
            }
        }
    }
}