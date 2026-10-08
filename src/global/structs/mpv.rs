use std::{process::Command, sync::mpsc, thread};

use libmpv::Mpv;
use typemap::Key;

pub struct MpvWrapper {
    pub sender: mpsc::Sender<MpvAction>,
    // pub copier: mpsc::Receiver<MpvResponse>,
}

impl Clone for MpvWrapper {
    fn clone(&self) -> Self {
        panic!("no")
    }
}

// impl Default for MpvWrapper {
//     fn default() -> Self {
//         Self::spawn()
//     }
// }

impl MpvWrapper {
    pub fn spawn(youtube_downloader: String) -> Self {
        let (sender, receiver) = mpsc::channel();
        // let (responder, copier) = mpsc::channel();
        thread::spawn(move || {
            let mpv = Mpv::new().unwrap();
            mpv.set_property("video", "no").unwrap();
            loop {
                let action = match receiver.recv() {
                    Ok(res) => res,
                    Err(_) => continue,
                };
                match action {
                    MpvAction::Command {
                        name,
                        args,
                        responder,
                    } => match prepare_command(&youtube_downloader, name, args) {
                        Ok((name, args)) => match mpv
                            .command(&name, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                        {
                            Ok(_) => responder.send(MpvResponse::Copy).unwrap(),
                            Err(e) => responder.send(MpvResponse::Error(e.to_string())).unwrap(),
                        },
                        Err(e) => responder.send(MpvResponse::Error(e)).unwrap(),
                    },
                    MpvAction::GetProperty { name, responder } => responder
                        .send(MpvResponse::Property(
                            mpv.get_property::<String>(&name).ok(),
                        ))
                        .unwrap(),
                    MpvAction::SetProperty {
                        name,
                        value,
                        responder,
                    } => responder
                        .send(match mpv.set_property(&name, value) {
                            Ok(_) => MpvResponse::Copy,
                            Err(e) => MpvResponse::Error(e.to_string()),
                        })
                        .unwrap(),
                }
            }
        });
        Self { sender }
    }

    pub fn command(&self, name: String, args: Vec<String>) -> MpvResponse {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::Command {
                name,
                args,
                responder: tx,
            })
            .unwrap();

        rx.recv().unwrap()
    }

    pub fn property(&self, prop: String) -> Option<String> {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::GetProperty {
                name: prop,
                responder: tx,
            })
            .unwrap();

        if let MpvResponse::Property(p) = rx.recv().unwrap() {
            p
        } else {
            unreachable!()
        }
    }

    pub fn set_property(&self, name: String, value: String) -> MpvResponse {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(MpvAction::SetProperty {
                name,
                value,
                responder: tx,
            })
            .unwrap();

        rx.recv().unwrap()
    }
}

impl MpvWrapper {
    pub fn playing(&self) -> bool {
        self.property("core-idle".to_string())
            .is_some_and(|s| s.as_str() == "no" || s.as_str() == "false")
    }
}

#[derive(Debug)]
pub enum MpvAction {
    Command {
        name: String,
        args: Vec<String>,
        responder: mpsc::Sender<MpvResponse>,
    },
    GetProperty {
        name: String,
        responder: mpsc::Sender<MpvResponse>,
    },
    SetProperty {
        name: String,
        value: String,
        responder: mpsc::Sender<MpvResponse>,
    },
}

pub enum MpvResponse {
    Copy,
    Property(Option<String>),
    Error(String),
}

impl Key for MpvWrapper {
    type Value = Self;
}

/// Prepares an `mpv` command for execution.
///
/// For `loadfile`, the first argument is cleaned up and, when it points at a
/// YouTube/Invidious watch page, resolved to a direct stream URL first:
/// embedded libmpv builds ship without lua, so mpv's own ytdl-hook cannot run
/// and page URLs would fail with `unrecognized file format`.
/// Everything else passes through untouched.
fn prepare_command(
    youtube_downloader: &str,
    name: String,
    mut args: Vec<String>,
) -> Result<(String, Vec<String>), String> {
    if name != "loadfile" || args.is_empty() {
        return Ok((name, args));
    }

    let url = trim_quotes(&args[0]);
    if is_watch_url(&url) {
        args[0] = resolve_media_url(youtube_downloader, &url)?;
    } else if url.len() != args[0].len() {
        // Strip the literal quotes the TUI command templates wrap URLs in,
        // libmpv would otherwise treat them as part of the path.
        args[0] = url;
    }
    Ok((name, args))
}

