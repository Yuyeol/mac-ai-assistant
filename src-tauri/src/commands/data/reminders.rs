// Apple Reminders 읽기
// osascript로 미완료 미리알림을 가져와서 React로 반환.

use crate::models::Reminder;
use std::process::Command;

#[tauri::command]
pub fn get_reminders() -> Result<Vec<Reminder>, String> {
    let script = r####"
set output to ""
tell application "Reminders"
    set allLists to lists
    repeat with aList in allLists
        set allReminders to reminders of aList
        repeat with r in allReminders
            if not completed of r then
                set rName to name of r
                set rDue to ""
                try
                    set rDue to due date of r as string
                end try
                set rNotes to ""
                try
                    set rNotes to body of r
                end try
                set output to output & rName & "|||" & rDue & "|||" & rNotes & "###"
            end if
        end repeat
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
    let mut reminders = Vec::new();

    for entry in stdout.split("###") {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let parts: Vec<&str> = entry.splitn(3, "|||").collect();
        if !parts.is_empty() && !parts[0].trim().is_empty() {
            reminders.push(Reminder {
                name: parts[0].trim().to_string(),
                due_date: parts
                    .get(1)
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string()),
                notes: parts
                    .get(2)
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string()),
            });
        }
    }

    Ok(reminders)
}
