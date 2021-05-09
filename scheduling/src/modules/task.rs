#[derive(PartialEq, Eq)]
pub struct Task {
    machine_id: u16,
    duration: u16,
    current_progress: u16,
    task_completed: bool,
    latest_start_time: u16,
}

impl Task {
    pub fn new(a_machine_id: u16, a_duration: u16) -> Self {
        Self {
            machine_id: a_machine_id,
            duration: a_duration,
            current_progress: 0,
            task_completed: false,
            latest_start_time: 0,
        }
    }

    pub fn get_machine_id(&self) -> &u16 {
        &self.machine_id
    }

    pub fn get_duration(&self) -> &u16 {
        &self.duration
    }

    pub fn get_current_progress(&self) -> &u16 {
        &self.current_progress
    }

    pub fn up_current_progress(&mut self) {
        self.current_progress = self.current_progress + 1;
    }

    pub fn is_task_completed(&self) -> &bool {
        &self.task_completed
    }

    pub fn set_task_completed(&mut self, new_task_completed: bool) {
        self.task_completed = new_task_completed;
    }

    pub fn get_latest_start_time(&self) -> &u16 {
        &self.latest_start_time
    }

    pub fn set_latest_start_time(&mut self, new_latest_start_time: u16) {
        self.latest_start_time = new_latest_start_time;
    }
}
