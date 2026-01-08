# TailwindCSS Enhanced - Zed Extension

A comprehensive Zed extension that enhances TailwindCSS support across all frontend frameworks including Next.js, Nuxt.js, Vue, Svelte, Astro, and more.

## Features

- 🚀 **Enhanced Language Server**: Advanced TailwindCSS language server configuration.
- 🎯 **Multi-Framework Support**: Works with React, Vue, Svelte, Astro, Angular, and more
- 📝 **Rich Snippets**: 25+ pre-built TailwindCSS snippets for common patterns
- 🔧 **Smart Configuration**: Automatic detection and configuration for different project types
- 🎨 **Advanced Class Detection**: Support for styled-components, emotion, and other CSS-in-JS solutions
- 🌙 **Dark Mode Ready**: Built-in support for dark mode classes
- ⚡ **Performance Optimized**: WebAssembly-based extension for maximum performance

## Supported Frameworks & File Types

- **React/Next.js**: `.jsx`, `.tsx`, `.js`, `.ts`
- **Vue/Nuxt.js**: `.vue`, `.js`, `.ts`
- **Svelte/SvelteKit**: `.svelte`, `.js`, `.ts`
- **Astro**: `.astro`, `.jsx`, `.tsx`
- **Angular**: `.html`, `.ts`, `.component.html`
- **Laravel/Blade**: `.php`, `.blade.php`
- **Ruby on Rails**: `.erb`, `.html.erb`
- **Phoenix/LiveView**: `.heex`, `.eex`
- **Standard Web**: `.html`, `.css`, `.scss`, `.sass`
- **Markdown**: `.md`, `.mdx`

## Installation

### From Zed Extensions (Recommended)

1. Open Zed
2. Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "Extensions" and select "Extensions: Open Extensions"
4. Search for "TailwindCSS Enhanced"
5. Click "Install"

### Development Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/zed-tailwindcss.git
   cd zed-tailwindcss
   ```

2. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Add WebAssembly target:
   ```bash
   rustup target add wasm32-wasip1
   ```

4. Build the extension:
   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

   Or use the build script:
   ```bash
   ./build.sh
   ```

5. Install as dev extension in Zed:
   - Open Zed
   - Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "Install Dev Extension" and select it
   - Navigate to and select this directory

## Usage

### Automatic Configuration

The extension automatically detects your project type and applies appropriate configurations. No manual setup required!

### Snippets

Type any of these prefixes and press Tab to expand:

#### Layout & Positioning
- `tw-flex-center` → `flex items-center justify-center`
- `tw-grid-center` → `grid place-items-center`
- `tw-abs-center` → Absolute positioning center
- `tw-full` → `w-full h-full`

#### Components
- `tw-btn-primary` → Primary button styling
- `tw-btn-secondary` → Secondary button styling
- `tw-card` → Basic card component
- `tw-input` → Styled input field
- `tw-badge` → Badge component
- `tw-navbar` → Navigation bar
- `tw-modal-overlay` → Modal overlay

#### Effects & Animations
- `tw-hover-transition` → Smooth hover transition
- `tw-shadow-hover` → Shadow effect on hover
- `tw-spinner` → Loading spinner animation
- `tw-gradient` → Gradient background

#### Responsive Design
- `tw-container` → Responsive container
- `tw-text-responsive` → Responsive text sizing
- `tw-grid-layout` → Responsive grid layout
- `tw-padding-responsive` → Responsive padding

#### Alerts & States
- `tw-alert-success` → Success alert styling
- `tw-alert-error` → Error alert styling
- `tw-focus` → Focus ring styling
- `tw-dark` → Dark mode classes

### Advanced Features

#### CSS-in-JS Support

The extension automatically detects and provides IntelliSense for:

```javascript
// Template literals
const Button = tw`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded`;

// Styled-components
const StyledButton = styled.button`
  ${tw`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded`}
`;

// Emotion
const buttonStyles = css`
  ${tw`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded`}
`;

