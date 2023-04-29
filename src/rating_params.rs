
#[derive(Debug, Clone, Copy)]
pub struct RatingParams {
    pub mu: f64,
    pub phi: f64,
    pub sigma: f64,
    pub tau: f64,
    pub is_scaled: bool,
}
impl RatingParams {
    pub fn new(mu: f64, phi: f64, sigma: f64, tau: f64, is_scaled: bool) -> RatingParams {
        RatingParams {
            mu,
            phi,
            sigma,
            tau,
            is_scaled,
        }
    }

    pub fn default() -> RatingParams {
        RatingParams {
            mu: 1500.0,
            phi: 350.0,
            sigma: 0.006,
            tau: 1.3,
            is_scaled: false,
        }
    }

    pub fn update(&mut self, mu: f64, phi: f64, sigma: f64, is_scaled: bool) {
        self.mu = mu;
        self.phi = phi;
        self.sigma = sigma;
        self.is_scaled = is_scaled;
    }
}