# Changelog

All notable changes to the TailwindCSS Enhanced extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-01-04

### Added
- Initial release of TailwindCSS Enhanced extension for Zed
- Enhanced language server configuration for TailwindCSS
- Multi-framework support (React, Vue, Svelte, Astro, Angular, etc.)
- 25+ pre-built TailwindCSS snippets for common patterns
- Smart configuration for different project types
- Advanced class detection for CSS-in-JS solutions
- Support for styled-components, emotion, CVA, and clsx patterns
- Framework-specific attribute recognition (className, :class, ngClass, etc.)
- Dark mode ready with built-in support for dark mode classes
- WebAssembly-based extension for optimal performance
- Comprehensive documentation and setup instructions
- Build script for easy development workflow
- Example Zed settings configuration
- Support for multiple file types:
  - React/Next.js: `.jsx`, `.tsx`, `.js`, `.ts`
  - Vue/Nuxt.js: `.vue`, `.js`, `.ts`
  - Svelte/SvelteKit: `.svelte`, `.js`, `.ts`
  - Astro: `.astro`, `.jsx`, `.tsx`
  - Angular: `.html`, `.ts`, `.component.html`
  - Laravel/Blade: `.php`, `.blade.php`
  - Ruby on Rails: `.erb`, `.html.erb`
  - Phoenix/LiveView: `.heex`, `.eex`
  - Standard Web: `.html`, `.css`, `.scss`, `.sass`
  - Markdown: `.md`, `.mdx`

### Features
- **Language Server Integration**: Seamless integration with TailwindCSS language server
- **Intelligent Completion**: Context-aware autocomplete for TailwindCSS classes
- **Advanced Linting**: Comprehensive linting rules for TailwindCSS best practices
- **Color Decorators**: Visual color previews for TailwindCSS color classes
- **Hover Previews**: Detailed hover information for TailwindCSS utilities
- **Code Actions**: Quick fixes and refactoring actions
- **Multi-Language Support**: Works across different programming languages and frameworks
- **Regex-based Detection**: Advanced pattern matching for CSS-in-JS libraries
- **Responsive Design Helpers**: Built-in snippets for responsive design patterns
- **Component Patterns**: Pre-built snippets for common UI components

### Developer Experience
- Easy installation process with clear documentation
- Build script for streamlined development workflow
- Example configuration files for quick setup
- Comprehensive troubleshooting guide
- Performance optimized with WebAssembly compilation
- Clean and maintainable codebase structure

### Supported CSS-in-JS Patterns
- Template literals with `tw` prefix
- Styled-components integration
- Emotion CSS integration
- Class Variance Authority (CVA)
- clsx and classNames utilities
- Custom class regex patterns

### Framework-Specific Features
- **Next.js**: Full support for pages, components, and app directory structure
- **Nuxt.js**: Support for pages, components, layouts, and plugins
- **Vue**: Support for single-file components and composition API
- **Svelte**: Support for Svelte and SvelteKit projects
- **Astro**: Support for Astro components and islands
- **Angular**: Support for components and templates
- **Laravel**: Support for Blade templates and PHP files
- **Rails**: Support for ERB templates
- **Phoenix**: Support for LiveView and EEX templates