# TUI Mining Dashboard - Style Guide

## Overview

This is a **Terminal User Interface (TUI)** style cryptocurrency mining pool dashboard. The design mimics classic terminal/console applications with a dark theme, monospace typography, and bordered window panels.

---

## Design Philosophy

- **Terminal Aesthetic**: Everything should look like it belongs in a CLI/terminal application
- **Information Density**: Compact layouts that maximize data visibility
- **No Decorations**: Avoid rounded corners, shadows, gradients, or modern UI flourishes
- **Monochrome Base**: Primarily grayscale with strategic accent colors for status indicators

---

## Color System

All colors use HSL format and are defined in `index.css`. Never use direct color values in components.

### Core Colors

| Token | HSL Value | Usage |
|-------|-----------|-------|
| `--background` | `0 0% 6%` | Main app background (near-black) |
| `--foreground` | `0 0% 90%` | Primary text color (light gray) |
| `--card` | `0 0% 9%` | Panel/window backgrounds |
| `--border` | `0 0% 20%` | All borders and dividers |
| `--muted` | `0 0% 14%` | Subtle backgrounds |
| `--muted-foreground` | `0 0% 55%` | Labels, secondary text |

### Semantic Colors

| Token | HSL Value | Usage |
|-------|-----------|-------|
| `--accent` | `120 60% 50%` | Primary accent (green) |
| `--success` | `120 60% 45%` | Positive values, online status |
| `--warning` | `45 100% 50%` | Warnings, caution states |
| `--destructive` | `0 70% 50%` | Errors, negative values, offline |

### Usage in Tailwind

```tsx
// ✅ Correct - use semantic tokens
<div className="bg-background text-foreground border-border" />
<span className="text-muted-foreground" />
<div className="bg-card" />

// ❌ Wrong - never use direct colors
<div className="bg-black text-white border-gray-700" />
<div className="bg-[#1a1a1a]" />
```

---

## Typography

### Font Family

- **Primary**: `JetBrains Mono` (monospace)
- **Fallback**: `monospace`

All text uses the same monospace font. No font variations.

### Font Sizes

| Class | Size | Usage |
|-------|------|-------|
| `text-xs` | 10px | Table data, secondary info |
| `text-sm` | 12px | Default body text (base) |
| `text-base` | 14px | Emphasized content |
| `text-lg` | 16px | Panel values, important numbers |
| `text-xl+` | 18px+ | Rarely used, major headings only |

### Text Styling

```tsx
// Labels (muted, descriptive)
<span className="text-muted-foreground">Hashrate:</span>

// Values (bright, prominent)
<span className="text-foreground">125.4 TH/s</span>

// Status indicators
<span className="text-success">Online</span>      // Green - positive
<span className="text-destructive">Offline</span> // Red - negative  
<span className="text-warning">Pending</span>     // Yellow - caution
```

---

## Border Radius

**All components use `--radius: 0px`** - No rounded corners anywhere.

```tsx
// ✅ Correct - sharp corners
<div className="border border-border" />

// ❌ Wrong - no rounded corners in TUI
<div className="rounded-lg" />
<div className="rounded-md" />
```

---

## Component Patterns

### TUI Window (Panel)

The fundamental container for all content sections.

```tsx
<div className="tui-window">
  <div className="tui-title">[ PANEL TITLE ]</div>
  <div className="tui-content">
    {/* Content here */}
  </div>
</div>
```

**Title Format**: Always uppercase, wrapped in `[ ]` brackets

### TUI Row (Label-Value Pair)

For displaying key-value data pairs.

```tsx
<div className="tui-row">
  <span className="tui-label">Label:</span>
  <span className="tui-value">Value</span>
</div>

// With status variants
<span className="tui-value-up">+5.2%</span>    // Green
<span className="tui-value-down">-3.1%</span>  // Red
<span className="tui-value-warn">Pending</span> // Yellow
```

### TUI Table

Compact data tables without decoration.

```tsx
<table className="tui-table">
  <thead>
    <tr>
      <th>Column</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Data</td>
    </tr>
  </tbody>
</table>
```

### TUI Input

Terminal-style form inputs.

```tsx
<input className="bg-secondary border border-border px-2 py-1 text-foreground focus:outline-none focus:border-foreground" />
```

### TUI Button

Text-based buttons with bracket notation.

```tsx
// Standard button
<button className="px-3 py-1 border border-border hover:bg-secondary">
  [ Action ]
</button>

// Primary/accent button
<button className="px-3 py-1 bg-foreground text-background hover:bg-muted-foreground">
  [ Confirm ]
</button>
```

