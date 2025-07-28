# Interactive CLI UTXO Selector

## Crossterm

```
CLICK on any UTXO to select/deselect, Press any key to exit

┌────────────────────────────────────────────────────────────────────────┐
│                                                                        │
│ txid: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef │
│ vout: 0                                                                │
│ amount: 100000 sats                                               ✓
│ address: bc1qxyz1234567890abcdef1234567890abcdef                       │
│ confirmations: 6                                                       │
└────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────┐
│                                                                        │
│ txid: 876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba9 │
│ vout: 1                                                                │
│ amount: 500000 sats                                                    
│ address: bc1qabc9876543210fedcba9876543210fedcba                       │
│ confirmations: 0                                                       │
└────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────┐
│                                                                        │
│ txid: 456789ab1234567890abcdef1234567890abcdef1234567890abcdef12345678 │
│ vout: 2                                                                │
│ amount: 1000000 sats                                              ✓    
│ address: bc1qdef4567890abcdef1234567890abcdef123                       │
│ confirmations: 12                                                      │
└────────────────────────────────────────────────────────────────────────┘
```

## Usage

```bash
cargo run
```

## Controls

| Key/Action | Function |
|------------|----------|
| `Mouse Click` | Select/deselect clicked item |
| `ANY KEY` | Confirm selections and exit |

## Dependencies

- `crossterm` - Cross-platform terminal manipulation library

## To Check

- Check for terminal compatinility across different Operating Systems. 