mod modules;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];
    let text_file = TextFile::new(arg.to_string());
    //text_file.execute();
}

use std::io::{self, BufRead};

pub struct TextFile {
    file_name: String,
    //currentJobs: Vec<Job>,
    currentTasks: Vec<modules::task::Task>,
    current_getal: Vec<u16>,
    current_job_progress: u16,
    current_number_of_jobs: u16,
    current_number_of_machines: u16,
    current_machine_id: u16,
    making_job_shop: bool,
}

impl TextFile {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            //currentJobs: Vec::with_capacity(50),
            currentTasks: Vec::with_capacity(50),
            current_getal: Vec::with_capacity(4),
            current_job_progress: 0,
            current_number_of_jobs: 0,
            current_number_of_machines: 0,
            current_machine_id: 0,
            making_job_shop: false,
        }
    }

//     pub fn execute(&self) {
//         if let Ok(lines) = Self::read_lines(&self.file_name) {
//             // Consumes the iterator, returns an (Optional) String
//             for line in lines {
//                 if let Ok(ip) = line {
//                     let i;
//                     let char_vec: Vec<char> = ip.chars().collect();
//                     for character in 0..char_vec.len()
//                     {
//                         i = char_vec[character];
//                         if (!checkForTasks(character, char_vec))
//                         {
//                             break;
//                         }
//                         //Checken of er een JobShop wordt gevuld of gezocht
//                         if (!makingJobShop)
//                         {
//                             checkForNewJobShop(static_cast<unsigned short>(i));
//                         }
//                         else
//                         {
//                             fillJobShop(static_cast<unsigned short>(i));
//                         }
//                     }
//                     changeJobShopProgress();
//                 }
//             }
//         }
//     }

//     fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
//     where
//         P: AsRef<std::path::Path>,
//     {
//         let file = std::fs::File::open(filename)?;
//         Ok(io::BufReader::new(file).lines())
//     }

//    fn checkForTasks(index: u16,  line: Vec<char>) -> bool
// {
// 	let tasks_remaining = false;
// 	std::string character;
// 	for (signed short i = line.size() - 1; i >= index; --i)
// 	{
// 		character = line.at(i);
// 		if (std::regex_search(character, std::regex("\\d+")))
// 		{
// 			tasksRemaining = true;
// 			break;
// 		}
// 	}
// 	return tasksRemaining;
// }
}
