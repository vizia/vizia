# Vizia CSS compared with standard CSS

Vizia styles its own view tree, not an HTML DOM. This document describes
differences from standard CSS in the current implementation. The missing
features listed below are representative, not exhaustive: CSS spans many
modules, and a value that parses is not necessarily applied or rendered as in
a browser.

## Missing standard features

### Properties

These are not implemented as built-in CSS properties. Some have Vizia-specific
alternatives, which are not necessarily equivalent. Partially supported
properties are described in the sections below instead.

| Area | Missing properties |
| --- | --- |
| Margins and insets | `margin`, `margin-top`, `margin-right`, `margin-bottom`, `margin-left`, `inset`, `inset-block`, `inset-inline` and their start/end longhands. Vizia's `space` and `left`/`right`/`top`/`bottom` are not CSS margins or inset shorthands. |
| Flex and alignment | `flex`, `flex-grow`, `flex-shrink`, `flex-basis`, `order`, `align-items`, `align-self`, `align-content`, `justify-content`, `justify-items`, `justify-self`, `place-items`, `place-content`, `place-self`. |
| Grid | `grid`, `grid-template`, `grid-template-areas`, `grid-auto-columns`, `grid-auto-rows`, `grid-auto-flow`, `grid-column`, `grid-row`, `grid-area`, `grid-column-start`, `grid-column-end`, `grid-row-start`, `grid-row-end`. The template columns and rows names are partial aliases, not missing properties. |
| Sizing and replaced content | `box-sizing`, `object-fit`, `object-position`. |
| Backgrounds and borders | `background-attachment`, `background-origin`, `background-clip`, `border-image` and its longhands, `outline-style`. |
| Text and fonts | `font`, `font-kerning`, `font-feature-settings`, `text-shadow`, `text-transform`, `text-indent`, `text-decoration-thickness`, `text-underline-offset`, `word-spacing`, `white-space`, `word-break`, `overflow-wrap`, `hyphens`. |
| Generated content and lists | `content`, `quotes`, `counter-reset`, `counter-increment`, `counter-set`, `list-style` and its longhands. |
| Effects | `isolation`, `mask` and its longhands. |
| Animations | `animation` and the `animation-*` longhands; Vizia loads `@keyframes` for use through its animation API. |
| Transitions | `transition-property`, `transition-duration`, `transition-delay`, `transition-timing-function`, `transition-behavior`. The `transition` shorthand is partially supported. |

See the [property list and aliases](crates/vizia_style/src/property.rs) and
[declaration parser](crates/vizia_style/src/declaration.rs); the `background`
shorthand is handled separately from the property list.

### Selectors and pseudo-elements

- Attribute selectors such as `[disabled]` and `[type="text"]` do not match.
- Link and namespace matching are not implemented. Link pseudo-classes such
  as `:link` and `:visited` are not functional.
- Standard pseudo-classes not recognized by the selector parser, such as
  `:target` and `:fullscreen`, are rejected in ordinary selectors.
- Pseudo-elements such as `::before`, `::after`, and `::selection` parse but
  do not match views or create generated content. Vizia provides the
  nonstandard `selection-color` property instead.

See the [selector parser](crates/vizia_style/src/selector.rs) and
[view-tree matcher](crates/vizia_core/src/systems/style.rs).

### Rules and other features

- Only `@keyframes` is implemented as a stylesheet at-rule. Missing examples
  include `@media`, `@supports`, `@import`, `@font-face`, `@layer`,
  `@container`, `@scope`, `@property`, `@namespace`, and `@page`.
- `!important` declarations and nested style rules are parsed, but the theme
  loader does not apply them.
