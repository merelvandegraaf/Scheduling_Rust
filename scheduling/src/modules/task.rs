#[derive(PartialEq, Clone, Copy, Eq)]
pub struct Task {
    machine_id: u16,
    duration: u16,
    current_progress: u16,
    pub task_completed: bool,
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

    pub fn get_machine_id(&self) -> u16 {
        self.machine_id
    }

    pub fn get_duration(&self) -> u16 {
        self.duration
    }

    pub fn get_current_progress(&self) -> u16 {
        self.current_progress
    }

    pub fn up_current_progress(&mut self) {
        self.current_progress = self.current_progress + 1;
    }

    pub fn is_task_completed(&self) -> bool {
        self.task_completed
    }

    pub fn set_task_completed(&mut self) {
        //println!("before1 {}",self.is_task_completed());
        self.task_completed = true;
        //println!("after1 {}",self.is_task_completed());
    }

    pub fn get_latest_start_time(&self) -> u16 {
        self.latest_start_time
    }

    pub fn set_latest_start_time(&mut self, new_latest_start_time: u16) {
        self.latest_start_time = new_latest_start_time;
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Job{
    job_id: u16,
    tasks: Vec<Task>,
    slack: i16,
    total_duration: u16,
    start_time: u16,
    end_time: u16,
}

impl Job{
    pub fn new(a_job_id: u16, some_tasks: Vec<Task>) -> Self {
        let mut temp_total_duration = 0;
        for t in &some_tasks{
            temp_total_duration += t.get_duration();
        }
        Self {
        job_id: a_job_id,
        tasks: some_tasks,
        slack: 0,
        total_duration: temp_total_duration,
        start_time: 0,
        end_time: 0,
        }
    }

    pub fn get_first_open_task(&self) -> &Task {
        let index = self.tasks.iter().position(|&r| !r.is_task_completed()).unwrap();
        return &self.tasks[index];
    }

    pub fn get_first_open_task_mut(&mut self) -> &mut Task {
        let index = self.tasks.iter().position(|&r| r.is_task_completed() == false).unwrap();
        return &mut self.tasks[index];
    }

    pub fn set_latest_start_times(&mut self){
        let mut sub_total = self.total_duration;
	    for t in &self.tasks
	    {
		    if !t.is_task_completed()
		    {
			    sub_total -= t.get_duration();
		    }
	    }
	    for  t in &mut self.tasks
	    {
		    if !t.is_task_completed()
		    {
			    t.set_latest_start_time(sub_total + (self.slack as u16));
			    sub_total += t.get_duration();
		    }
	    }
    }

    pub fn calculate_total_duration(&mut self, current_time: u16){
        if self.get_first_open_task().get_latest_start_time() < current_time
	    {
		    //Reset de totalDuration.
		    self.total_duration = 0;
		    for t in &mut self.tasks
		    {
			    self.total_duration += t.get_duration();
		    }
		    //Bereken de tijd van de tasks die klaar zijn.
		    let mut worked_time = 0;
		    for t in &self.tasks
		    {
			    if t.is_task_completed()
			    {
				    worked_time += t.get_duration();
			    }
		    }
		    //Verhoogt de totalDuration met de tijd die extra nodig was om de voltooide tasks af te krijgen
		    if current_time >= worked_time
		    {
		    	self.total_duration += current_time - worked_time;
	    	}
	    }
    }

    pub fn get_end_task(&self) -> &Task{
        return &self.tasks.last().unwrap();
    }

    pub fn get_job_id(&self) -> u16{
        return self.job_id;
    }

    pub fn set_job_id(&mut self, a_job_id: u16){
        self.job_id = a_job_id;
    }

    pub fn get_slack(&self) -> i16{
        return self.slack;
    }

    pub fn set_slack(&mut self, a_slack: i16){
        self.slack = a_slack;
    }

    pub fn get_total_duration(&self) -> u16{
        return self.total_duration;
    }

    pub fn get_start_time(&self) -> u16{
        return self.start_time;
    }

    pub fn set_start_time(&mut self, a_start_time: u16){
        self.start_time = a_start_time;
    }

    pub fn get_end_time(&self) -> u16{
        return self.end_time;
    }

    pub fn set_end_time(&mut self, a_end_time: u16){
        self.end_time = a_end_time;
    }

    pub fn get_first_task(&self) -> &Task{
        return &self.tasks[0];
    }

    pub fn get_tasks(&self) -> &Vec<Task>{
        return &self.tasks;
    }
}
