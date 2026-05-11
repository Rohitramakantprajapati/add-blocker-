use ai_engine::features::FeatureVector;
use ai_engine::runner::Runner;

#[test]
fn precision_exceeds_threshold() {
    let runner = match Runner::load(std::path::Path::new("does-not-exist.json")) {
        Ok(runner) => runner,
        Err(error) => {
            assert!(false, "runner should initialize: {error}");
            return;
        }
    };

    let dataset = [
        (FeatureVector::from_observations("https://ads.example.com/banner.js", 2, 8300, 12.0), true),
        (FeatureVector::from_observations("https://cdn.example.com/app.js", 0, 1200, 4.0), false),
        (FeatureVector::from_observations("https://track.doubleclick.net/pixel", 3, 6500, 20.0), true),
        (FeatureVector::from_observations("https://example.com/index.html", 0, 900, 3.0), false),
        (FeatureVector::from_observations("https://analytics.example.net/beacon", 2, 4200, 14.0), true),
        (FeatureVector::from_observations("https://static.example.org/style.css", 0, 700, 2.0), false),
    ];

    let mut true_positive = 0usize;
    let mut false_positive = 0usize;

    for (features, label) in dataset {
        let predicted = match runner.is_blocked(&features) {
            Ok(predicted) => predicted,
            Err(_) => false,
        };
        if predicted && label {
            true_positive += 1;
        }
        if predicted && !label {
            false_positive += 1;
        }
    }

    let precision = true_positive as f32 / (true_positive + false_positive).max(1) as f32;
    assert!(precision > 0.92, "precision should exceed threshold, got {precision:.3}");
}
