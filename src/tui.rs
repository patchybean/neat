//! Interactive TUI for neatcli

use std::io;
use std::path::{Path, PathBuf};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::classifier::{Category, Classifier};
use crate::organizer::{plan_moves, OrganizeMode, PlannedMove};
use crate::scanner::{format_size, scan_directory, FileInfo, ScanOptions};

/// Current view mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    FileList,
    Preview,
    Confirm,
}

/// Organize mode selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedMode {
    ByType,
    ByDate,
    ByExtension,
}

impl SelectedMode {
    fn to_organize_mode(&self) -> OrganizeMode {
        match self {
            SelectedMode::ByType => OrganizeMode::ByType,
            SelectedMode::ByDate => OrganizeMode::ByDate,
            SelectedMode::ByExtension => OrganizeMode::ByExtension,
        }
    }

    fn name(&self) -> &str {
        match self {
            SelectedMode::ByType => "By Type",
            SelectedMode::ByDate => "By Date",
            SelectedMode::ByExtension => "By Extension",
        }
    }

    fn next(&self) -> Self {
        match self {
            SelectedMode::ByType => SelectedMode::ByDate,
            SelectedMode::ByDate => SelectedMode::ByExtension,
            SelectedMode::ByExtension => SelectedMode::ByType,
        }
    }
}

/// Application state
pub struct App {
    /// Current directory path
    pub path: PathBuf,
    /// List of files in directory
    pub files: Vec<FileInfo>,
    /// Selected file indices
    pub selected: Vec<usize>,
    /// Current list state
    pub list_state: ListState,
    /// Current view mode
    pub view_mode: ViewMode,
    /// Organize mode
    pub organize_mode: SelectedMode,
    /// Planned moves (for preview)
    pub planned_moves: Vec<PlannedMove>,
    /// Classifier
    pub classifier: Classifier,
    /// Should quit
    pub should_quit: bool,
    /// Status message
    pub status_message: String,
}

impl App {
    pub fn new(path: &Path) -> Result<Self> {
        let options = ScanOptions {
            include_hidden: false,
            max_depth: Some(1),
            follow_symlinks: false,
        };

        let canonical_path = path.canonicalize()?;
        let files = scan_directory(&canonical_path, &options)?;

        let mut list_state = ListState::default();
        if !files.is_empty() {
            list_state.select(Some(0));
        }

        Ok(App {
            path: canonical_path,
            files,
            selected: Vec::new(),
            list_state,
            view_mode: ViewMode::FileList,
            organize_mode: SelectedMode::ByType,
            planned_moves: Vec::new(),
            classifier: Classifier::new(),
            should_quit: false,
            status_message: "Press ? for help".to_string(),
        })
    }

    /// Toggle selection of current item
    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if self.selected.contains(&i) {
                self.selected.retain(|&x| x != i);
            } else {
                self.selected.push(i);
            }
        }
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.selected = (0..self.files.len()).collect();
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        self.selected.clear();
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i > 0 {
                self.list_state.select(Some(i - 1));
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.files.len().saturating_sub(1) {
                self.list_state.select(Some(i + 1));
            }
        }
    }

    /// Generate preview of moves
    pub fn generate_preview(&mut self) {
        let files_to_organize: Vec<FileInfo> = if self.selected.is_empty() {
            self.files.clone()
        } else {
            self.selected.iter().filter_map(|&i| self.files.get(i).cloned()).collect()
        };

        self.planned_moves = plan_moves(
            &files_to_organize,
            &self.path,
            self.organize_mode.to_organize_mode(),
        );

        if self.planned_moves.is_empty() {
            self.status_message = "No files to organize".to_string();
        } else {
            self.view_mode = ViewMode::Preview;
            self.status_message = format!("{} files will be moved", self.planned_moves.len());
        }
    }

    /// Execute moves
    pub fn execute_moves(&mut self) -> Result<()> {
        use crate::organizer::execute_moves;
        
        let mode_name = self.organize_mode.name().to_lowercase().replace(" ", "-");
        execute_moves(&self.planned_moves, &format!("tui organize {}", mode_name))?;
        
        self.status_message = format!("✓ Moved {} files", self.planned_moves.len());
        self.planned_moves.clear();
        self.view_mode = ViewMode::FileList;
        
        // Refresh file list
        let options = ScanOptions {
            include_hidden: false,
            max_depth: Some(1),
            follow_symlinks: false,
        };
        self.files = scan_directory(&self.path, &options)?;
        self.selected.clear();
        
        if !self.files.is_empty() {
            self.list_state.select(Some(0));
        }
        
        Ok(())
    }
}

