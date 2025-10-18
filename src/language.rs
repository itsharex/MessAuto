use sys_locale::get_locale;

const AVAILABLE_LOCALES: &[&str] = &["en", "zh-CN"];

pub fn detect_system_locale() -> String {
    match get_locale() {
        Some(locale) => {
            eprintln!("System locale detected: {}", locale);

            if AVAILABLE_LOCALES.contains(&locale.as_str()) {
                eprintln!("Exact match found: {}", locale);
                return locale;
            }

            let lang_code = if let Some(pos) = locale.find('-') {
                &locale[..pos]
            } else {
                locale.as_str()
            };

            eprintln!("Language code extracted: {}", lang_code);

            for &available_locale in AVAILABLE_LOCALES {
                if available_locale.starts_with(lang_code) {
                    eprintln!(
                        "Language code match found: {} -> {}",
                        locale, available_locale
                    );
                    return available_locale.to_string();
                }
            }

            match lang_code {
                "zh" => {
                    eprintln!("Chinese language detected, defaulting to zh-CN");
                    "zh-CN".to_string()
                }
                "en" => {
                    eprintln!("English language detected");
                    "en".to_string()
                }
                _ => {
                    eprintln!("Unsupported language, defaulting to en");
                    "en".to_string()
                }
            }
        }
        None => {
            eprintln!("Could not determine system locale, defaulting to en");
            "en".to_string()
        }
    }
}
