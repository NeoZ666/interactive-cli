use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, MouseButton, MouseEventKind, read, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};

#[derive(Debug, Clone)]
struct Utxo {
    txid: String,
    vout: u32,
    amount: u64, // in satoshis
    address: String,
    confirmations: u32,
}

fn draw_box(text: &Utxo, is_selected: bool, is_cursor: bool) -> Vec<String> {
    let width = text.txid.len() + 10; // Fixed width for consistent formatting
    let mut lines = Vec::new();
    
    // Top border
    lines.push(format!("┌{}┐", "─".repeat(width - 2)));
    
    // Content with selection marker and cursor
    let marker = if is_selected { "✓" } else { " " };
    let cursor_char = if is_cursor { "►" } else { " " };
    
    // Header line with markers
    // let header_content = format!("{} Details {}", marker, cursor_char);
    // lines.push(format!("│ {:<width$} │", header_content, width = width - 4));
    lines.push(format!("│{}│", " ".repeat(width - 2)));
    
    // UTXO details on separate lines - all using the same width
    lines.push(format!("│ txid: {:<width$} │", text.txid, width = width - 10));
    lines.push(format!("│ vout: {:<width$} │", text.vout, width = width - 10));
    lines.push(format!("│ amount: {} sats{:<width$}{}{}", text.amount, "", marker, cursor_char, width = width - 20 - text.amount.to_string().len()));
    lines.push(format!("│ address: {:<width$} │", text.address, width = width - 13));
    lines.push(format!("│ confirmations: {:<width$} │", text.confirmations, width = width - 19));
    
    // Bottom border
    lines.push(format!("└{}┘", "─".repeat(width - 2)));
    
    lines
}

fn main() -> std::io::Result<()> {
    // Fixed vector of choices
    let choices = vec![
        Utxo {
            txid: "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            vout: 0,
            amount: 100_000, // 0.001 BTC
            address: "bc1qxyz1234567890abcdef1234567890abcdef".to_string(),
            confirmations: 6,
        },
        Utxo  {
            txid: "876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba9".to_string(),
            vout: 1,
            amount: 500_000, // 0.005 BTC
            address: "bc1qabc9876543210fedcba9876543210fedcba".to_string(),
            confirmations: 0,
        },
        Utxo {
            txid: "456789ab1234567890abcdef1234567890abcdef1234567890abcdef12345678".to_string(),
            vout: 2,
            amount: 1_000_000, // 0.01 BTC
            address: "bc1qdef4567890abcdef1234567890abcdef123".to_string(),
            confirmations: 12,
        },
    ];
    
    let mut selected = vec![false; choices.len()];
    let mut cursor: usize = 0;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), EnableMouseCapture)?;

    loop {
        // Clear screen and render choices
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        
        println!("Use ↑/↓ to navigate, SPACE to select, CLICK to select, ENTER to confirm, Q to quit\n");

        let mut line_offset = 3; // Start after instructions
        let mut box_positions = Vec::new(); // Track where each box is positioned
        
        for (i, choice) in choices.iter().enumerate() {
            let box_start_line = line_offset;
            let box_lines = draw_box(choice, selected[i], i == cursor);
            let box_height = box_lines.len() as u16;
            
            for line in box_lines {
                execute!(stdout, MoveTo(0, line_offset))?;
                println!("{}", line);
                line_offset += 1;
            }
            line_offset += 1; // Extra space between boxes
            
            // Store the box position and height for mouse detection
            box_positions.push((box_start_line, box_height));
        }
        stdout.flush()?;

        // Handle input
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => cursor = cursor.saturating_sub(1),
                    KeyCode::Down => cursor = (cursor + 1).min(choices.len() - 1),
                    KeyCode::Char(' ') => selected[cursor] = !selected[cursor],
                    KeyCode::Enter => break,
                    KeyCode::Char('q') => {
                        execute!(stdout, DisableMouseCapture)?;
                        disable_raw_mode()?;
                        return Ok(());
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse_event) => {
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    let click_row = mouse_event.row;
                    
                    // Check which box was clicked
                    for (i, (box_start, box_height)) in box_positions.iter().enumerate() {
                        if click_row >= *box_start && click_row < (*box_start + *box_height) {
                            cursor = i;
                            selected[i] = !selected[i];
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    execute!(stdout, DisableMouseCapture)?;
    disable_raw_mode()?;
    println!(
        "Selected: {:?}",
        choices
            .into_iter()
            .zip(selected)
            .filter(|(_, sel)| *sel)
            .map(|(choice, _)| choice)
            .collect::<Vec<_>>()
    );
    Ok(())
}
