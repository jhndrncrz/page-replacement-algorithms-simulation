use std::io;
use std::io::prelude::*;
use std::process::Command;

// *********************************************************
// Displays
// *********************************************************
pub fn display_header() {
    const NAME: &'static str = "CRUZ, JOHN ADRIAN B.";
    const TERM: &'static str = "Midterm";
    const MODULE_NUMBER: i8 = 4;
    const ACTIVITY_NUMBER: i8 = 1;
    const ACTIVITY_TITLE: &'static str = "LRU Page Replacement Algorithm";
  
    system_clear_screen();

    display_title(&format!(
      "{}L-M{}: ACT{} {}\n",
      TERM.as_bytes()[0] as char,
      MODULE_NUMBER,
      ACTIVITY_NUMBER,
      ACTIVITY_TITLE
    ));
    display_subtitle(&format!("{}\n", NAME));
    display("───────────────────────────────────────────────────────────\n");
}

pub fn display_footer() {
    display("───────────────────────────────────────────────────────────\n");
    display_pause();
    system_clear_screen();
}

pub fn display_table(headers: &Vec<String>, body: &Vec<Vec<String>>, footers: &Vec<String>) {
    let mut max_columns: Vec<usize> = vec![0; headers.len()];

    for i in 0..headers.len() {
        if i < body.len() {
            for data in &body[i] {
                if data.len() > max_columns[i] {
                    max_columns[i] = data.len();
                }
            }
        }

        if i < footers.len() {
            if footers[i].len() > max_columns[i] {
                max_columns[i] = footers[i].len();
            }
        }

        if headers[i].len() > max_columns[i] {
            max_columns[i] = headers[i].len();
        }
    }

    // edge border
    for i in 0..headers.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[i] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }
    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().write("│".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // headers
    for j in 0..headers.len() {
        if j == 0 {
            io::stdout()
            .write(
                text_apply_style(
                    &format!(" {: <width$} ", headers[j], width = max_columns[j]),
                    "bold",
                )
                .as_bytes(),
            )
            .unwrap();
        } else {
            io::stdout()
            .write(
                text_apply_style(
                    &format!(" {: ^width$} ", headers[j], width = max_columns[j]),
                    "bold",
                )
                .as_bytes(),
            )
            .unwrap();
        }
        io::stdout().write("│".as_bytes()).unwrap();
    }
    io::stdout().write("\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // edge border
    for j in 0..headers.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[j] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }
    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // body
    for i in 0..body[0].len() {
        io::stdout().write("│".as_bytes()).unwrap();
      
        for j in 0..headers.len() {
            if j == 0 {
                io::stdout()
                .write(
                    if j >= body.len() {
                        format!(" {: <width$} ", " ", width = max_columns[j])
                    }
                    else if body[j][i].contains("[") && body[j][i].contains("]") {
                        text_apply_style(
                            &format!(" {: <width$} ", body[j][i], width = max_columns[j]),
                            "fg-red",
                        )
                    }
                    else if body[j][i].contains("(") && body[j][i].contains(")") {
                        text_apply_style(
                            &format!(" {: <width$} ", body[j][i], width = max_columns[j]),
                            "bg-yellow",
                        )
                    }
                    else if body[j][i].contains("-") {
                        text_apply_style(
                            &format!(" {: <width$} ", body[j][i], width = max_columns[j]),
                            "fg-yellow italic",
                        )
                    } 
                    else {
                        format!(" {: <width$} ", body[j][i], width = max_columns[j])
                    }
                    .as_bytes(),
                )
                .unwrap();
            }
            else {
                io::stdout()
                .write(
                    if j >= body.len() {
                        format!(" {: <width$} ", " ", width = max_columns[j])
                    }
                    else if body[j][i].contains("[") && body[j][i].contains("]") {
                        text_apply_style(
                            &format!(" {: ^width$} ", body[j][i], width = max_columns[j]),
                            "fg-red",
                        )
                    }
                    else if body[j][i].contains("(") && body[j][i].contains(")") {
                        text_apply_style(
                            &format!(" {: ^width$} ", body[j][i], width = max_columns[j]),
                            "bg-green",
                        )
                    }
                    else if body[j][i].contains("-") {
                        text_apply_style(
                            &format!(" {: ^width$} ", body[j][i], width = max_columns[j]),
                            "fg-yellow italic",
                        )
                    } 
                    else {
                        format!(" {: ^width$} ", body[j][i], width = max_columns[j])
                    }
                    .as_bytes(),
                )
                .unwrap();
            }
            
            io::stdout().write("│".as_bytes()).unwrap();
        }
        io::stdout().write("\n".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }

    // edge border
    for i in 0..headers.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[i] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }

    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().write("│".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // footers
    for j in 0..headers.len() {
        if j >= body.len() {
            io::stdout()
            .write(
                format!(" {: <width$} ", " ", width = max_columns[j])
                .as_bytes(),
            )
            .unwrap();
        }
        else if j == 0 {
            io::stdout()
            .write(
                text_apply_style(
                    &format!(" {: <width$} ", footers[j], width = max_columns[j]),
                    "bold",
                )
                .as_bytes(),
            )
            .unwrap();
        } else {
            io::stdout()
            .write(
                text_apply_style(
                    &format!(" {: ^width$} ", footers[j], width = max_columns[j]),
                    "bold fg-red",
                )
                .as_bytes(),
            )
            .unwrap();
        }
        io::stdout().write("│".as_bytes()).unwrap();
    }
    io::stdout().write("\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // edge border
    for j in 0..headers.len() {
        io::stdout().write("┼".as_bytes()).unwrap();
        for _ in 0..max_columns[j] + 2 {
            io::stdout().write("─".as_bytes()).unwrap();
        }
    }
    io::stdout().write("┼\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn display(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout().write(string.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_pause() {
    if cfg!(windows) {
        Command::new("cmd").args(["/c", "pause"]).status().unwrap();
    } else if cfg!(unix) {
        Command::new("bash")
            .args([
                "-c",
                &format!(
                    "read -n 1 -s -r -p \"{}\"",
                    text_apply_style(&"Press any key to continue. . .", "italic")
                ),
            ])
            .status()
            .unwrap();
    }
    terminal_clear_line(0);
}

pub fn display_title(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(text_apply_style(string, "bold").as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_subtitle(string: &str) {
    if string.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(text_apply_style(string, "italic").as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_divider(n: usize) {
    for _ in 0..n {
        io::stdout().write("\n".as_bytes()).unwrap();
    }
    io::stdout().flush().unwrap();
}

pub fn display_prompt(message: &str, descriptor: &str) -> String {
    terminal_clear_line(0);
    io::stdout()
        .write(format!("{} {}: ", message, text_apply_style(descriptor, "italic")).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input;
}

pub fn display_confirm(message: &str, descriptor: &str) -> char {
    terminal_clear_line(0);
    io::stdout()
        .write(
            format!(
                "{} {}? ([Y]es/[N]o): ",
                message,
                text_apply_style(descriptor, "italic")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.to_uppercase().chars().next().unwrap();
}

pub fn display_labelled(label: &str, value: &str) {
    if value.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(format!("{}: {}", text_apply_style(label, "italic"), value).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_log(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" LOG ", "bold bg-white"),
                text_apply_style(message, "italic")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_success(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" SUCCESS ", "bold bg-green"),
                text_apply_style(message, "italic fg-green")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_error(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" ERROR ", "bold bg-red"),
                text_apply_style(message, "italic fg-red")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn display_info(message: &str) {
    if message.contains("\n") {
        terminal_clear_line(0);
    }
    io::stdout()
        .write(
            format!(
                "{} {}",
                text_apply_style(" INFO ", "bold bg-yellow"),
                text_apply_style(message, "italic fg-yellow")
            )
            .as_bytes(),
        )
        .unwrap();
    io::stdout().flush().unwrap();
}

// *********************************************************
// Terminal control
// *********************************************************
pub fn system_clear_screen() {
    if cfg!(windows) {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else if cfg!(unix) {
        Command::new("clear").status().unwrap();
    }
}

pub fn terminal_clear_screen(n: i8) {    
    match n {
        -1 => {
            io::stdout().write("\x1B[1J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        1 => {
            io::stdout().write("\x1B[0J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        _ => {
            io::stdout().write("\x1B[2J\x1B[1;1H".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn terminal_clear_line(n: i8) {
    match n {
        -1 => {
            io::stdout().write("\x1B[1K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        1 => {
            io::stdout().write("\x1B[0K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
        _ => {
            io::stdout().write("\x1B[2K\x1B[1G".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn terminal_cursor_up(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}A", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_down(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}B", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_right(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}C", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_left(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}D", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_next_line(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}E", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_cursor_previous_line(n: usize) {
    io::stdout()
        .write(format!("\x1B[{}F", n).as_bytes())
        .unwrap();
    io::stdout().flush().unwrap();
}

pub fn terminal_clear_previous_lines(n: usize) {
    terminal_clear_line(0);
    for _ in 1..n {
        terminal_cursor_previous_line(1);
        terminal_clear_line(0);
    }
}

pub fn terminal_clear_next_lines(n: usize) {
    terminal_clear_line(0);
    for _ in 1..n {
        terminal_cursor_next_line(1);
        terminal_clear_line(0);
    }
}

// *********************************************************
// Text style
// *********************************************************
pub fn text_apply_style(string: &str, styles: &str) -> String {
    let mut ansi_style_codes: String = String::new();

    for style in styles.split(" ") {
        ansi_style_codes.push_str(match style {
            "bold" => "\x1B[1m",
            "italic" => "\x1B[3m",
            "underline" => "\x1B[4m",
            "fg-black" => "\x1B[30m",
            "fg-red" => "\x1B[31m",
            "fg-green" => "\x1B[32m",
            "fg-yellow" => "\x1B[33m",
            "fg-blue" => "\x1B[34m",
            "fg-magenta" => "\x1B[35m",
            "fg-cyan" => "\x1B[36m",
            "fg-white" => "\x1B[37m",
            "bg-red" => "\x1B[41m",
            "bg-green" => "\x1B[42m",
            "bg-yellow" => "\x1B[43m",
            "bg-blue" => "\x1B[44m",
            "bg-magenta" => "\x1B[45m",
            "bg-cyan" => "\x1B[46m",
            "bg-white" => "\x1B[47m",
            _ => "",
        });
    }

    return format!("{}{}\x1B[0m", ansi_style_codes, string);
}

pub fn decimal_to_letteral(mut n: usize) -> String {
    let mut result: String;
    result = String::new();

    while n > 0 {
        let remainder: usize;
  
        remainder = (n - 1) % 26;
        result = format!("{}{}", (remainder as u8 + 'A' as u8) as char, result);
        n = (n - remainder) / 26;
    }
  
    return result;
}
