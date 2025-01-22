#[cfg(test)]
mod tests {
    use numerics_rs::root_finding::ConvergenceLog;

    #[test]
    fn test_add_and_display_log() {
        let mut log = ConvergenceLog::new();

        // Add some example iterations for testing
        log.add_entry(
            0,
            Box::from(vec![1.0, 2.0, 1.5]),
            Box::from(vec![2.0, -1.0, 0.5]),
        );
        log.add_entry(1, Box::from(vec![1.5, 2.0]), Box::from(vec![0.5, -0.25]));

        // Display the log
        log.display_log();

        // Assertions to verify the log contents
        assert_eq!(log.get_entries().len(), 2);
        assert_eq!(log.get_entries()[0].x.len(), 3);
        assert_eq!(log.get_entries()[0].x[2], 1.5);
        assert_eq!(log.get_entries()[1].fx[1], -0.25);
    }

    #[test]
    fn test_reset_log() {
        let mut log = ConvergenceLog::new();
        log.add_entry(
            0,
            Box::from(vec![1.0, 2.0, 1.5]),
            Box::from(vec![2.0, -1.0, 0.5]),
        );

        log.reset(); // Clear the log
        assert!(log.get_entries().is_empty());
    }
}
