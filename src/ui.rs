use std::{io, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, List, ListItem, Paragraph, Tabs}, layout::{Constraint, Direction, Layout}, text::Spans};

use crate::state::{AppState, FocusArea, Preset};
use crate::ffmpeg;

pub fn run_selection(state: &mut AppState) -> anyhow::Result<bool> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut start = false;
    loop {
        terminal.draw(|f| ui(f, state))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => { break; },
                    KeyCode::Tab => {
                        state.focus = match state.focus {
                            FocusArea::Files => FocusArea::Presets,
                            FocusArea::Presets => FocusArea::Files,
                        }
                    }
                    KeyCode::Up => match state.focus {
                        FocusArea::Files => {
                            if state.cursor > 0 { state.cursor -= 1; }
                        }
                        FocusArea::Presets => {
                            state.preset = Preset::CpuH264;
                        }
                    },
                    KeyCode::Down => match state.focus {
                        FocusArea::Files => {
                            if state.cursor + 1 < state.files.len() { state.cursor += 1; }
                        }
                        FocusArea::Presets => {
                            state.preset = Preset::GpuH264;
                        }
                    },
                    KeyCode::Char(' ') => {
                        if let FocusArea::Files = state.focus {
                            if state.cursor < state.files.len() {
                                let sel = &mut state.files[state.cursor];
                                sel.selected = !sel.selected;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        start = true;
                        break; // signal conversion start
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(start)
}

pub async fn run_conversion(state: &mut AppState) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    for idx in 0..state.files.len() {
        if !state.files[idx].selected { continue; }
        state.files[idx].progress = Some("starting".into());
        terminal.draw(|f| progress_ui(f, state))?;
        ffmpeg::convert_file(&mut state.files[idx], state.preset).await?;
        terminal.draw(|f| progress_ui(f, state))?;
    }

    loop {
        terminal.draw(|f| progress_ui(f, state))?;
        if state.all_done() { break; }
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Esc { break; }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Length(3),
            Constraint::Length(3),
        ]).split(f.size());

    let items: Vec<ListItem> = state
        .files
        .iter()
        .map(|f| {
            let mark = if f.selected { "[x]" } else { "[ ]" };
            ListItem::new(Spans::from(format!("{} {}", mark, f.path.display())))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"));
    let mut stateful = tui::widgets::ListState::default();
    if !state.files.is_empty() {
        stateful.select(Some(state.cursor));
    }
    f.render_stateful_widget(list, chunks[0], &mut stateful);

    let preset_titles: Vec<Spans> = Preset::ALL.iter().map(|p| Spans::from((*p).name())).collect();
    let tabs = Tabs::new(preset_titles)
        .select(match state.preset { Preset::CpuH264 => 0, Preset::GpuH264 => 1 })
        .block(Block::default().borders(Borders::ALL).title("Preset"));
    f.render_widget(tabs, chunks[1]);

    let help = Paragraph::new("Tab: switch | Space: select | Enter: start | Esc: exit");
    f.render_widget(help, chunks[2]);
}

fn progress_ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, state: &AppState) {
    let items: Vec<ListItem> = state
        .files
        .iter()
        .filter(|f| f.selected)
        .map(|f| {
            let prog = f.progress.clone().unwrap_or_else(|| "waiting".into());
            ListItem::new(Spans::from(format!("{} - {}", f.path.display(), prog)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(list, f.size());
}
