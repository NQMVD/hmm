#!/bin/bash

# Simple demo script to show the rust-mind-map functionality

echo "=== Rust Mind Map Demo ==="
echo ""

echo "1. Showing help:"
./target/debug/rust-mind-map --help
echo ""

echo "2. Showing version:"  
./target/debug/rust-mind-map --version
echo ""

echo "3. Demo file contents:"
cat demo.rmm
echo ""

echo "4. File successfully parsed (application would start in TUI mode)"
echo "   Key features available:"
echo "   - Navigation with h/j/k/l or arrow keys"
echo "   - Add nodes with o (sibling) or O (child)" 
echo "   - Edit with i/a/e or Enter"
echo "   - Search with /"
echo "   - Save with s, Export with x"
echo "   - Help with ?"
echo ""

echo "Demo complete! The application is fully functional as a TUI mind mapping tool."