/// Removes one layer of surrounding single or double quotes.
fn trim_quotes(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Whether the URL points at a watch page (rather than directly at media).
fn is_watch_url(url: &str) -> bool {
    (url.starts_with("http://") || url.starts_with("https://"))
        && (url.contains("youtube.com/watch")
            || url.contains("youtube.com/embed")
            || url.contains("youtu.be/")
            || url.contains("/watch?v="))
}

/// Resolves a watch page URL to a direct stream URL using the configured
/// downloader (e.g. `yt-dlp`).
fn resolve_media_url(downloader: &str, url: &str) -> Result<String, String> {
    let output = Command::new(downloader)
        .args([
            "--no-playlist",
            "--no-warnings",
            "--socket-timeout",
            "15",
            "-f",
            "bestaudio",
            "-g",
            url,
        ])
        .output()
        .map_err(|e| format!("failed to run `{downloader}` (is it installed?): {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "`{downloader}` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let resolved = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();
    if resolved.is_empty() {
        return Err(format!("`{downloader}` returned no stream URL"));
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_matching_quotes() {
        assert_eq!(trim_quotes("\"https://youtu.be/x\""), "https://youtu.be/x");
        assert_eq!(trim_quotes("'https://youtu.be/x'"), "https://youtu.be/x");
        assert_eq!(trim_quotes("https://youtu.be/x"), "https://youtu.be/x");
        // mismatched quotes are left alone
        assert_eq!(
            trim_quotes("\"https://youtu.be/x'"),
            "\"https://youtu.be/x'"
        );
        assert_eq!(trim_quotes(""), "");
    }

    #[test]
    fn detects_watch_urls() {
        assert!(is_watch_url("https://www.youtube.com/watch?v=aqz-KE-bpKQ"));
        assert!(is_watch_url("https://youtu.be/aqz-KE-bpKQ"));
        assert!(is_watch_url("https://www.youtube.com/embed/aqz-KE-bpKQ"));
        assert!(is_watch_url("https://invidious.f5.si/watch?v=aqz-KE-bpKQ"));
        assert!(!is_watch_url(
            "https://rr4---sn-8xgp1vo.googlevideo.com/videoplayback?expire=1"
        ));
        assert!(!is_watch_url("/home/user/song.opus"));
        assert!(!is_watch_url(""));
    }

    #[test]
    fn prepare_loadfile_passes_through_non_watch_urls() {
        // direct URL, quoted by the TUI templates: quotes stripped, no resolving
        let (name, args) = prepare_command(
            "yt-dlp",
            "loadfile".to_string(),
            vec!["\"https://example.com/song.opus\"".to_string()],
        )
        .unwrap();
        assert_eq!(name, "loadfile");
        assert_eq!(args, vec!["https://example.com/song.opus".to_string()]);

        // local file: untouched
        let (name, args) = prepare_command(
            "yt-dlp",
            "loadfile".to_string(),
            vec!["/home/user/song.opus".to_string(), "append".to_string()],
        )
        .unwrap();
        assert_eq!(name, "loadfile");
        assert_eq!(
            args,
            vec!["/home/user/song.opus".to_string(), "append".to_string()]
        );

        // other commands: untouched
        let (name, args) = prepare_command("yt-dlp", "stop".to_string(), Vec::new()).unwrap();
        assert_eq!(name, "stop");
        assert!(args.is_empty());
    }

    #[test]
    fn prepare_loadfile_errors_without_downloader() {
        let err = prepare_command(
            "yt-dlp-definitely-missing-binary",
            "loadfile".to_string(),
            vec!["\"https://youtu.be/aqz-KE-bpKQ\"".to_string()],
        )
        .unwrap_err();
        assert!(err.contains("yt-dlp-definitely-missing-binary"), "{err}");
    }
}
