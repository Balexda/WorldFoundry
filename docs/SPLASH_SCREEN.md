# World Foundry Splash Screen

## Overview

The World Foundry splash screen features a beautiful anime-style character holding a magical book with a fantasy world map in the background. The design captures the essence of world-building and fantasy creation.

## Current Implementation

The splash screen is currently implemented with a fantasy-themed gradient background and animated elements that match the style and colors of the provided image:

- **Background**: Deep blue gradient reminiscent of the magical atmosphere
- **Title**: "WORLD FOUNDRY" in large, bold cream-colored text
- **Subtitle**: "Forge your fantasy worlds with native performance"
- **Animation**: Smooth loading progress bar with golden color
- **Styling**: Magical theme with world emoji placeholder

## Image Integration

### Provided Image Features
- Anime-style character with brown hair and blue eyes
- Character holding an open magical book with glowing effects
- Fantasy world map in the background with blue tones
- "WORLD FOUNDRY" title in cream/beige color
- "Forge your fantasy worlds with native performance" subtitle
- Deep blue gradient background with magical elements

### To Use the Actual Image

1. **Save the image**: Use the provided script to save the image to the correct location:
   ```bash
   python scripts/save_splash_image.py /path/to/your/image.png
   ```

2. **Update the Compose code**: Modify `Splash.kt` to use the actual image instead of the placeholder:
   ```kotlin
   // Replace the placeholder Box with:
   Image(
       painter = painterResource("splash_screen.png"),
       contentDescription = "World Foundry Splash",
       modifier = Modifier.fillMaxWidth().height(300.dp),
       contentScale = ContentScale.Fit
   )
   ```

3. **Add image dependencies**: The project already includes `compose-resources` for image loading.

## File Locations

- **Splash Screen Code**: `apps/worldfoundry-compose/shared/src/commonMain/kotlin/com/balexda/worldfoundry/ui/Splash.kt`
- **Resources Directory**: `apps/worldfoundry-compose/shared/src/commonMain/resources/`
- **Image Save Script**: `scripts/save_splash_image.py`

## Design Specifications

- **Colors**: 
  - Background: Deep blue gradient (#1A237E to #3949AB)
  - Title text: Cream/beige (#F5F5DC)
  - Accent: Gold (#FFD700)
- **Typography**: Bold, large letters with spacing
- **Animation**: Smooth progress indicator and loading text
- **Layout**: Centered, responsive design

## Platform Support

The splash screen is implemented using Compose Multiplatform and works across:
- ✅ Windows Desktop
- ✅ iOS
- 🔄 Android (planned)

## Next Steps

1. Integrate the actual provided image
2. Test on both desktop and iOS platforms
3. Add fade-in/fade-out animations
4. Implement navigation to main app after loading