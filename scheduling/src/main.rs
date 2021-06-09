#![crate_name = "scheduling"]

mod modules;

use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];
    let mut text_file = TextFile::new(arg.to_string());
    text_file.execute();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

use std::io::{self, BufRead};
use std::collections::HashMap;

/// A TextFile object.
pub struct TextFile {
    /// This is the name of the textfile containing the jobshop.
    file_name: String,
    /// Vector containing multiple Job objects
    current_jobs: Vec<modules::task::Job>,
    /// Vector containing multiple task objects
    current_tasks: Vec<modules::task::Task>,
    /// A vector with shorts
    current_getal: Vec<u16>,
    /// Short for which job is in progress
    current_job_progress: u16,
    /// short for how many jobs the jobshop has
    current_number_of_jobs: u16,
    /// short for how many machines the jobshop has
    current_number_of_machines: u16,
    /// short for storing the id of the machine that is currently busy.
    current_machine_id: u16,
    /// bool storing if the job_shop is scheduling
    making_job_shop: bool,
}

impl TextFile {
    /// Returns a TextFile object with the filename given them
    ///
    /// # Arguments
    ///
    /// * `file_name` - A string slice that holds the name of file the program reads from
    ///
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            current_jobs: Vec::with_capacity(50),
            current_tasks: Vec::with_capacity(50),
            current_getal: Vec::with_capacity(4),
            current_job_progress: 0,
            current_number_of_jobs: 0,
            current_number_of_machines: 0,
            current_machine_id: 0,
            making_job_shop: false,
        }
    }

    /// Reads all info from textFile and fills and executes a jobshop
    pub fn execute(&mut self) {
        if let Ok(lines) = TextFile::read_lines(&self.file_name) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    let mut i;
                    let char_vec: Vec<char> = ip.chars().collect();
                    for character in 0..char_vec.len()
                    {
                        i = &char_vec[character];
                        if !self.check_for_tasks(character as u16, char_vec.clone())
                        {
                            break;
                        }
                        //Check if jobshop is being filled or checked
                        if !self.making_job_shop
                        {
                            self.check_for_new_jobshop(i);
                        }
                        else
                        {
                            self.fill_jobshop(i);
                        }
                    }
                    self.change_jobshop_progress();
                }
            }
        }
    }

    /// Function used for reading a line from a textfile
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    /// Returns a boolean. true if there are still unread tasks
    ///
    /// # Arguments
    ///
    /// * `index` - A index for checking where you are
    /// * `line` - A vector containing characters from a textfile
    ///
   pub fn check_for_tasks(&self, index: u16,  line: Vec<char>) -> bool{
       let mut tasks_remaining = false;
       let mut character;

       for i in (index..(line.len() as u16)).rev(){
           character = line[i as usize];
           if character.is_digit(10){
               tasks_remaining = true;
               break;
           }
       }
       return tasks_remaining;
    }

    /// Returns a short. Calculates which number is read from the file
    pub fn calculate_current_getal(&mut self) -> u16{
        let mut factor = 1;
        let mut getal = 0;

        for i in (0..(self.current_getal.len())).rev(){
            if factor == 0{
                getal += self.current_getal[i];
            }else{
                getal += self.current_getal[i] * factor;
            }
            factor = factor * 10;
        }
        self.current_getal.clear();
        return getal;
    }

    /// Returns a boolean. true if the character is a whitespace
    ///
    /// # Arguments
    ///
    /// * `i` - A character that is either a whitespace or a number
    ///
    pub fn check_white_space(&mut self, i: &char) -> bool{
        if i.is_digit(10){
            self.current_getal.push(*i as u16 - 48);
        }else if self.current_getal.len() > 0 &&
        i.is_ascii_whitespace(){
            return true;
        }
        return false;
    }

    /// checks if a new jobshop is starting
    ///
    /// # Arguments

    /// * `i` - A character from a line
    ///
    pub fn check_for_new_jobshop(&mut self, i: &char){
        if self.check_white_space(i) && self.current_number_of_jobs == 0{
            self.current_number_of_jobs = self.calculate_current_getal();
        }
    }

    /// fills a variables needed for a new jobshop object
    ///
    /// # Arguments
    ///
    /// * `i` - A character from a line
    pub fn fill_jobshop(&mut self, i: &char){
        if self.check_white_space(i){
            if self.current_machine_id == self.current_number_of_machines{
                self.current_machine_id = self.calculate_current_getal();
            }else{
                let getal = self.calculate_current_getal();
                self.current_tasks.push(modules::task::Task::new(self.current_machine_id, getal));
                self.current_machine_id = self. current_number_of_machines;
            }
        }
    }

    /// fills a jobshop or creates one and starts scheduling.
    pub fn change_jobshop_progress(&mut self){
        if !self.making_job_shop && self.current_number_of_jobs != 0{
            self.current_number_of_machines = self.calculate_current_getal();
            self.current_machine_id = self.current_number_of_machines;
            self.making_job_shop = true;
        }else if self.making_job_shop{
            let getal = self.calculate_current_getal();
            self.current_tasks.push(modules::task::Task::new(self.current_machine_id, getal));
            self.current_machine_id = self.current_number_of_machines;
            self.current_jobs.push(modules::task::Job::new(self.current_job_progress,self.current_tasks.clone()));
            self.current_tasks.clear();
            self.current_job_progress +=1;

            if self.current_job_progress == self.current_number_of_jobs{
                let mut jb = JobShop::new(self.current_jobs.clone(), self.current_number_of_machines);
                jb.schedule();
                println!();
                
                self.current_jobs.clear();
                self.current_number_of_jobs = 0;
                self.current_number_of_machines = 0;
                self.current_job_progress = 0;
                self. making_job_shop = false;
            }
        }
    }
}

