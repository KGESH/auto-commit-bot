# Git Auto Commit

---

## Description

- ### Select to project, then change README.md file.

---

## Target Project

- ### This project


---

## How to build

<pre>git clone https://github.com/KGESH/auto-commit-bot</pre>

<pre>cd auto-commit-bot</pre>

<pre>cargo build --release</pre>

---

## Run

<pre>target/release/auto-commit</pre>

---

## Sequence

### 1. Execute this binary
### 2. Check project_repo_url.txt exist in directory
### 3. If project_repo_url.txt is empty then input your git remote repository url
### 4. Clone from remote repository
### 5. Change directory to cloned repository
### 6. If README.md file is not exist then create README.md file
### 7. Open README.md file to write mode
### 8. Append string to README.md file and close file
### 9. Git add all files
### 10. Git commit -m 'something string'
### 11. Git push
### 12. Done
