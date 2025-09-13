#!/usr/bin/env python3
"""
Script to save the World Foundry splash screen image.
This script can be used to save the provided image to the correct location.
"""

import os
import sys
import shutil
from pathlib import Path

def save_splash_image(source_path: str):
    """Save the splash screen image to the resources directory."""
    
    # Define the target path
    project_root = Path(__file__).parent.parent
    target_path = project_root / "apps" / "worldfoundry-compose" / "shared" / "src" / "commonMain" / "resources" / "splash_screen.png"
    
    # Ensure the resources directory exists
    target_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Copy the image
    if os.path.exists(source_path):
        shutil.copy2(source_path, target_path)
        print(f"✅ Splash screen image saved to: {target_path}")
        return True
    else:
        print(f"❌ Source image not found: {source_path}")
        return False

def main():
    if len(sys.argv) != 2:
        print("Usage: python save_splash_image.py <path_to_image>")
        print("Example: python save_splash_image.py /path/to/worldfoundry_splash.png")
        sys.exit(1)
    
    source_path = sys.argv[1]
    success = save_splash_image(source_path)
    
    if success:
        print("\n🎨 Next steps:")
        print("1. Update Splash.kt to use the actual image instead of the placeholder")
        print("2. Test the app to ensure the image displays correctly")
        print("3. Commit the changes")
    else:
        sys.exit(1)

if __name__ == "__main__":
    main()