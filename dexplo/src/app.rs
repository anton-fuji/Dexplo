use ratatui::widgets::ListState;
use std::{cmp::min, io, path::PathBuf};

pub struct App {
    pub current_path: PathBuf,
    pub items: Vec<(String, bool)>,
    pub state: ListState,
    pub preview_content: String,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let current_path = std::env::current_dir()?;
        let mut app = App {
            current_path,
            items: Vec::new(),
            state: ListState::default(),
            preview_content: String::new(),
        };
        app.load_files()?;
        Ok(app)
    }

    pub fn load_files(&mut self) -> io::Result<()> {
        self.items.clear();
        self.items.push(("/..".to_string(), true));

        for entry in std::fs::read_dir(&self.current_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().into_owned(),
                None => continue,
            };

            if file_name.starts_with('.') && file_name != ".." {
                continue;
            }

            let is_dir = path.is_dir();
            let prefix = if is_dir { " " } else { " " };
            self.items
                .push((format!("{}{}", prefix, file_name), is_dir));
        }

        self.items[1..].sort_by_key(|(name, _)| name.to_lowercase());
        self.state
            .select(Some(min(1, self.items.len().saturating_sub(1))));
        self.update_preview();
        Ok(())
    }

    pub fn selected_index(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    pub fn move_up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.update_preview();
    }

    pub fn move_down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.update_preview();
    }

    pub fn enter_selected(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let i = self.selected_index();
        if self.items.is_empty() {
            return Ok(());
        }

        let selected_item = &self.items[i];

        let raw_name = selected_item
            .0
            .trim_start_matches("/")
            .trim_start_matches(" ")
            .trim_start_matches(" ");

        let is_dir = selected_item.1;

        if is_dir {
            if raw_name == ".." {
                if self.current_path.pop() {
                    self.load_files()?;
                }
            } else {
                let new_path = self.current_path.join(raw_name);
                if new_path.is_dir() {
                    self.current_path = new_path;
                    self.load_files()?;
                }
            }
        } else {
            let file_path = self.current_path.join(raw_name);
            self.open_file(&file_path)?;
        }
        Ok(())
    }

    fn open_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(path).spawn()?;
        }

        Ok(())
    }

    pub fn go_parent(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_path.pop() {
            self.load_files()?;
        }
        Ok(())
    }

    pub fn update_preview(&mut self) {
        let i = self.selected_index();
        if self.items.is_empty() {
            self.preview_content = String::from("No items");
            return;
        }

        let selected_item = &self.items[i];
        let raw_name = selected_item
            .0
            .trim_start_matches("󰁞 ")
            .trim_start_matches(" ")
            .trim_start_matches(" ");

        let is_dir = selected_item.1;

        if raw_name == ".." {
            self.preview_content = String::from("Parent directory");
            return;
        }

        let path = self.current_path.join(raw_name);

        if is_dir {
            // Directory preview: show contents
            match std::fs::read_dir(&path) {
                Ok(entries) => {
                    let mut content = String::new();
                    let mut items: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().into_owned();
                            if e.path().is_dir() {
                                format!(" {}", name)
                            } else {
                                format!(" {}", name)
                            }
                        })
                        .collect();

                    items.sort();

                    for (idx, item) in items.iter().enumerate() {
                        content.push_str(item);
                        if idx < items.len() - 1 {
                            content.push('\n');
                        }
                    }

                    if content.is_empty() {
                        content = String::from("Empty directory");
                    }

                    self.preview_content = content;
                }
                Err(e) => {
                    self.preview_content = format!("Error reading directory: {}", e);
                }
            }
        } else {
            // File preview: show file contents
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    // Limit to first 500 lines for performance
                    let lines: Vec<&str> = content.lines().take(500).collect();
                    self.preview_content = lines.join("\n");
                }
                Err(_) => {
                    // Try to read as binary and show file info
                    match std::fs::metadata(&path) {
                        Ok(metadata) => {
                            self.preview_content = format!(
                                "Binary file\n\nSize: {} bytes\nType: {}",
                                metadata.len(),
                                if metadata.is_file() { "File" } else { "Other" }
                            );
                        }
                        Err(e) => {
                            self.preview_content = format!("Error reading file: {}", e);
                        }
                    }
                }
            }
        }
    }
}
