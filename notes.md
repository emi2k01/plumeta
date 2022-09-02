# Plumeta

- Performant:
  Must not do unnecessary computations at runtime.

- Reusable:
  Users must be able to easily create reusable styles and allow other users to override them.

- First-class responsive styles:
  Media queries must be written per style rule.

## Example

```plum
@Value(margin, padding, gap) {
  sm: 0.25rem,
  md: 0.5rem,
}

@Group {
  centerContent: {
    display: grid,
    placeItems: center,
  }
}

Menu {
  @Alias {
    dir: flexDirection,
    py: [padding-top, padding-bottom],
    px: [padding-left, padding-right],
  },

  @Props {
      spacing: unit(padding-top),
  },

  d: flex,
  dir: column,
  gap: spacing,

  Target {
      cursor: pointer,
  },

  Dropdown {
      px: md,
      py: sm,
      ...centerContent
      (ctx.vp.width < 200px) {
          width: 100%;
      }
  }
}
```

This would get transpiled to optimized CSS and a component would be generated.
This component could be for React, Svelte or any other framework (even in other languages).

## Implementation

The implementation will be divided in phases. This is only to have some kind of timeline while implementing
Plumeta.

### Phase 1

- Parse:
  - Component selector (`Menu {}`).
  - Arbitrary selectors: `&:anything(up-to-the)leftBrace {`.
  - Arbitrary properties with arbitrary values (`padding: rgb(10, 20, 30);`).

### Phase 2

- Parse:
  - Rule conditions (`(ctx.vp.width > 200px)`).
- Generate:
  - React component without props.
  - CSS styles.

### Phase 3

- Parse:
  - Component props (`@Props {}`).
- Generate:
  - React component with untyped props.

### Phase 4

- Typecheck:
  - CSS props; that they exist and that their value is valid.
  - Selectors.
  - @Props.
- Generate:
  - React component with typed props.

### Phase 5

- Parse:
  - @Value, @Alias and @Group.
- Typecheck:
  - @Value, @Alias and @Group.
