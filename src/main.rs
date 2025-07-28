use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture, Event, MouseButton, MouseEventKind, read},
    execute, queue,
    style::Print,
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

fn interactive_select(choices: Vec<Utxo>) -> std::io::Result<Vec<Utxo>> {
    let mut selected = vec![false; choices.len()];
    let mut box_positions = Vec::new(); // Stores (start_line, height, checkbox_line) for each UTXO
    let mut stdout = stdout();

    // Initialize terminal
    enable_raw_mode()?;
    execute!(
        stdout,
        Clear(ClearType::All),
        EnableMouseCapture,
        MoveTo(0, 0)
    )?;
    println!("CLICK on any UTXO to select/deselect, Press any key to exit\n");

    // Initial render of all UTXO boxes
    let mut line_offset = 3;
    box_positions.clear();
    for (i, choice) in choices.iter().enumerate() {
        let box_start_line = line_offset;
        let width = choice.txid.len() + 10;
        let marker = if selected[i] { "✓" } else { " " };
        let box_lines = [
            format!("┌{}┐", "─".repeat(width - 2)),
            format!("│{}│", " ".repeat(width - 2)),
            format!("│ txid: {:<width$} │", choice.txid, width = width - 10),
            format!("│ vout: {:<width$} │", choice.vout, width = width - 10),
            format!(
                "│ amount: {} sats{:<width$}{} ",
                choice.amount,
                "",
                marker,
                width = width - 18 - choice.amount.to_string().len()
            ),
            format!(
                "│ address: {:<width$} │",
                choice.address,
                width = width - 13
            ),
            format!(
                "│ confirmations: {:<width$} │",
                choice.confirmations,
                width = width - 19
            ),
            format!("└{}┘", "─".repeat(width - 2)),
        ];
        let box_height = box_lines.len() as u16;
        let checkbox_line = box_start_line + 4; // Checkbox is on the 5th line (index 4) of the box

        for line in box_lines.iter() {
            queue!(stdout, MoveTo(0, line_offset), Print(line))?;
            line_offset += 1;
        }
        line_offset += 1; // Extra line between boxes
        box_positions.push((box_start_line, box_height, checkbox_line));
    }
    stdout.flush()?;

    // Event loop
    loop {
        match read()? {
            Event::Mouse(mouse_event) => {
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    let click_row = mouse_event.row;
                    for (i, (box_start, box_height, checkbox_line)) in
                        box_positions.iter().enumerate()
                    {
                        if click_row >= *box_start && click_row < *box_start + *box_height {
                            // Toggle selection
                            selected[i] = !selected[i];
                            // Update only the checkbox line
                            let marker = if selected[i] { "✓" } else { " " };
                            let width = choices[i].txid.len() + 10;
                            let line = format!(
                                "│ amount: {} sats{:<width$}{} ",
                                choices[i].amount,
                                "",
                                marker,
                                width = width - 18 - choices[i].amount.to_string().len()
                            );
                            queue!(stdout, MoveTo(0, *checkbox_line), Print(line))?;
                            stdout.flush()?;
                            break;
                        }
                    }
                }
            }
            Event::Key(_) => break,
            _ => {}
        }
    }

    // Cleanup
    execute!(stdout, DisableMouseCapture, Clear(ClearType::All))?;
    disable_raw_mode()?;

    Ok(choices
        .into_iter()
        .zip(selected)
        .filter(|(_, sel)| *sel)
        .map(|(choice, _)| choice)
        .collect())
}

fn main() -> std::io::Result<()> {
    let choices = vec![
        Utxo {
            txid: "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            vout: 0,
            amount: 100_000,
            address: "bc1qxyz1234567890abcdef1234567890abcdef".to_string(),
            confirmations: 6,
        },
        Utxo {
            txid: "876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba9".to_string(),
            vout: 1,
            amount: 500_000,
            address: "bc1qabc9876543210fedcba9876543210fedcba".to_string(),
            confirmations: 0,
        },
        Utxo {
            txid: "456789ab1234567890abcdef1234567890abcdef1234567890abcdef12345678".to_string(),
            vout: 2,
            amount: 1_000_000,
            address: "bc1qdef4567890abcdef1234567890abcdef123".to_string(),
            confirmations: 12,
        },
    ];

    let selected_utxos = interactive_select(choices)?;
    println!("Manually Selected UTXOs: {:#?}", selected_utxos);

    Ok(())
}
