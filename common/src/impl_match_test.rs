#[derive(PartialEq, Debug)]
struct ExecuteJob(u64);

#[derive(PartialEq, Debug)]
enum Alert {
    Warnning,
    Error,
    Other,
}

impl ExecuteJob {
    fn new(status: i64) -> Result<ExecuteJob, Alert> {
        if status > 0 {
            Ok(ExecuteJob(status as u64))
        } else {
            match status {
                -1 => Err(Alert::Warnning),
                -2 => Err(Alert::Error),
                _ => Err(Alert::Other),
            }
        }
    }
}

#[test]
fn test_creation() {
    assert!(ExecuteJob::new(1).is_ok());
    assert_eq!(Err(Alert::Warnning), ExecuteJob::new(-1));
    assert_eq!(Err(Alert::Error), ExecuteJob::new(-2));
}

