// Apple Notes 읽기
// osascript로 Apple Notes 데이터를 가져와서 React로 반환.

use crate::models::Note;
use std::process::Command;

#[tauri::command]
pub fn get_notes() -> Result<Vec<Note>, String> {
    let script = r####"
set output to ""
tell application "Notes"
    repeat with n in notes
        set noteName to name of n
        set noteBody to plaintext of n
        set output to output & noteName & "|||" & noteBody & "###"
    end repeat
end tell
return output
"####;

    let result = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&result.stdout).to_string();
    let mut notes = Vec::new();

    for entry in stdout.split("###") {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let parts: Vec<&str> = entry.splitn(2, "|||").collect();
        if parts.len() == 2 {
            notes.push(Note {
                name: parts[0].trim().to_string(),
                body: parts[1].trim().to_string(),
            });
        }
    }

    Ok(notes)
}