/// A jobshop object containing jobs
pub struct JobShop{
    jobs : Vec<modules::task::Job>,
    machine_status: HashMap< u16, u16>,
	maximum_duration: u16,
}

impl JobShop{
    
    /// Returns a new Jobshop object
    ///
    /// # Arguments
    ///
    /// * `some_jobs` - a vector containing the jobs for the jobshop
    /// * `number_of_machines` - the number of machines the jobshop has
    ///
    pub fn new(some_jobs: Vec<modules::task::Job>, number_of_machines: u16) -> Self{
        let mut temp_map = HashMap::new();
        for x in 0..(number_of_machines){
            temp_map.insert(x, some_jobs.len() as u16);
        }

        let mut temp_object = JobShop {
            jobs : some_jobs,
            machine_status: temp_map,
            maximum_duration: 0,
        };
        temp_object.change_latest_start_times();
        return temp_object;
    }

    /// starts scheduling
    pub fn schedule(&mut self){
        let mut current_time = 0;
        while self.check_all_jobs_completed() == false{
            self.calculate_progress(current_time);
            current_time += 1;
        }
        self.print_output();
    }

    /// Calculates the progress based on the time
    ///
    /// # Arguments
    ///
    /// * `current_time` - A number indicating the time
    pub fn calculate_progress(&mut self, current_time: u16){
        for machine_index in 0..self.machine_status.len(){
            if self.machine_status[&(machine_index as u16)] != (self.jobs.len() as u16){
                let machine_job_id = self.machine_status[&(machine_index as u16)];
                self.jobs[machine_job_id as usize].get_first_open_task_mut().up_current_progress();

                if self.jobs[machine_job_id as usize].get_first_open_task().get_current_progress() > 
                self.jobs[machine_job_id as usize].get_first_open_task().get_duration(){

                    self.jobs[machine_job_id as usize].get_first_open_task_mut().set_task_completed();

                    self.recalculate_total_durations(current_time);
                    self.change_latest_start_times();

                    self.set_machine_status(machine_index as u16, self.jobs.len() as u16);

                    if self.jobs[machine_job_id as usize].get_first_open_task() == self.jobs[machine_job_id as usize].get_end_task(){
                        self.jobs[machine_job_id as usize].set_end_time(current_time);
                    }
                }
            }
        }

        for machine_index in 0..self.machine_status.len(){
            if self.machine_status[&(machine_index as u16)] == (self.jobs.len() as u16){
                let mut current_slack = self.maximum_duration + 1;
                let mut current_job_id = self.jobs.len() as u16;
                
                for j in &self.jobs{
                    if j.get_first_open_task() != j.get_end_task() &&
                    j.get_first_open_task().get_machine_id() == machine_index as u16 &&
                    j.get_first_open_task().get_latest_start_time() - current_time < current_slack{
                        current_job_id = j.get_job_id();
                        current_slack = j.get_first_open_task().get_latest_start_time() - current_time;
                    }
                }

                if current_job_id != self.jobs.len() as u16{
                    self.set_machine_status(machine_index as u16, current_job_id);
                    
                    self.jobs[current_job_id as usize].get_first_open_task_mut().up_current_progress();
                    if self.jobs[current_job_id as usize].get_first_open_task() == self.jobs[current_job_id as usize].get_first_task(){
                        self.jobs[current_job_id as usize].set_start_time(current_time);
                    }
                }
            }

        }
    }

    /// Recalculates the duration
    ///
    /// # Arguments
    ///
    /// * `current_time` - A number indicating the time
    ///
    pub fn recalculate_total_durations(&mut self, current_time: u16){
        for j in &mut self.jobs{
            j.calculate_total_duration(current_time);
        }
    }

    /// Recalculates and sets the start times
    pub fn change_latest_start_times(&mut self){
        let old_maximum_duration =  self.maximum_duration;

        for j in &self.jobs{
            if j.get_first_open_task() != j.get_end_task() &&
            j.get_total_duration() > self.maximum_duration{
                self.maximum_duration = j.get_total_duration();
            }
        }

        if self.maximum_duration > old_maximum_duration{
            for j in &mut self.jobs{
                if j.get_first_open_task() != j.get_end_task(){
                    j.set_slack((self.maximum_duration - j.get_total_duration()) as i16);
                    j.set_latest_start_times();
                }
            }
        }
    }

    /// Prints the job ids, start times and end times
    pub fn print_output(&self){
        for j in &self.jobs{
            println!("{0} {1} {2}", j.get_job_id(), j.get_start_time(), j.get_end_time());
        }  
    }

    /// returns true if all the jobs are completed
    pub fn check_all_jobs_completed(&self) -> bool{
        for j in &self.jobs{
            if j.get_first_open_task() != j.get_end_task(){
                return false;
            }
        }
        return true;
    }

    /// sets current status of a machine
    ///
    /// # Arguments
    ///
    /// * `machine_id` - A id for the machine that needs to be changed
    /// * `job_id` - A id for the job that is busy
    ///
    pub fn set_machine_status(&mut self, machine_id: u16, job_id: u16){
        self.machine_status.insert(machine_id, job_id);
    }
}