### TUI Checkbox

ASCII-style checkboxes.

```tsx
// Unchecked
<span>[  ]</span>

// Checked
<span>[X]</span>
```

### TUI Dropdown

Flat, borderless dropdown with caret indicator.

```tsx
<button className="flex items-center justify-between w-full bg-secondary border border-border px-2 py-1">
  <span>Selected Option</span>
  <span>▼</span>  // or ▲ when open
</button>
```

### TUI Modal

Overlay dialogs maintaining terminal aesthetic.

```tsx
<div className="fixed inset-0 bg-background/80 flex items-center justify-center">
  <div className="tui-window max-w-md w-full">
    <div className="tui-title">[ MODAL TITLE ]</div>
    <div className="tui-content">
      {/* Modal content */}
    </div>
  </div>
</div>
```

### Progress/Bar Indicator

Simple filled bars for percentages.

```tsx
<div className="tui-bar">
  <div className="tui-bar-fill" style={{ width: '75%' }} />
</div>
```

---

## Layout Patterns

### Grid System

Use CSS Grid for dashboard layouts.

```tsx
// Main dashboard grid
<div className="grid grid-cols-12 gap-2">
  <div className="col-span-3">Panel 1</div>
  <div className="col-span-6">Panel 2</div>
  <div className="col-span-3">Panel 3</div>
</div>
```

### Spacing

| Token | Value | Usage |
|-------|-------|-------|
| `gap-1` | 4px | Tight spacing within panels |
| `gap-2` | 8px | Standard panel gaps |
| `p-2` | 8px | Panel content padding |
| `py-0.5` | 2px | Row vertical spacing |
| `py-1` | 4px | Input/button padding |

---

## Status Bar

Bottom fixed bar for keyboard shortcuts and status.

```tsx
<div className="fixed bottom-0 left-0 right-0 bg-card border-t border-border px-4 py-1 flex justify-between text-xs">
  <div className="flex gap-4">
    <span><span className="text-foreground">[F1]</span> <span className="text-muted-foreground">Help</span></span>
  </div>
  <div className="text-success">● Connected</div>
</div>
```

---

## Animation

Minimal animations only. Use sparingly.

### Blink Animation

For cursor or status indicators.

```css
.blink {
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  50% { opacity: 0; }
}
```

---

## Icons & Symbols

Use ASCII/Unicode symbols instead of icon libraries.

| Symbol | Usage |
|--------|-------|
| `▲` / `▼` | Dropdown arrows, sort indicators |
| `●` | Status dot (online/offline) |
| `█` / `▓` / `░` | Progress bars, graphs |
| `[X]` / `[ ]` | Checkboxes |
| `[ ]` | Button/title brackets |
| `│` / `─` / `┌` / `┐` / `└` / `┘` | Box drawing (optional) |

---

## Don'ts

❌ **Never use:**
- Rounded corners (`rounded-*`)
- Box shadows (`shadow-*`)
- Gradients (`bg-gradient-*`)
- Icon libraries (Lucide icons should be minimal)
- Colorful backgrounds
- Modern card styles
- Hover animations with scale/transform
- Sans-serif fonts
- Direct color values (`bg-black`, `text-white`, `#hex`)

---

## File Organization

```
src/
├── components/
│   ├── tui/           # TUI-specific components
│   │   ├── TuiWindow.tsx
│   │   ├── TuiRow.tsx
│   │   ├── TuiInput.tsx
│   │   ├── TuiButton.tsx
│   │   ├── TuiCheckbox.tsx
│   │   ├── TuiDropdown.tsx
│   │   ├── TuiModal.tsx
│   │   └── menus/     # Modal menu content
│   └── ui/            # Shadcn base (rarely used)
├── index.css          # Design tokens, TUI classes
└── STYLE_GUIDE.md     # This file
```

---

## Quick Reference

```tsx
// Panel structure
<div className="tui-window">
  <div className="tui-title">[ TITLE ]</div>
  <div className="tui-content">{children}</div>
</div>

// Data row
<div className="tui-row">
  <span className="tui-label">Label:</span>
  <span className="tui-value">Value</span>
</div>

// Status colors
text-foreground     // Default white
text-muted-foreground // Gray labels
text-success        // Green positive
text-destructive    // Red negative
text-warning        // Yellow caution

// Backgrounds
bg-background       // Main dark bg
bg-card            // Panel bg
bg-secondary       // Input/subtle bg
border-border      // All borders
```
