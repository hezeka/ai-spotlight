# AI Spotlight

A sleek, macOS Spotlight-like AI assistant interface for Windows that integrates with local LLM servers like LM Studio.

For a better experience, use light models whose responses your system processes as quickly as possible. This is not a primary chat for conversation but a fast assistant that can handle simple tasks.

![AI Spotlight Interface](/dist/planets.jpg)

## Features

- **Spotlight-like Interface**: Beautiful, translucent overlay window similar to macOS Spotlight
- **Global Hotkey**: Quick access with `Ctrl+Shift+Space`
- **Click-through Support**: Window becomes transparent when not in use
- **Local LLM Integration**: Works with LM Studio and other OpenAI-compatible APIs
- **Customizable Settings**: Adjust model, temperature, max tokens, and system prompts
- **Always on Top**: Stays accessible above other windows
- **System Tray Integration**: Minimizes to system tray
- **Animated Responses**: Smooth loading animations and response display

## Prerequisites

- Windows 10/11
- [LM Studio](https://lmstudio.ai/) or another OpenAI-compatible local server
- Node.js (for development)
- Rust (for building from source)

## Quick Start

### Using Pre-built Release

1. Download the latest release from the [Releases](../../releases) page
2. Extract and run `AI Spotlight.exe`
3. Make sure LM Studio is running on `localhost:1234`
4. Press `Ctrl+Shift+Space` to open the spotlight
5. Start asking questions!

### Building from Source

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd spotlight-ai
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Install Rust and Tauri CLI**
   ```bash
   # Install Rust from https://rustup.rs/
   cargo install tauri-cli
   ```

4. **Run in development mode**
   ```bash
   npm run dev
   ```

5. **Build for production**
   ```bash
   npm run build
   ```

## Configuration

### LM Studio Setup

1. Download and install [LM Studio](https://lmstudio.ai/)
2. Download your preferred model (e.g., Llama, Mistral, etc.)
3. Start the local server:
   - Go to "Local Server" tab
   - Click "Start Server"
   - Ensure it's running on port 1234
   - Turn on the «Enable CORS» option.

![LM Studio Settings](/dist/lm-studio-settings.jpg)

### AI Spotlight Settings

Click the "⚙️ Настройки" button to configure:

- **Model**: Choose from available models in LM Studio
- **Max Tokens**: Response length limit (100-4000)
- **Temperature**: Creativity level (0.0-2.0)
- **API URL**: Server endpoint (default: `http://localhost:1234/v1/chat/completions`) (this input in hided by default, remove style="display:none;" attribute in html)
- **User Switch**: Some models do not understand system prompts. You can enable this switch and the system prompt will be sent on behalf of the user.
- **System Prompt**: Custom instructions for the AI

## Usage

1. **Open Spotlight**: Press `Ctrl+Shift+Space` or click the system tray icon
2. **Ask Questions**: Type your question and press Enter
3. **View Response**: The AI response appears below with timing information
4. **Close**: Press Escape or click elsewhere to hide
5. **Settings**: Click the settings button to customize behavior
6. **Drag**: Drag spotlight window by "search icon"

## Keyboard Shortcuts

- `Ctrl+Shift+Space` - Toggle spotlight window
- `Enter` - Send query
- `Escape` - Close window or settings panel

## Technical Details

### Architecture

- **Frontend**: HTML/CSS/JavaScript with modern styling
- **Backend**: Rust with Tauri framework
- **API**: OpenAI-compatible REST API integration
- **Platform**: Windows-specific with click-through support

### Key Features Implementation

- **Click-through**: Uses Windows API to make window transparent to mouse clicks
- **Global Shortcuts**: Tauri's global shortcut management
- **Window Management**: Dynamic resizing and positioning
- **System Tray**: Background operation with tray icon

### File Structure

```
spotlight-ai/
├── dist/                 # Built frontend files
│   └── index.html       # Main application interface
├── src-tauri/           # Rust backend
│   ├── src/
│   │   └── main.rs      # Main Tauri application
│   ├── Cargo.toml       # Rust dependencies
│   └── tauri.conf.json  # Tauri configuration
├── package.json         # Node.js dependencies
└── README.md           # This file
```

## Troubleshooting

### Common Issues

**Spotlight doesn't open**
- Check if LM Studio is running
- Verify the global shortcut isn't conflicting with other apps
- Try running as administrator

**No response from AI**
- Ensure LM Studio server is started
- Check API URL in settings
- Verify a model is loaded in LM Studio

**Window positioning issues**
- Try centering the window from system tray menu
- Check display scaling settings

### Logs

Check the browser console (F12) for JavaScript errors or Tauri logs in the terminal when running in development mode.

## Development

### Running in Development

```bash
npm run dev
```

This starts the Tauri application in development mode with hot reload.

### Building

```bash
npm run build
```

### Install by EXE of MSI

```
Run the /dist/MSI/AI Spotlight_*.*.*_x64_en-US.msi
Or the /dist/MSI/AI Spotlight_*.*.*_x64_en-US.exe
```

Creates a production build in `src-tauri/target/release/bundle/`.

### Dependencies

- **Tauri**: Desktop application framework
- **Serde**: JSON serialization for Rust
- **Windows API**: For click-through functionality
- **Raw Window Handle**: Low-level window access

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by macOS Spotlight
- Built with [Tauri](https://tauri.app/)
- Designed for [LM Studio](https://lmstudio.ai/) integration

## Support

For issues and questions:
1. Check the [Issues](../../issues) page
2. Create a new issue with detailed information
3. Include system information and error messages