# Vizia CSS compared with standard CSS

Vizia styles its own view tree, not an HTML DOM. Its stylesheets use familiar
CSS syntax, but they are not a browser CSS implementation. This document
describes the behavior of this repository's current implementation; it is not a
complete CSS conformance table. In particular, a value that the parser accepts
is not necessarily applied or rendered like it would be in a browser.

## Layout and positioning

| Standard CSS | Vizia |
| --- | --- |
| Browser layout modes such as `display: block`, `flex`, and `grid` | `display` accepts only `flex` and `none`. Choose the layout with `layout-type: row`, `column`, `grid`, or `overlay`; this is not the standard flexbox or grid API. |
| `flex-direction` | An alias for `layout-type`. The `row` and `column` values select Vizia's corresponding layout; `row-reverse` and `column-reverse` are not supported. The alias also accepts Vizia's nonstandard `grid` and `overlay` values. |
| `position: relative` or `absolute` | `position` is an alias for `position-type`. Only `relative` and `absolute` are supported; `static`, `fixed`, and `sticky` are not. Positioning follows Vizia layout behavior. |
| `flex-wrap` | `flex-wrap` is an alias for `wrap`, accepting standard `wrap` and `nowrap` plus Vizia's legacy `no-wrap`. `wrap-reverse` is not supported; wrapping follows Vizia layout behavior. |
| `align-items`, `justify-content`, etc. | Vizia provides `alignment`, with values including `center`, `top-left`, `top-center`, `top-right`, `left`, `right`, `bottom-left`, `bottom-center`, and `bottom-right`. It is not a one-to-one replacement for the CSS alignment properties. |
| `grid-template-columns`, `grid-template-rows`, and grid placement properties | The template names alias `grid-columns` and `grid-rows`. Tracks use Vizia layout units, not the full CSS grid track syntax. Placement uses `column-start`, `column-span`, `row-start`, and `row-span`; Vizia start indices are zero-based, unlike CSS grid lines. |
| `width` and `height` | Vizia also provides `size` to set both dimensions, and `min-size` and `max-size` to set both corresponding constraints. |
| `margin` and `inset` shorthands | Vizia provides `space` to set `left`, `right`, `top`, and `bottom`. These layout properties are not a browser margin shorthand. |
| `padding: 4px 8px` | Vizia supports one to four values using CSS top/right/bottom/left expansion. Values use Vizia layout units; a single `var()` can supply one value, but mixed or multi-value variable substitution is not supported. |
| `gap: 4px 8px`, `row-gap`, `column-gap` | Vizia supports one or two values, in row/column order, using Vizia layout units. A single `var()` can supply one value, but mixed or multi-value variable substitution is not supported. `row-gap` and `column-gap` alias `vertical-gap` and `horizontal-gap`. Vizia also has `min-gap`, `max-gap`, and per-axis gap constraints. |
| `overflow: visible`, `hidden`, `clip`, `scroll`, or `auto` | `overflow`, `overflow-x`, and `overflow-y` accept `visible` and `hidden`; the other listed values are not supported. |

### Layout units

For layout properties using Vizia's `Units` type, including widths, offsets,
padding, and gaps, the parser accepts `auto`, percentages, absolute lengths
convertible to pixels (`px`, `in`, `cm`, `mm`, `q`, `pt`, and `pc`), and the
Vizia-specific stretch unit `s`. Unitless zero is accepted for these
properties, including longhands and the `padding` and `gap` shorthands. For
example, `width: 1s` requests one share of remaining space; in a CSS
transition, by contrast, `1s` means one second.

Do not assume browser length support for layout properties: `em`, `rem`, and
viewport units are not convertible to pixels by the layout-units parser.
Other properties use different value types, so unit and `calc()` support
depends on the property.

## Appearance and text

| Standard CSS | Vizia |
| --- | --- |
| `border-radius` and `border-top-left-radius`, etc. | These alias `corner-radius` and the corresponding `corner-*-radius` properties. The value parser does not support the full CSS elliptical radius syntax. |
| `box-shadow` | An alias for `shadow`. Supports `none`, unitless zero, color and `inset` before or after lengths, and comma-separated shadows. Vizia also accepts some nonstandard orderings and negative blur radii. Length resolution and animation behavior are not fully equivalent to browser CSS; transitions between `none` and shadows with omitted color or `currentColor` are discrete. |
| `background` shorthand | Use individual properties such as `background-color`, `background-image`, `background-position`, `background-repeat`, and `background-size`. `background-repeat` accepts `repeat`, `repeat-x`, `repeat-y`, and `no-repeat`. |
| `outline-style` | Not present in the property list. `outline` applies its color and width; use `outline-color`, `outline-width`, and `outline-offset` as needed. |
| `mix-blend-mode` | An alias for `blend-mode`. |
| `font-style`, `font-stretch`, and `font` shorthand | `font-style` and `font-stretch` alias `font-slant` and `font-width`; their values remain limited to Vizia's parsers. The `font` shorthand is not in the property list. |
| `text-wrap` CSS keywords | Vizia's `text-wrap` is a boolean, for example `text-wrap: false`. |
| `::selection` | Pseudo-elements do not match in the current implementation; Vizia provides `selection-color` instead. |
| Text decoration thickness properties | Vizia parses `underline-thickness`, `overline-thickness`, and `strikethrough-thickness`, but the current stylesheet property application does not apply them. |

