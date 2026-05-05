# CLI TODO SIMPLE PROJECT 

## CURRENT COMMANDS

Help -> Lists all the commands 
list | ls -> Lists all the tasks | Flags { -e, -ext, -extended => for extended list with tags}
add -> add a task
done -> mark a task completed
exit -> exit the program


## WIP

CURRENT KEYS:


MainFocus = Category ->
| Key             | Action              |
| --------------- | ------------------- |
| `q`             | Exit app            |
| `Ctrl + c`      | Clear overdue tasks |
| `Ctrl + d`      | Clear done tasks    |
| `S`             | Enter search mode   |
| `↓` / `j`       | Move down           |
| `↑` / `k`       | Move up             |
| `D`             | Delete category     |
| `a`             | Add category        |
| `s`             | Sort tasks          |
| `Tab` / `Enter` | Focus tasks         |
| `Esc`           | Exit search         |

MainFocus = CMD ->
| Key         | Action                           |
| ----------- | -------------------------------- |
| `Esc`       | Exit command mode                |
| `Char`      | Add character                    |
| `Backspace` | Delete character                 |
| `Enter`     | Execute command (mode-dependent) |
| `→`         | Move cursor right                |
| `←`         | Move cursor left                 |
MainFocus = Help ->
| Key         | Action                           |
| ----------- | -------------------------------- |
| `Esc`       | Exit command mode                |
Focus = Search ->

| Key       | Condition       | Action                                        |
| --------- | --------------- | --------------------------------------------- |
| `↓` / `j` | `SearchResults` | Next category result                          |
| `↓` / `j` | `Task`          | Next task result                              |
| `↑` / `k` | —               | Previous category                             |
| `Enter`   | `SearchResults` | Switch to Task view                           |
| `Enter`   | `Task`          | Open Details popup                            |
| `Esc`     | —               | Exit search mode, reset cmd, go to Categories |

MainMode ->
| Key   | Action                                                  |
| ----- | ------------------------------------------------------- |
| `q`   | Exit app                                                |
| `S`   | Enter search command mode                               |
| `H`   | Open Help popup                                         |
| `Tab` | Switch focus to Categories                              |
| `Esc` | Return to Categories (reset state depending on context) |

Popup Mode (NON INPUTTING MODE)
| Key         | Action                                                      |
| ----------- | ----------------------------------------------------------- |
| `Tab` / `j` | Move to next field (Title → Tags → Due → Recurring → Title) |
| `k`         | Move to previous field                                      |
| `c`         | Clear current field input                                   |
| `e` / `i`   | Enter input mode                                            |
| `→`         | Move cursor right                                           |
| `←`         | Move cursor left                                            |
| `Enter`     | Submit task (if title not empty)                            |
| `q` / `Esc` | Close popup                                                 |

Popup MOde (Inputting Mode)
| Key         | Action                                                      |
| ----------- | ----------------------------------------------------------- |
| `Esc`       | Exit input mode                                             |
| `Char`      | Insert character at cursor                                  |
| `Backspace` | Delete character before cursor                              |
| `Enter`     | Move to next field (Title → Tags → Due → Recurring → Title) |

Details Popup

| Key         | Action                                      |
| ----------- | ------------------------------------------- |
| `Esc` / `q` | Close details popup and return to task view |

MainFocus = Task =>
| Key       | Action                                               |
| --------- | ---------------------------------------------------- |
| `q`       | Exit app                                             |
| `S`       | Enter search command mode                            |
| `d`       | Enter **add description mode**                       |
| `↓` / `j` | Select next task                                     |
| `↑` / `k` | Select previous task                                 |
| `Delete`  | Delete selected task                                 |
| `H`       | Open Help popup                                      |
| `a`       | Open **Add Task popup** (new task)                   |
| `e`       | Edit selected task (opens popup with prefilled data) |
| `Enter`   | Open **Details popup**                               |
| `Esc`     | Return to Categories view                            |
| `x`       | Mark task as **completed**                           |
| `p`       | Mark task as **in progress**                         |
| `Tab`     | Switch to Categories view                            |




