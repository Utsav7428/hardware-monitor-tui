# Rust System Monitor TUI

  A lightweight, high-performance, terminal-based system monitor written in Rust. This application provides real-time insights into your system's CPU, memory, network, and hardware temperatures.
  
  It is built with an asynchronous, event-driven architecture to ensure minimal resource consumption and a non-blocking UI.

---

## Features

  Real-time Metrics: View live data for global CPU usage, memory usage, and network I/O (transmit/receive).
  
  Hardware Monitoring: Displays component temperatures (e.g., CPU core temperature).
  
  High Performance: Built with tokio for an asynchronous runtime, ensuring the UI remains responsive and data collection is efficient.
  
  Efficient Rendering: Uses ratatui for a terminal-based user interface, which minimizes I/O and renders only the changes to the screen.
  
  Cross-Platform: Utilizes the sysinfo library for cross-platform data collection (Windows, macOS, Linux).

----

## Technical Stack

  This project leverages several powerful libraries from the Rust ecosystem:
  
      Rust: The core programming language.
      
      Tokio: An asynchronous runtime for non-blocking I/O.
      
      Ratatui: A modern Rust library for building Terminal User Interfaces.
      
      Crossterm: The terminal backend used by Ratatui for input and screen control.
      
      Sysinfo: A cross-platform library for fetching system information.