Vizia also recognizes familiar properties such as `color`, `font-family`,
`font-size`, `font-weight`, `border`, `opacity`, `transform`, `filter`, and
`cursor`. Their presence does **not** imply that every standard value or
shorthand is supported. See the [property list](crates/vizia_style/src/property.rs)
and the individual [value parsers](crates/vizia_style/src/values/) for details.

## Selectors

- Type selectors match Vizia view element names, such as `button`, `label`,
  `textbox`, and `window`, rather than HTML elements. Class and ID selectors,
  combinators, and various structural selectors are supported on the view tree.
  Parent and sibling matching uses the layout tree.
- `:root` matches Vizia's root entity, not every window. Use the `window`
  selector to style windows.
- Attribute selectors do not match: the view-tree selector implementation
  returns false for attribute matches. Link and namespace matching are also
  not implemented.
- Vizia recognizes state selectors including `:hover`, `:active`, `:focus`,
  `:focus-visible`, `:focus-within`, `:disabled`, and `:checked`. It also
  provides the nonstandard `:over`. State availability depends on the view.
- `:dir()` and `:has()` have matching implementations. `:lang()` is parsed but
  its matcher is unimplemented and can panic. Unknown nonfunctional
  pseudo-classes are treated as custom pseudo-classes, whose matcher is also
  unimplemented and can panic.
- Pseudo-elements such as `::before`, `::after`, and `::selection` are parsed
  but do not match views. They do not create browser-style generated content.

See the [selector parser](crates/vizia_style/src/selector.rs) and
[view-tree matcher](crates/vizia_core/src/systems/style.rs).

## Variables, cascade, and rules

- Custom properties and `var()` are supported for selected property/value
  types, including colors, several lengths and layout units, opacity, and
  shadows. This is not general CSS token substitution: the property must be
  handled by the variable application code, and supported fallbacks depend on
  its value type. For example, `background-color: var(--accent, #ff0000)` is
  supported, but do not assume arbitrary substitutions inside compound values.
- Standard property aliases also work for the variable-backed properties,
  supported keyframe properties, and supported transition property names.
  Existing Vizia property names continue to work; aliases do not add support
  for properties the animation system does not already handle.
- Inline Rust style modifiers override shared stylesheet styling.
- `!important` is parsed into a separate declaration list, but the current
  theme loader applies only ordinary declarations. Do not use `!important` to
  override a rule: the important declaration itself is not applied.
- Nested style rules are represented by the parser, but the theme loader does
  not apply their nested rule lists. Write selectors as separate top-level
  rules instead.
- `@keyframes` is the only implemented stylesheet at-rule. Browser at-rules
  such as `@media`, `@supports`, `@import`, `@font-face`, and `@layer` are not
  implemented by this parser. The theme loader uses error recovery; unsupported
  syntax should not be assumed to fail like it would in a browser.

See [declaration parsing](crates/vizia_style/src/declaration.rs),
[rule parsing](crates/vizia_style/src/parser/rule.rs), and
[theme loading and variable application](crates/vizia_core/src/style/mod.rs).

## Animations and transitions

- `@keyframes` definitions are loaded as named Vizia animations. Standard
  `animation` and `animation-*` properties are not in the property list; play
  named animations through Vizia's animation API instead.
- `transition` accepts a comma-separated list starting with `property duration`,
  followed by an optional delay and timing function in either order, for example
  `transition: width 200ms ease 50ms` or `width 200ms 50ms ease`. Other standard
  orderings and separate `transition-*` longhands are not supported. The loader
  implements transitions for selected properties, not all CSS properties.
- Supported timing functions include `linear`, `ease`, `ease-in`, `ease-out`,
  `ease-in-out`, and `cubic-bezier(...)`; `steps(...)` is not implemented.

See the [transition parser](crates/vizia_style/src/values/transition.rs),
[easing parser](crates/vizia_style/src/values/easing.rs), and
[animation API](crates/vizia_core/src/context/event.rs).

## Example

```css
:root {
    --panel-color: #303030;
}

window {
    background-color: #202020;
}

.panel {
    layout-type: column;
    width: 1s;
    padding: 8px;
    row-gap: 4px;
    background-color: var(--panel-color, #303030);
    border-radius: 6px;
}

.panel > button:hover {
    background-color: #555555;
}
```

The built-in [layout stylesheet](crates/vizia_core/resources/themes/default_layout.css)
and [theme](crates/vizia_core/resources/themes/default_theme.css) contain more
examples of Vizia's actual syntax.
