# Git Auto Commit

---

## Description

- ### Select to project, then change README.md file.

---

## Target Project

- ### rust-study


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
### 3. If 'project_repo_url.txt' is not exist then input your git remote repository url
### 4. Clone from remote repository
### 5. Change directory to cloned repository
### 6. If 'auto-commit.log' file is not exist then create 'auto-commit.log' file
### 7. Open 'auto-commit.log' file to write mode
### 8. Append current date to 'auto-commit.log' file and close file
### 9. Git add all files
### 10. Git commit -m 'something string'
### 11. Git push
### 12. Remove clone repository
### 13. Done

