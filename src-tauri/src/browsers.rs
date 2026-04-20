use std::{env, path::PathBuf};

use crate::models::BrowserDefinition;

pub fn browser_definitions() -> Vec<BrowserDefinition> {
    #[cfg(target_os = "macos")]
    {
        return vec![
            BrowserDefinition {
                id: "chrome",
                name: "Google Chrome",
                local_app_data_segments: &["Google", "Chrome"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                )],
            },
            BrowserDefinition {
                id: "edge",
                name: "Microsoft Edge",
                local_app_data_segments: &["Microsoft Edge"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
                )],
            },
            BrowserDefinition {
                id: "brave",
                name: "Brave",
                local_app_data_segments: &["BraveSoftware", "Brave-Browser"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
                )],
            },
            BrowserDefinition {
                id: "vivaldi",
                name: "Vivaldi",
                local_app_data_segments: &["Vivaldi"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Vivaldi.app/Contents/MacOS/Vivaldi",
                )],
            },
            BrowserDefinition {
                id: "yandex",
                name: "Yandex Browser",
                local_app_data_segments: &["Yandex", "YandexBrowser"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Yandex.app/Contents/MacOS/Yandex",
                )],
            },
            BrowserDefinition {
                id: "chromium",
                name: "Chromium",
                local_app_data_segments: &["Chromium"],
                executable_candidates: &[ExecutableCandidate::Absolute(
                    "/Applications/Chromium.app/Contents/MacOS/Chromium",
                )],
            },
        ];
    }

    vec![
        BrowserDefinition {
            id: "chrome",
            name: "Google Chrome",
            local_app_data_segments: &["Google", "Chrome", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::ProgramFiles(&[
                    "Google",
                    "Chrome",
                    "Application",
                    "chrome.exe",
                ]),
                ExecutableCandidate::ProgramFilesX86(&[
                    "Google",
                    "Chrome",
                    "Application",
                    "chrome.exe",
                ]),
                ExecutableCandidate::LocalAppData(&[
                    "Google",
                    "Chrome",
                    "Application",
                    "chrome.exe",
                ]),
            ],
        },
        BrowserDefinition {
            id: "edge",
            name: "Microsoft Edge",
            local_app_data_segments: &["Microsoft", "Edge", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::ProgramFilesX86(&[
                    "Microsoft",
                    "Edge",
                    "Application",
                    "msedge.exe",
                ]),
                ExecutableCandidate::ProgramFiles(&[
                    "Microsoft",
                    "Edge",
                    "Application",
                    "msedge.exe",
                ]),
            ],
        },
        BrowserDefinition {
            id: "brave",
            name: "Brave",
            local_app_data_segments: &["BraveSoftware", "Brave-Browser", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::ProgramFiles(&[
                    "BraveSoftware",
                    "Brave-Browser",
                    "Application",
                    "brave.exe",
                ]),
                ExecutableCandidate::ProgramFilesX86(&[
                    "BraveSoftware",
                    "Brave-Browser",
                    "Application",
                    "brave.exe",
                ]),
                ExecutableCandidate::LocalAppData(&[
                    "BraveSoftware",
                    "Brave-Browser",
                    "Application",
                    "brave.exe",
                ]),
            ],
        },
        BrowserDefinition {
            id: "vivaldi",
            name: "Vivaldi",
            local_app_data_segments: &["Vivaldi", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::LocalAppData(&["Vivaldi", "Application", "vivaldi.exe"]),
                ExecutableCandidate::ProgramFiles(&["Vivaldi", "Application", "vivaldi.exe"]),
            ],
        },
        BrowserDefinition {
            id: "yandex",
            name: "Yandex Browser",
            local_app_data_segments: &["Yandex", "YandexBrowser", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::LocalAppData(&[
                    "Yandex",
                    "YandexBrowser",
                    "Application",
                    "browser.exe",
                ]),
                ExecutableCandidate::ProgramFiles(&[
                    "Yandex",
                    "YandexBrowser",
                    "Application",
                    "browser.exe",
                ]),
            ],
        },
        BrowserDefinition {
            id: "chromium",
            name: "Chromium",
            local_app_data_segments: &["Chromium", "User Data"],
            executable_candidates: &[
                ExecutableCandidate::LocalAppData(&["Chromium", "Application", "chrome.exe"]),
                ExecutableCandidate::ProgramFiles(&["Chromium", "Application", "chrome.exe"]),
            ],
        },
    ]
}

pub fn browser_definition_by_id(browser_id: &str) -> Option<BrowserDefinition> {
    browser_definitions()
        .into_iter()
        .find(|definition| definition.id == browser_id)
}

pub fn resolve_browser_executable(browser_id: &str) -> Option<PathBuf> {
    let definition = browser_definition_by_id(browser_id)?;
    definition
        .executable_candidates
        .iter()
        .filter_map(resolve_executable_candidate)
        .find(|path| path.is_file())
}

fn resolve_executable_candidate(candidate: &ExecutableCandidate) -> Option<PathBuf> {
    match candidate {
        ExecutableCandidate::ProgramFiles(segments) => env::var_os("ProgramFiles")
            .map(PathBuf::from)
            .map(|root| join_segments(root, segments)),
        ExecutableCandidate::ProgramFilesX86(segments) => env::var_os("ProgramFiles(x86)")
            .map(PathBuf::from)
            .map(|root| join_segments(root, segments)),
        ExecutableCandidate::LocalAppData(segments) => env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|root| join_segments(root, segments)),
        #[cfg(target_os = "macos")]
        ExecutableCandidate::Absolute(path) => Some(PathBuf::from(path)),
    }
}

fn join_segments(mut root: PathBuf, segments: &[&str]) -> PathBuf {
    for segment in segments {
        root.push(segment);
    }
    root
}

pub enum ExecutableCandidate {
    ProgramFiles(&'static [&'static str]),
    ProgramFilesX86(&'static [&'static str]),
    LocalAppData(&'static [&'static str]),
    #[cfg(target_os = "macos")]
    Absolute(&'static str),
}
