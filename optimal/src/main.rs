use activity::*;
use std::collections::VecDeque;

// *********************************************************
// Main application
// *********************************************************
fn main() {
    let mut is_last_input_valid: bool;
    let mut system: System;
  
    is_last_input_valid = false;    
    system = build_system_and_virtual_memory_structure(&mut is_last_input_valid);

    system.handle_requests();

    system.display_cumulative_snapshots();

    display_header();

    display_title("Summary\n");
    system.display_snapshots_summary();

    display_divider(1);

    display_title("Conclusion\n");
    system.display_conclusion();

    display_footer();
}

// *********************************************************
// Subroutines
// *********************************************************
fn build_system_and_virtual_memory_structure(is_last_input_valid: &mut bool) -> System {
    let virtual_memory: VirtualMemory;
    let frames: Vec<Frame>;
    let mut pages: Vec<Page>;
    let requests: VecDeque<Request>;

    display_header();
    frames = input_frames(is_last_input_valid);
    pages = input_pages(is_last_input_valid);
    display_footer();
  
    display_header();
    requests = input_requests(is_last_input_valid, &mut pages);
    display_footer();
  
    virtual_memory = VirtualMemory::new(frames);

    return System::new(virtual_memory, pages, requests);
}

fn input_frames(is_last_input_valid: &mut bool) -> Vec<Frame> {
    let frames_count: usize;
    let mut frames: Vec<Frame>;

    frames = Vec::new();

    loop {
        let frames_count_input: String;
      
        frames_count_input = display_prompt("Enter", "number of frames");

        match frames_count_input.trim().parse::<isize>() {
            Ok(frames_count_value) => {
                if frames_count_value <= 0 {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error(&format!(
                        "Number of frames cannot be less than or equal to zero.\n"
                    ));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                frames_count = frames_count_value as usize;
                break;
            },
            Err(error) => {
                display_info("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }

    for i in 0..frames_count {
        frames.push(Frame::new(i + 1, format!("Frame {}", i + 1)));
    }

    return frames;
}

fn input_pages(is_last_input_valid: &mut bool) -> Vec<Page> {
    let pages_count: usize;
    let mut pages: Vec<Page>;

    pages = Vec::new();

    loop {
        let pages_count_input: String;
        pages_count_input = display_prompt("Enter", "number of pages");

        match pages_count_input.trim().parse::<isize>() {
            Ok(pages_count_value) => {
                if pages_count_value <= 0 {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error(&format!(
                        "Number of pages cannot be less than or equal to zero.\n"
                    ));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                pages_count = pages_count_value as usize;
                break;
            },
            Err(error) => {
                display_info("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }

    for i in 0..pages_count {
        pages.push(Page::new(i + 1, decimal_to_letteral(i + 1)));
    }

    return pages;
}

fn input_requests(is_last_input_valid: &mut bool, pages: &mut Vec<Page>) -> VecDeque<Request> {
    let requests_count: usize;
    let mut requests: VecDeque<Request>;
    let stringified_pages: String;

    requests = VecDeque::new();
    stringified_pages = Page::stringify_pages(pages);

    loop {
        let requests_count_input: String;
        requests_count_input = display_prompt("Enter", "number of requests");

        match requests_count_input.trim().parse::<isize>() {
            Ok(requests_count_value) => {
                if requests_count_value <= 0 {
                    display_info("Please enter a valid positive integer value.\n");
                    display_error(&format!(
                        "Number of requests cannot be less than or equal to zero.\n"
                    ));

                    terminal_cursor_previous_line(3);

                    *is_last_input_valid = false;
                    continue;
                }

                if !*is_last_input_valid {
                    terminal_clear_next_lines(2);
                    terminal_cursor_previous_line(1);
                }

                *is_last_input_valid = true;
                requests_count = requests_count_value as usize;
                break;
            }
            Err(error) => {
                display_info("Please enter a valid positive integer value.\n");
                display_error(&format!("Failed to parse number: {}.\n", error));

                terminal_cursor_previous_line(3);

                *is_last_input_valid = false;
                continue;
            }
        }
    }

    pages.sort_by_key(| page | page.name.clone());

    for i in 0..requests_count {
        let request_name: String;
      
        loop {
            let request_name_input: String;

            display("\n");
            display_labelled("Available Pages", &stringified_pages);
            terminal_cursor_previous_line(2);
            request_name_input = display_prompt(
                "Enter",
                &format!(
                    "page requested of {}",
                    text_apply_style(&format!("Request {}", i + 1), "fg-yellow")
                ),
            );

            if request_name_input.trim() == "-" {
                if !*is_last_input_valid {
                    terminal_clear_next_lines(3);
                    terminal_cursor_previous_line(2);
                }

                *is_last_input_valid = true;
                
                requests.push_back(Request::new(i + 1, "-".to_string()));
                break;
            }

            match pages.binary_search_by_key(&request_name_input.trim(), | page | page.name.trim()) {
                Ok(_) => {
                    if !*is_last_input_valid {
                        terminal_clear_next_lines(3);
                        terminal_cursor_previous_line(2);
                    }

                    *is_last_input_valid = true;
                    request_name = request_name_input.trim().to_string();
                    requests.push_back(Request::new(i + 1, request_name));
                    break;
                }
                Err(_) => {
                    display("\n");
                    display_info("Please enter a valid page name from the given selection.\n");
                    display_error("Requested page not found from available pages.\n");

                    terminal_cursor_previous_line(4);

                    *is_last_input_valid = false;
                    continue;
                }
            }
        }
    }

    pages.sort_by_key(| page | page.number);
  
    return requests;
}


// *********************************************************
// Structure definitions and implementations
// *********************************************************
struct System {
    virtual_memory: VirtualMemory,
    pages: Vec<Page>,
    requests: VecDeque<Request>,
    handled_requests: Vec<Request>,
    snapshots: Vec<Snapshot>,
    snapshot_summary: (Vec<String>, Vec<Vec<String>>, Vec<String>),
    page_faults_count: usize,
    page_hits_count: usize,
}

impl System {
    fn new(virtual_memory: VirtualMemory, pages: Vec<Page>, requests: VecDeque<Request>) -> Self {
        return Self {
            virtual_memory: virtual_memory,
            pages: pages,
            requests: requests,
            handled_requests: Vec::new(),
            snapshots: Vec::new(),
            snapshot_summary: (Vec::new(), Vec::new(), Vec::new()),
            page_faults_count: 0,
            page_hits_count: 0,
        }
    }
  
    fn handle_requests(self: &mut Self) {
        let mut frames_number: Vec<usize>;

        frames_number = Vec::new();

        for frame in self.virtual_memory.frames.iter() {
            frames_number.push(frame.number);
        }
      
        while !self.requests.is_empty() {
            let request: Request;
            let snapshot: Snapshot;
            let mut logs: Vec<Log>;
            let request_name: String;
            let history: Vec<String>;
            let status: Status;

            request = self.requests.pop_front().unwrap();
            logs = Vec::new();

            if request.name != "-" {
                logs.push(Log::Request(request.name.clone()));
            }
            
            match self.get_requested_page(&request) {
                Ok(mut page) => {
                    status = Status::PageFault;
                    self.page_faults_count += 1;
                    logs.push(Log::Status(request.name.clone(), Status::PageFault));
                    
                    loop {
                        match self.virtual_memory.load(page) {
                            Ok(frame_number) => {
                                logs.push(Log::Load(request.name.clone(), frame_number));
                                break;
                            },
                            Err(failed_page) => {
                                let unloaded_page: Page;
                                let frame_number: usize;
                                
                                page = failed_page;
                                (frame_number, unloaded_page) = self.virtual_memory.unload(&self.requests);

                                logs.push(Log::Unload(unloaded_page.name.clone(), frame_number));
                              
                                self.pages.push(unloaded_page);
                                self.pages.sort_by_key(| page | page.number);
                            }
                        }
                    }
                },
                Err(_) => {
                    if request.name == "-" {
                        status = Status::NoRequest;
                        logs.push(Log::NoRequest);
                    }
                    else {
                        status = Status::PageHit;
                        self.page_hits_count += 1;
                        logs.push(Log::Status(request.name.clone(), Status::PageHit));
                    }
                }
            }

            for frame in self.virtual_memory.frames.iter_mut() {
                if let Some(page_name) = frame.check_page_name() {
                    if page_name == request.name {
                          frame.reset_turnaround();
                          break;
                    }
                }
            }

            self.virtual_memory.turnaround();

            request_name = request.name.clone();
            history = self.virtual_memory.take_history();
            snapshot = Snapshot::new(self.virtual_memory.turnaround_count, logs, request_name, history, status);

            self.snapshots.push(snapshot);

            self.handled_requests.push(request);
        }
    }

    fn get_requested_page(self: &mut Self, request: &Request) -> Result<Page, ()> {
        let requested_page: Page;
        
        for (i, page) in self.pages.iter().enumerate() {
            if request.name == page.name {
                requested_page = self.pages.remove(i);
                return Ok(requested_page);
            }
        }

        return Err(());
    }

    fn display_cumulative_snapshots(self: &mut Self) {
        let mut header: Vec<String>;
        let mut body: Vec<Vec<String>>;
        let mut footer: Vec<String>;
        let mut frames_name: Vec<String>;

        header = Vec::new();
        body = Vec::new();
        footer = Vec::new();
        frames_name = Vec::new();

        header.push("Frame".to_string());
      
        for request in self.handled_requests.iter() {
            header.push(request.name.clone());
        }

        for frame in self.virtual_memory.frames.iter() {
            frames_name.push(frame.name.clone());
        }

        body.push(frames_name);

        footer.push("Page Fault".to_string());
        
        for snapshot in self.snapshots.iter_mut() {
            let mut last: usize;
            
            last = body.len() - 1;
          
            for log in snapshot.logs.iter() {
                if let Log::Unload(_, frame_number) = log {
                    body[last][frame_number - 1] = format!("[{}]", body[last][frame_number - 1]);
                }
            }

            header[last] = header[last].replace(&['(', ')'][..], "");
            for i in 0..body[last].len() {
                body[last][i] = body[last][i].replace(&['(', ')'][..], "");
            }
          
            body.push(snapshot.history.clone());
          
            match snapshot.status {
                Status::PageFault => {
                    footer.push("*".to_string());
                },
                Status::PageHit | Status::NoRequest => {
                    footer.push(" ".to_string());
                }
            }

            last = body.len() - 1;

            header[last] = format!("({})", header[last]);
            for i in 0..body[last].len() {
                if body[last][i] == snapshot.request_name && snapshot.request_name != "-" {
                    body[last][i] = format!("({})", body[last][i]);
                }
            }

            display_header();
          
            display_title(&format!("Request {}\n", snapshot.number));
            
            display_subtitle("Virtual Memory Table\n");  
            display_table(&header, &body, &footer);

            display_divider(1);

            display_subtitle("Activity Log\n");
            snapshot.display_activity_log();
          
            display_footer();
        }
        let last: usize;
        last = body.len() - 1;
        header[last] = header[last].replace(&['(', ')'][..], "");
        for i in 0..body[last].len() {
            body[last][i] = body[last][i].replace(&['(', ')'][..], "");
        }
        self.snapshot_summary = (header, body, footer);
    }

    fn display_snapshots_summary(self: &Self) {
        display_table(&self.snapshot_summary.0, &self.snapshot_summary.1, &self.snapshot_summary.2);
    }

    fn display_conclusion(self: &Self) {
        display("All page requests were successfully handled.\n");
        display_labelled("Number of Requests", &format!("{}\n", self.page_faults_count + self.page_hits_count));
        display_labelled("Page Faults", &format!("{}\n", self.page_faults_count));
        display_labelled("Page Hits", &format!("{}\n", self.page_hits_count));
    }
}

struct VirtualMemory {
    frames: Vec<Frame>,
    turnaround_count: usize,
}

impl VirtualMemory {
    fn new(frames: Vec<Frame>) -> Self {
        return Self {
            frames: frames,
            turnaround_count: 0,
        }
    }

    fn load(self: &mut Self, page: Page) -> Result<usize, Page> {
        for frame in self.frames.iter_mut() {
            if frame.is_empty() {
                frame.allocate(page).unwrap();
                return Ok(frame.number);
            }
        }

        return Err(page);
    }

    fn unload(self: &mut Self, requests: &VecDeque<Request>) -> (usize, Page) {
        let mut unloaded_page: Page;
        let mut frame_index_with_max_distance: usize;
        let mut max_turnaround_count_of_max_distance: usize;
        let mut max_distance: usize;

        frame_index_with_max_distance = 0;
        max_turnaround_count_of_max_distance = 0;
        max_distance = 0;

        for (i, frame) in self.frames.iter_mut().enumerate() {
            if frame.is_empty() {
                continue;    
            }

            let loaded_page: Page;
            let mut distance: usize;

            loaded_page = frame.page.take().unwrap();
            distance = 0;

            for request in requests.iter() {
                distance += 1;
              
                if request.name == loaded_page.name {
                    distance -= 1;
                    break;
                }
            }

            if distance >= max_distance {                
                if distance == max_distance && loaded_page.turnaround_count < max_turnaround_count_of_max_distance {
                    frame.allocate(loaded_page).unwrap();
                    continue;
                }
              
                frame_index_with_max_distance = i;
                max_turnaround_count_of_max_distance = loaded_page.turnaround_count;
                max_distance = distance;
            }

            frame.allocate(loaded_page).unwrap();
        }
      
        unloaded_page = self.frames[frame_index_with_max_distance].deallocate().unwrap();
        unloaded_page.turnaround_count = 0;
        return (self.frames[frame_index_with_max_distance].number, unloaded_page);
    }

    fn turnaround(self: &mut Self) {
        for frame in self.frames.iter_mut() {
            frame.turnaround();
        }

        self.turnaround_count += 1;
    }

    fn take_history(self: &Self) -> Vec<String> {
        let mut history: Vec<String>;

        history = Vec::new();

        for frame in self.frames.iter() {
            let loaded_page: Option<&Page> = frame.page.as_ref();
            let loaded_page_name: String;

            match loaded_page {
                Some(page) => {
                    loaded_page_name = page.name.clone();
                }
                None => {
                    loaded_page_name = "-".to_string();
                }
            }

            history.push(loaded_page_name);
        }

        return history;
    }
}

struct Frame {
    number: usize,
    name: String,
    page: Option<Page>,
}

impl Frame {
    fn new(number: usize, name: String) -> Self {
        return Self {
            number: number,
            name: name,
            page: None,
        }
    }

    fn allocate(self: &mut Self, page: Page) -> Result<(), ()> {
        if !self.is_empty() {
            return Err(());
        }

        self.page = Some(page);

        return Ok(());
    }

    fn deallocate(self: &mut Self) -> Result<Page, ()> {
        let unloaded_page: Page;
      
        if self.is_empty() {
            return Err(());
        }

        unloaded_page = self.page.take().unwrap();

        return Ok(unloaded_page);
    }

    fn is_empty(self: &Self) -> bool {
        return self.page.is_none();
    }

    fn check_page_name(self: &Self) -> Option<String> {
        if self.page.is_some() {
            return Some(self.page.as_ref().unwrap().name.clone());
        }
      
        return None;
    }

    fn turnaround(self: &mut Self) {
        self.page.as_mut().map(| page | page.turnaround_count += 1);
    }

    fn reset_turnaround(self: &mut Self) {
        self.page.as_mut().map(| page | page.turnaround_count = 0);
    }
}

struct Page {
    number: usize,
    name: String,
    turnaround_count: usize,
}

impl Page {
    fn new(number: usize, name: String) -> Self {
        return Self {
            number: number,
            name: name,
            turnaround_count: 0,
        }
    }

    fn stringify_pages(pages: &mut Vec<Page>) -> String {
        let mut result: String;
        
        if pages.len() > 26 {
            let last: usize;
            
            last = pages.len() - 1;

            pages.sort_by_key(| page | page.number);
            result = format!("{}, {}, {}, ..., {}, {}, {}\n", pages[0].name, pages[1].name, pages[2].name, pages[last - 2].name, pages[last - 1].name, pages[last].name);
            pages.sort_by_key(| page | page.name.clone());
          
            return result;
        }
      
        result = String::new();
      
        for (i, page) in pages.iter().enumerate() {
            if i == 0 {
                result = format!("{}", page.name);
            }
            else {
                result = format!("{}, {}", result, page.name);
            }
        }

        result = format!("{}, -\n", result);

        return result;
    }
}

struct Request {
    number: usize,
    name: String,
}

impl Request {
    fn new(number: usize, name: String) -> Self {
        return Self {
            number: number,
            name: name,
        }
    }
}

struct Snapshot {
    number: usize,
    logs: Vec<Log>,
    request_name: String,
    history: Vec<String>,
    status: Status,
}

impl Snapshot {
    fn new(number: usize, logs: Vec<Log>, request_name: String, history: Vec<String>, status: Status) -> Self {
        return Self {
            number: number,
            logs: logs,
            request_name: request_name,
            history: history,
            status: status,
        };
    }

    fn display_table(self: &Self) {
        let mut header: Vec<String>;
        let mut body: Vec<Vec<String>>;
        let mut footer: Vec<String>;
      
        header = Vec::new();
        body = Vec::new();
        footer = Vec::new();

        header.push(self.request_name.clone());
        body.push(self.history.clone());
        match self.status {
            Status::PageFault => {
                footer.push("*".to_string());
            },
            Status::PageHit | Status::NoRequest => {
                footer.push(" ".to_string());
            }
        }

        display_table(&header, &body, &footer);
    }

    fn display_activity_log(self: &Self) {
        for log in self.logs.iter() {
            match log {
                Log::NoRequest => {
                    display_log("No page is currently requested.\n");
                },
                Log::Request(page_name) => {
                    display_log(&format!("Requesting Page {}...\n", page_name));
                },
                Log::Status(page_name, status) => {
                    match status {
                        Status::PageFault => {
                            display_info(&format!("Page Fault: Page {} is not yet loaded in virtual memory.\n", page_name));
                        },
                        Status::PageHit => {
                            display_info(&format!("Page Hit: Page {} is already loaded in virtual memory.\n", page_name));
                        },
                        Status::NoRequest => {},
                    }
                },
                Log::Load(page_name, frame_number) => {
                    display_success(&format!("Page {} is successfully loaded to Frame {}.\n", page_name, frame_number));
                },
                Log::Unload(page_name, frame_number) => {
                    display_success(&format!("Page {} is successfully unloaded from Frame {}.\n", page_name, frame_number));
                },
            }
        }
    }
}

enum Status {
    PageHit,
    PageFault,
    NoRequest,
}

enum Log {
    Request(String),
    Status(String, Status),
    Load(String, usize),
    Unload(String, usize),
    NoRequest,
}