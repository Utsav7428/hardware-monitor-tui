use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::{io, time::Duration};
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, RefreshKind,
    System,Networks,Components
};
use tokio::sync::mpsc;
use tokio::time::interval;

use super::app::App;
use super::event::GuiEvent;
use super::ui::ui;

pub async fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let (tx, mut rx) = mpsc::channel(32);

   
    let tick_rate = Duration::from_millis(250);
    let tx_tick = tx.clone(); 
    let _tick_handle = tokio::spawn(async move {
        let mut last_tick = interval(tick_rate);
        loop {
            last_tick.tick().await;
            if tx_tick.send(GuiEvent::Tick).await.is_err() {
                break; 
            }
        }
    });


    let tx_input = tx; 
    let _input_handle = tokio::spawn(async move {
        loop {
            // Poll for key events
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        if tx_input.send(GuiEvent::Input(key)).await.is_err() {
                            break; // Receiver dropped
                        }
                    }
                }
            }
        }
    });


    let mut app = App::new();
    
    
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())

        
    );
     let networks = Networks::new_with_refreshed_list();
     let components= Components::new_with_refreshed_list();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        match rx.recv().await {
            Some(GuiEvent::Tick) => {
        
                sys.refresh_cpu_specifics(CpuRefreshKind::everything());
                sys.refresh_memory_specifics(MemoryRefreshKind::everything());
                app.on_tick(&sys, &networks,&components);
            }
            Some(GuiEvent::Input(key)) => {
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    _ => {}
                }
            }
            _ => break, // Channel closed
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}