- General custom-property token substitution and compound `var()`
  substitutions are not implemented; see [Variables and cascade](#variables-and-cascade).
- Background image functions such as `conic-gradient()`,
  `repeating-linear-gradient()`, `repeating-radial-gradient()`, and
  `image-set()` are not implemented.
- `steps()` timing functions are not implemented.

See [rule parsing](crates/vizia_style/src/parser/rule.rs) and
[theme loading](crates/vizia_core/src/style/mod.rs).

## Layout and positioning

| Standard CSS | Vizia |
| --- | --- |
| Browser layout modes such as `display: block`, `flex`, and `grid` | `display` accepts only `flex` and `none`. Choose the layout with `layout-type: row`, `column`, `grid`, or `overlay`; this is not the standard flexbox or grid API. |
| `flex-direction` | An alias for `layout-type`. The `row` and `column` values select Vizia's corresponding layout; `row-reverse` and `column-reverse` are not supported. The alias also accepts Vizia's nonstandard `grid` and `overlay` values. |
| `position: relative` or `absolute` | `position` is an alias for `position-type`. Only `relative` and `absolute` are supported; `static`, `fixed`, and `sticky` are not. Positioning follows Vizia layout behavior. |
| `flex-wrap` | `flex-wrap` is an alias for `wrap`, accepting standard `wrap` and `nowrap` plus Vizia's legacy `no-wrap`. `wrap-reverse` is not supported; wrapping follows Vizia layout behavior. |
| `grid-template-columns`, `grid-template-rows`, and grid placement properties | The template names alias `grid-columns` and `grid-rows`. Tracks use Vizia layout units, not the full CSS grid track syntax. Placement uses `column-start`, `column-span`, `row-start`, and `row-span`; Vizia start indices are zero-based, unlike CSS grid lines. |
| `width` and `height` | Vizia also provides nonstandard `size`, `min-size`, and `max-size` properties to set both dimensions or constraints together. |
| `padding` | Values use Vizia layout units; a single `var()` can supply one value, but mixed or multi-value variable substitution is not supported. |
| `gap`, `row-gap`, `column-gap` | Values use Vizia layout units. Mixed or multi-value `var()` substitution is not supported. Vizia also has nonstandard `min-gap`, `max-gap`, and per-axis gap constraints. |
| `overflow: visible`, `hidden`, `clip`, `scroll`, or `auto` | `overflow`, `overflow-x`, and `overflow-y` accept `visible` and `hidden`; the other listed values are not supported. |

### Layout units

Layout properties using Vizia's `Units` type, including widths, offsets,
padding, and gaps, accept the nonstandard stretch unit `s`: `width: 1s`
requests one share of remaining space. Font-relative units such as `em` and
`rem`, and viewport units, are not resolved by the layout-units parser.
Other properties use different value types, so relative units and general
`calc()` support depend on the property.

## Appearance and text

| Standard CSS | Vizia |
| --- | --- |
| Colors and alpha | Vizia stores colors in 8-bit RGBA components; alpha values are clamped and rounded to the nearest byte, so they may lose precision compared with CSS color values. |
| `border-style` | Only `none`, `solid`, `dotted`, and `dashed` are parsed; styles such as `double`, `groove`, `ridge`, `inset`, and `outset` are missing. |
| `border-radius` and `border-top-left-radius`, etc. | These alias the corresponding `corner-*` properties. Multi-value variable substitution and elliptical radii are not supported. |
| `box-shadow` | Vizia accepts some nonstandard orderings and negative blur radii. Length resolution and animation behavior are not fully equivalent to browser CSS; transitions between `none` and shadows with omitted color or `currentColor` are discrete. |
| `background` | Attachment, origin, clip, and `var()` substitution in the shorthand are not supported. Layers with unresolved length calculations in their size, position, or gradient stops are skipped with a warning. The value and rendering limitations of the component properties also apply. |
| `background-repeat` | Vizia uses its existing background bounds rather than CSS `background-origin` and `background-clip` boxes. |
| `background-position` | Edge offsets do not support relative lengths or general mixed-unit `calc()`. Vizia also accepts nonstandard two-value cases such as `top 20px`; full CSS position syntax is not supported. |
| `transform-origin` | Three- and four-value edge offsets and full CSS position syntax are not supported. |
| `background-size` | SVG intrinsic dimensions in font-relative units are not fully resolved. Gradient and SVG rendering are not fully equivalent to browser CSS. |
| `outline` | The shorthand applies color and width, but its parsed style is not applied; `outline-style` is missing. |
| `mix-blend-mode` | Vizia accepts the nonstandard `plus` value but does not recognize the standard `plus-lighter` value. |
| `font-style`, `font-stretch` | These alias `font-slant` and `font-width`. `font-style` does not accept an oblique angle. Font stretch percentages are quantized to named widths and are not scaled correctly: for example, `100%` maps to ultra-expanded rather than normal. |
| `font-size` | Percentages and the relative keywords `smaller` and `larger` are not parsed. Length resolution is limited by Vizia's text rendering support. |
| `font-weight` | Numeric weights are parsed as integers without enforcing the standard 1-1000 range; CSS also permits fractional values. |
| `letter-spacing` | Unresolved length calculations are not accepted; length resolution is limited by Vizia's text rendering support. |
| `text-wrap` CSS keywords | `wrap` and `nowrap` map to Vizia's text-wrapping behavior. Vizia also accepts legacy boolean values such as `false`; advanced CSS wrapping modes are not supported. |
| `filter` and `backdrop-filter` | Only `blur()` is parsed; other standard filter functions are not supported. |
| Gradients | Only linear and radial gradients are parsed. Radial shape and size syntax and gradient color hints are not implemented. |
| Text decoration thickness properties | Vizia parses `underline-thickness`, `overline-thickness`, and `strikethrough-thickness`, but the current stylesheet property application does not apply them. |

See the individual [value parsers](crates/vizia_style/src/values/) for details.

## Selectors

- Type selectors match Vizia view element names rather than HTML elements.
  Parent and sibling matching uses the layout tree.
- `:root` matches Vizia's root entity, not every window. Use the `window`
  selector to style windows.
- Vizia provides the nonstandard `:over` state selector. State availability
  depends on the view.
- For `direction: auto`, `:dir()` uses direction resolved from the view's own
  text, not descendant text. `:lang()` uses the application locale; per-view
  language overrides and the language of translated text are not tracked.

See the [selector parser](crates/vizia_style/src/selector.rs) and
[view-tree matcher](crates/vizia_core/src/systems/style.rs).

## Variables and cascade

- Custom properties and `var()` are limited to selected property/value types
  and typed fallbacks. This is not general CSS token substitution: arbitrary
  substitutions inside compound values, including `padding`, `gap`, and
  `background`, are not supported.
- Inline Rust style modifiers override shared stylesheet styling.
- `!important` is parsed into a separate declaration list, but the current
  theme loader applies only ordinary declarations. Do not use `!important` to
  override a rule: the important declaration itself is not applied.
- Nested style rules are represented by the parser, but the theme loader does
  not apply their nested rule lists. Write selectors as separate top-level
  rules instead.
- The theme loader uses error recovery; unsupported syntax should not be
  assumed to fail like it would in a browser.

See [declaration parsing](crates/vizia_style/src/declaration.rs),
[rule parsing](crates/vizia_style/src/parser/rule.rs), and
[theme loading and variable application](crates/vizia_core/src/style/mod.rs).

## Animations and transitions

- `transition` requires an explicit property and duration, unlike standard CSS
  shorthand defaults. Transitions are implemented for selected properties,
  not all CSS properties.

See the [transition parser](crates/vizia_style/src/values/transition.rs),
[easing parser](crates/vizia_style/src/values/easing.rs), and
[animation API](crates/vizia_core/src/context/event.rs).
