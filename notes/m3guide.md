1. The Core Prefix & Suffix Rules

Before looking at individual tokens, you can master the naming convention with four simple formulas:
• [Key]: The baseline fill color for high-emphasis elements (e.g., primary, error).
• on[Key]: The text or icon color designed to sit safely on top of its matching base color, ensuring readable contrast (e.g., onPrimary goes on top of primary).
• [Key]Container: A lower-intensity standalone fill color meant for block elements like cards, buttons, or list items (e.g., primaryContainer).
• on[Key]Container: The text or icon color meant to sit on top of the container color (e.g., onPrimaryContainer).

2. Accent & Utility Color Groups

These elements drive the core branding, actions, and states of your application interface.

Base Color	On Color	Container	On Container	Purpose & Relationship
primary	onPrimary	primaryContainer	onPrimaryContainer	Main brand color; used for prominent components like Floating Action Buttons (FABs) and key active states.
secondary	onSecondary	secondaryContainer	onSecondaryContainer	Less prominent accents; used for minor UI pieces like filter chips or selection controls.
tertiary	onTertiary	tertiaryContainer	onTertiaryContainer	Contrasting balance; neutralizes primary/secondary saturation or brings emphasis to input fields and alerts.
error	onError	errorContainer	onErrorContainer	Urgency states; universally maps out validation issues, destructive actions, or system failures.


3. Surfaces & Structural Hierarchy

Material 3 replaced traditional drop shadows with color-based tonal shifts to indicate depth and hierarchy.

The Main Backgrounds & Utility Roles

• background / onBackground: The absolute default underlying canvas behind scrollable views.
• surface / onSurface: The baseline color for standalone structural blocks (like sheets, dialogs, and cards).
• surfaceVariant / onSurfaceVariant: A subtle color variation providing distinct contrast boundaries against the base surface.
• outline: High-contrast border lines or decorative dividers.
• outlineVariant: Lower-contrast dividers for clean, minimal spacing.
• shadow / scrim: Overlays meant to block background interaction or anchor legacy elevated components.
• surfaceTint: A transparent color layer calculated from the primary scheme applied over surfaces to indicate altitude shifts.

The 5-Tier Surface Containers

Instead of creating arbitrary colors, M3 provides a strict stacked scale for containers based on visual prominence:
• surfaceContainerLowest → surfaceContainerLow → surfaceContainer → surfaceContainerHigh → surfaceContainerHighest
• Relationship: Lowest is typically closest to white (in light mode) or pure black (in dark mode), whereas Highest pops forward with the highest relative emphasis.

Adaptive Surface Additions

• surfaceDim / surfaceBright: Fixed variants that preserve a specific contrast standard regardless of surrounding ambient light conditions or theme mode shifts.

4. Inverse & Fixed Schemes

These colors exist to maintain visual stability when building cross-thematic elements or switching device modes.

Inverse Elements

Used to display small high-contrast snippets (like dark Snackbars/Toasts appearing inside a light-themed screen).
• inverseSurface: Flippantly flips the scheme (e.g., uses dark color in a light theme).
• inverseOnSurface: Text/icons that print cleanly over the inverseSurface.
• inversePrimary: An alternate primary color used explicitly on top of inverted surfaces.

Fixed Accent Elements

These colors never change their structural lightness or darkness values, remaining completely constant whether the phone is toggled into Dark Mode or Light Mode. They are crucial for consistent accenting in hybrid layouts.
• Primary: primaryFixed | onPrimaryFixed | primaryFixedDim | onPrimaryFixedVariant
• Secondary: secondaryFixed | onSecondaryFixed | secondaryFixedDim | onSecondaryFixedVariant
• Tertiary: tertiaryFixed | onTertiaryFixed | tertiaryFixedDim | onTertiaryFixedVariant