#[derive(Debug)]
pub struct Classification;

impl Classification {
    pub fn new() -> Self {
        Classification
    }

    pub fn classify(&self, pm25: f64, pm10: f64) -> String {
        // Thresholds based on general air quality standards
        if pm25 <= 50.0 && pm10 <= 100.0 {
            "Good".to_string()
        } else if pm25 <= 100.0 && pm10 <= 150.0 {
            "Moderate".to_string()
        } else {
            "Poor".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification() {
        let classifier = Classification::new();
        assert_eq!(classifier.classify(30.0, 50.0), "Good");
        assert_eq!(classifier.classify(75.0, 120.0), "Moderate");
        assert_eq!(classifier.classify(150.0, 200.0), "Poor");
    }
}
