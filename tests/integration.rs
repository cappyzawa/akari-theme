use akari_theme::{Generator, Palette, Variant, find_project_root};
use std::path::PathBuf;

fn palette_dir() -> PathBuf {
    find_project_root()
        .expect("project root not found")
        .join("palette")
}

fn templates_dir() -> PathBuf {
    find_project_root()
        .expect("project root not found")
        .join("templates")
}

mod palette {
    use super::*;

    #[test]
    fn load_night() {
        let path = palette_dir().join("akari-night.toml");
        let palette = Palette::from_path(&path, Variant::Night).unwrap();
        assert_eq!(palette.variant, Variant::Night);
        assert_eq!(palette.name, "akari-night");
    }

    #[test]
    fn load_dawn() {
        let path = palette_dir().join("akari-dawn.toml");
        let palette = Palette::from_path(&path, Variant::Dawn).unwrap();
        assert_eq!(palette.variant, Variant::Dawn);
        assert_eq!(palette.name, "akari-dawn");
    }

    #[test]
    fn both_palettes_have_valid_hex_colors() {
        for variant in [Variant::Night, Variant::Dawn] {
            let path = palette_dir().join(variant.palette_filename());
            let palette = Palette::from_path(&path, variant).unwrap();

            // Check base colors are valid hex
            assert!(palette.base.background.starts_with('#'));
            assert!(palette.base.foreground.starts_with('#'));
            assert_eq!(palette.base.background.len(), 7);
            assert_eq!(palette.base.foreground.len(), 7);

            // Check semantic colors are resolved to hex
            assert!(palette.semantic.keyword.starts_with('#'));
            assert!(palette.semantic.string.starts_with('#'));
            assert!(palette.semantic.function.starts_with('#'));

            // Check ANSI colors are resolved to hex
            assert!(palette.ansi.black.starts_with('#'));
            assert!(palette.ansi.white.starts_with('#'));
            assert!(palette.ansi_bright.black.starts_with('#'));
        }
    }

    #[test]
    fn embedded_night_matches_file() {
        let path = palette_dir().join("akari-night.toml");
        let from_file = Palette::from_path(&path, Variant::Night).unwrap();
        let embedded = Palette::night();

        assert_eq!(embedded.variant, from_file.variant);
        assert_eq!(embedded.name, from_file.name);
        assert_eq!(embedded.base.background, from_file.base.background);
        assert_eq!(embedded.base.foreground, from_file.base.foreground);
    }

    #[test]
    fn embedded_dawn_matches_file() {
        let path = palette_dir().join("akari-dawn.toml");
        let from_file = Palette::from_path(&path, Variant::Dawn).unwrap();
        let embedded = Palette::dawn();

        assert_eq!(embedded.variant, from_file.variant);
        assert_eq!(embedded.name, from_file.name);
        assert_eq!(embedded.base.background, from_file.base.background);
        assert_eq!(embedded.base.foreground, from_file.base.foreground);
    }
}

mod generator {
    use super::*;

    fn load_palettes() -> (Palette, Palette) {
        let dir = palette_dir();
        let night = Palette::from_path(dir.join("akari-night.toml"), Variant::Night).unwrap();
        let dawn = Palette::from_path(dir.join("akari-dawn.toml"), Variant::Dawn).unwrap();
        (night, dawn)
    }

    #[test]
    fn available_tools_not_empty() {
        let generator = Generator::new(templates_dir()).unwrap();
        let tools = generator.available_tools().unwrap();
        assert!(!tools.is_empty());
    }

    #[test]
    fn generate_helix() {
        let generator = Generator::new(templates_dir()).unwrap();
        let (night, dawn) = load_palettes();
        let artifacts = generator.generate_tool("helix", &night, &dawn).unwrap();

        assert!(!artifacts.is_empty());
        // Should have at least night and dawn themes
        let paths: Vec<_> = artifacts
            .iter()
            .map(|a| a.rel_path.display().to_string())
            .collect();
        assert!(paths.iter().any(|p| p.contains("night")));
        assert!(paths.iter().any(|p| p.contains("dawn")));
    }

    #[test]
    fn generate_all_tools() {
        let generator = Generator::new(templates_dir()).unwrap();
        let (night, dawn) = load_palettes();
        let tools = generator.available_tools().unwrap();

        for tool in tools {
            let result = generator.generate_tool(&tool, &night, &dawn);
            assert!(
                result.is_ok(),
                "failed to generate {}: {:?}",
                tool,
                result.err()
            );
        }
    }
}

mod terminal {
    use super::*;

    #[test]
    fn generate_night() {
        let path = palette_dir().join("akari-night.toml");
        let palette = Palette::from_path(&path, Variant::Night).unwrap();
        let content = akari_theme::terminal::generate(&palette).unwrap();

        assert!(content.contains("<?xml"));
        assert!(content.contains("plist"));
        assert!(content.contains("Akari-Night"));
        assert!(content.contains("ANSIBlackColor"));
        assert!(content.contains("BackgroundColor"));
    }

    #[test]
    fn generate_dawn() {
        let path = palette_dir().join("akari-dawn.toml");
        let palette = Palette::from_path(&path, Variant::Dawn).unwrap();
        let content = akari_theme::terminal::generate(&palette).unwrap();

        assert!(content.contains("Akari-Dawn"));
    }
}
