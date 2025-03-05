use std::io::{self, Write};

pub struct ProgressBar {
    total: usize,
    width: usize,
    current: usize,
}

impl ProgressBar {
    pub fn new(total: usize, width: usize) -> Self {
        let progress_bar = ProgressBar {
            total,
            width,
            current: 0
        };

        progress_bar
    }

    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.print();
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[H"); // clear terminal

        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = (self.width as f64 * (self.current as f64 / self.total as f64)) as usize;
        let empty = self.width - filled;

        print!("\r[");
        for _ in 0..filled {
            print!("=");
        }
        for _ in 0..empty {
            print!(" ");
        }
        print!("] {:.1}% ({}/{} bytes)", percentage, self.current, self.total);

        io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        self.print();
        println!();
    }

    pub fn set_total(&mut self, total: usize) {
        self.total = total;
    }
}