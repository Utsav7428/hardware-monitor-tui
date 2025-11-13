use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, Paragraph, Row, Table},
};

use super::app::App; 

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Min(3),        
        ])
        .split(f.size());

   
    let header = Paragraph::new("Rust Hardware Monitor")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);


    let datasets = vec![Dataset::default()
        .name("CPU Usage %")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Red))
        .data(&app.cpu_usage)];

    let x_min = if app.window_width > 100.0 { app.window_width - 100.0 } else { 0.0 };
    let x_max = if app.window_width > 100.0 { app.window_width } else { 100.0 };

    let chart = Chart::new(datasets)
        .block(Block::default().title("CPU Metrics").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([x_min, x_max]),
        )
        .y_axis(
            Axis::default()
                .title("Usage %")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0]),
        );
    f.render_widget(chart, chunks[1]);

 
    let mem_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(app.memory_usage / 100.0);
    f.render_widget(mem_gauge, mem_chunks[0]);

 let temp_gauge = Gauge::default()
        .block(Block::default().title("Core Temperature").borders(Borders::ALL))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        // Set ratio assuming 100°C is the max
        .ratio((app.core_temp / 100.0).clamp(0.0, 1.0) as f64)
        .label(format!("{:.1}°C", app.core_temp)); // Display the temperature
    f.render_widget(temp_gauge, mem_chunks[1]);


    let rx_mb = format!("{:.2} MB", app.network_rx as f64 / 1_048_576.0);
    let tx_mb = format!("{:.2} MB", app.network_tx as f64 / 1_048_576.0);

    let header_row = Row::new(vec!["Device", "RX", "TX"])
        .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1);

    let data_row = Row::new(vec!["All Interfaces", rx_mb.as_str(), tx_mb.as_str()]);

    let table = Table::new(
        vec![data_row],
        [
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ],
    )
    .block(Block::default().title("Network IO").borders(Borders::ALL))
    .header(header_row);

    f.render_widget(table, chunks[3]);
}