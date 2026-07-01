use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

use crate::content::quests::Quest;

/// Where quest exercise files are scaffolded, relative to the game's cwd.
pub const QUEST_DIR: &str = "quests";

const RUN_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT_LINES: usize = 200;

#[derive(Debug)]
pub enum Outcome {
    /// rustc rejected the file — the rune fizzled before it even sparked.
    CompileFail { stderr: String },
    /// It compiled, but the lantern/ledger/golem judged it wanting.
    TestFail { output: String },
    /// The rune took hold.
    Pass { output: String },
    /// The spell spun in circles until we gently stopped it.
    Timeout,
    /// Something outside the player's code went wrong (missing rustc, etc.).
    Error { msg: String },
}

pub fn quest_path(quest: &Quest) -> PathBuf {
    Path::new(QUEST_DIR).join(quest.file_name)
}

/// Write the quest's exercise file if it doesn't exist yet. Never overwrites:
/// the player's work is sacred.
pub fn scaffold(quest: &Quest) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(QUEST_DIR)?;
    let path = quest_path(quest);
    if !path.exists() {
        std::fs::write(&path, quest.template)?;
    }
    Ok(path)
}

/// Compile & test the quest file on a background thread; the UI keeps
/// animating and polls the receiver each tick.
pub fn cast(quest: &'static Quest) -> Receiver<Outcome> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let src = quest_path(quest);
        let outcome = if src.exists() {
            check_file(&src, &format!("quest_{:02}", quest.id))
        } else {
            Outcome::Error {
                msg: format!(
                    "The quest scroll {} has gone missing! Talk to {} again to receive a fresh one.",
                    src.display(),
                    quest.npc
                ),
            }
        };
        let _ = tx.send(outcome);
    });
    rx
}

/// Compile a single exercise file with `rustc --test` and run its tests.
/// `tag` names the output binary (so parallel checks can't collide).
pub fn check_file(src: &Path, tag: &str) -> Outcome {
    let out_dir = std::env::temp_dir().join("rune_and_road");
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        return Outcome::Error {
            msg: format!("Couldn't prepare a casting circle in the temp dir: {e}"),
        };
    }
    let bin = out_dir.join(tag);

    let compile = Command::new("rustc")
        .args(["--edition", "2024", "--test", "--color", "never", "-o"])
        .arg(&bin)
        .arg(src)
        .output();

    match compile {
        Err(e) => Outcome::Error {
            msg: format!("Couldn't summon rustc: {e}. Is the Rust toolchain on your PATH?"),
        },
        Ok(out) if !out.status.success() => Outcome::CompileFail {
            stderr: truncate(&String::from_utf8_lossy(&out.stderr)),
        },
        Ok(_) => run_tests(&bin),
    }
}

/// Run the compiled test binary with a timeout, so a `loop {}` in a player's
/// echo-rune can't hang the game.
fn run_tests(bin: &Path) -> Outcome {
    let child = Command::new(bin)
        .args(["--test-threads", "1"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            return Outcome::Error {
                msg: format!("The compiled rune wouldn't start: {e}"),
            };
        }
    };

    // Drain the pipes on side threads so a chatty test can't deadlock us.
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let out_handle = std::thread::spawn(move || read_all(stdout));
    let err_handle = std::thread::spawn(move || read_all(stderr));

    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if started.elapsed() > RUN_TIMEOUT {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Outcome::Timeout;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                return Outcome::Error {
                    msg: format!("Lost sight of the running rune: {e}"),
                };
            }
        }
    };

    let stdout = out_handle.join().unwrap_or_default();
    let stderr = err_handle.join().unwrap_or_default();
    let mut output = stdout;
    if !stderr.trim().is_empty() {
        if !output.trim().is_empty() {
            output.push('\n');
        }
        output.push_str(&stderr);
    }
    let output = truncate(&output);

    if status.success() {
        Outcome::Pass { output }
    } else {
        Outcome::TestFail { output }
    }
}

fn read_all(pipe: Option<impl Read>) -> String {
    let mut buf = String::new();
    if let Some(mut pipe) = pipe {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        buf = String::from_utf8_lossy(&bytes).into_owned();
    }
    buf
}

fn truncate(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    if lines.len() <= MAX_OUTPUT_LINES {
        s.trim_end().to_string()
    } else {
        let mut out = lines[..MAX_OUTPUT_LINES].join("\n");
        out.push_str(&format!(
            "\n… ({} more lines — the scroll ran out of room)",
            lines.len() - MAX_OUTPUT_LINES
        ));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_keeps_short_output() {
        assert_eq!(truncate("a\nb"), "a\nb");
    }

    #[test]
    fn truncate_caps_long_output() {
        let long = "x\n".repeat(500);
        let t = truncate(&long);
        assert!(t.lines().count() <= MAX_OUTPUT_LINES + 1);
        assert!(t.contains("more lines"));
    }
}