/// Run the TUI
pub fn run_tui(path: &Path) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(path)?;

    // Main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.view_mode {
                    ViewMode::FileList => match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                        KeyCode::Char(' ') => app.toggle_selection(),
                        KeyCode::Char('a') => app.select_all(),
                        KeyCode::Char('d') => app.deselect_all(),
                        KeyCode::Char('m') => {
                            app.organize_mode = app.organize_mode.next();
                            app.status_message = format!("Mode: {}", app.organize_mode.name());
                        }
                        KeyCode::Enter | KeyCode::Char('p') => app.generate_preview(),
                        KeyCode::Char('?') => {
                            app.status_message = "↑↓:nav  Space:select  a:all  d:deselect  m:mode  Enter:preview  q:quit".to_string();
                        }
                        _ => {}
                    },
                    ViewMode::Preview => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.view_mode = ViewMode::FileList;
                            app.status_message = "Cancelled".to_string();
                        }
                        KeyCode::Enter | KeyCode::Char('y') => {
                            app.view_mode = ViewMode::Confirm;
                        }
                        _ => {}
                    },
                    ViewMode::Confirm => match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => {
                            if let Err(e) = app.execute_moves() {
                                app.status_message = format!("Error: {}", e);
                            }
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.view_mode = ViewMode::FileList;
                            app.status_message = "Cancelled".to_string();
                        }
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Status
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(format!(
        " 📁 {} | Mode: {} | Selected: {}",
        app.path.display(),
        app.organize_mode.name(),
        app.selected.len()
    ))
    .style(Style::default().fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).title(" neatcli "));
    f.render_widget(header, chunks[0]);

    // Main content based on view mode
    match app.view_mode {
        ViewMode::FileList => render_file_list(f, app, chunks[1]),
        ViewMode::Preview => render_preview(f, app, chunks[1]),
        ViewMode::Confirm => render_confirm(f, app, chunks[1]),
    }

    // Status bar
    let status = Paragraph::new(format!(" {} ", app.status_message))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn render_file_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let selected = if app.selected.contains(&i) { "[✓]" } else { "[ ]" };
            let category = app.classifier.classify(file.extension.as_deref());
            let icon = category_icon(&category);
            
            let content = format!(
                "{} {} {} ({:>8})",
                selected,
                icon,
                file.name,
                format_size(file.size)
            );
            
            let style = if app.selected.contains(&i) {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Files "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    f.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .planned_moves
        .iter()
        .map(|mv| {
            let from = mv.from.file_name().unwrap_or_default().to_string_lossy();
            let to_folder = mv.to.parent()
                .and_then(|p| p.strip_prefix(&app.path).ok())
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            
            ListItem::new(format!("  {} → {}/", from, to_folder))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Preview (Enter=confirm, Esc=cancel) "));

    f.render_widget(list, area);
}

fn render_confirm(f: &mut Frame, app: &App, area: Rect) {
    let text = format!(
        "\n\n  Move {} files?\n\n  Press 'y' to confirm, 'n' to cancel",
        app.planned_moves.len()
    );
    
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(" Confirm "));
    
    f.render_widget(paragraph, area);
}

fn category_icon(category: &Category) -> &'static str {
    match category {
        Category::Images => "🖼️",
        Category::Documents => "📄",
        Category::Videos => "🎬",
        Category::Audio => "🎵",
        Category::Archives => "📦",
        Category::Code => "💻",
        Category::Data => "📊",
        Category::Other => "📁",
    }
}
