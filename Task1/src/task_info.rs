pub struct TaskInfo {
    task_number: usize,
    method_number: usize,
    start_section: f64,
    end_section: f64,
    number_of_partitions: usize,
}

impl TaskInfo {
    pub fn new(
        task_number: usize,
        method_number: usize,
        start_section: f64,
        end_section: f64,
        number_of_partitions: usize,
    ) -> Self {
        TaskInfo {
            task_number,
            method_number,
            start_section,
            end_section,
            number_of_partitions,
        }
    }
}