// Class variance authority (CVA)
const button = cva("px-4 py-2 rounded", {
  variants: {
    intent: {
      primary: "bg-blue-500 text-white",
      secondary: "bg-gray-200 text-gray-900"
    }
  }
});
```

#### Framework-Specific Attributes

The extension recognizes framework-specific class attributes:

- **React**: `className`
- **Vue**: `:class`, `v-bind:class`
- **Angular**: `ngClass`
- **Astro**: `class:list`

## Configuration

### Custom Settings

Add these to your Zed `settings.json` for additional customization:

```json
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "tailwindCSS": {
          "classAttributes": ["class", "className", "ngClass", "class:list"],
          "experimental": {
            "classRegex": [
              "tw`([^`]*)",
              "tw=\"([^\"]*)",
              "tw={\"([^\"}]*)",
              "classNames\\s*\\(\\s*['\"`]([^'\"`]*)['\"`]",
              "cva\\s*\\([^)]*['\"`]([^'\"`]*)['\"`]"
            ]
          },
          "emmetCompletions": false,
          "validate": true,
          "lint": {
            "cssConflict": "warning",
            "invalidApply": "error",
            "invalidScreen": "error",
            "invalidVariant": "error",
            "invalidConfigPath": "error",
            "invalidTailwindDirective": "error",
            "recommendedVariantOrder": "warning"
          },
          "hovers": true,
          "suggestions": true,
          "codeActions": true,
          "colorDecorators": true
        }
      }
    }
  }
}
```

### Project-Specific Configuration

The extension looks for and respects your project's TailwindCSS configuration files:

- `tailwind.config.js`
- `tailwind.config.ts`
- `tailwind.config.cjs`
- `tailwind.config.mjs`

## Troubleshooting

### Language Server Not Starting

1. Ensure you have Node.js installed (version 14 or later)
2. Check Zed's log: `View → Developer → Open Logs`
3. Restart Zed with verbose logging: `zed --foreground`

### IntelliSense Not Working

1. Verify your project has a `tailwind.config.js` file
2. Check that TailwindCSS is installed in your project:
   ```bash
   npm list tailwindcss
   ```
3. Make sure your file type is supported (see list above)

### Custom Class Regex Not Working

Add your custom patterns to the `experimental.classRegex` array in your Zed settings (see Configuration section above).

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and test them
4. Build the extension: `cargo build --target wasm32-wasi --release`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## Requirements

- **Zed Editor**: Latest version
- **Rust**: 1.70+ (for development)
- **Node.js**: 16+ (for TailwindCSS language server)
- **TailwindCSS**: Any version (in your project)

### Installing TailwindCSS Language Server

If you don't have the TailwindCSS language server installed globally, you can install it:

```bash
npm install -g @tailwindcss/language-server
```

Or using your project's package manager:

```bash
# npm
npm install -D @tailwindcss/language-server

# yarn
yarn add -D @tailwindcss/language-server

# pnpm
pnpm add -D @tailwindcss/language-server
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Quick Setup

1. **Install the extension** (see Installation section above)
2. **Copy example settings**: Use the provided `example-zed-settings.json` as a starting point for your Zed configuration
3. **Install TailwindCSS language server** globally or in your project
4. **Create/verify your TailwindCSS config** in your project root
5. **Start coding** with enhanced TailwindCSS support!

## Changelog

### v0.1.0
- Initial release
- Enhanced language server configuration
- Multi-framework support
- 25+ TailwindCSS snippets
- CSS-in-JS support
- Framework-specific attribute recognition
- WebAssembly-based extension for performance
- Build script for easy development

## Support

If you encounter any issues or have suggestions:

1. Check the [troubleshooting section](#troubleshooting)
2. Search existing [GitHub Issues](https://github.com/yourusername/zed-tailwindcss/issues)
3. Create a new issue with:
   - Your Zed version
   - Your operating system
   - Steps to reproduce the issue
   - Relevant log output

## Acknowledgments

- [TailwindLabs](https://tailwindlabs.com) for the amazing TailwindCSS framework and language server
- [Zed Industries](https://zed.dev) for the fast and extensible editor
- The Zed community for feedback and contributions..
