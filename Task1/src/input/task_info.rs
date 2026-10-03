pub struct TaskInfo {
    pub task_number: usize,
    pub method_number: usize,
    pub start_section: f64,
    pub end_section: f64,
}

impl TaskInfo {
    pub fn new(
        task_number: usize,
        method_number: usize,
        start_section: f64,
        end_section: f64,
    ) -> Self {
        TaskInfo {
            task_number,
            method_number,
            start_section,
            end_section,
        }
    }
}
