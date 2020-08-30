# Apo

termBased Schedule Manager with git-based sync and Simple editable.

## How to use.

- Implementation is not completed yet.

## Writable Syntax in File

```
ALL 110 Come CEO
NGT 100 Party

4:40 201 Wake Up
5:00 101 Walking
5:30 101 Morning Coffee Time
5:40 201 Task Management
5:50 201 Mail Check
6:20 201 Preparation for work
6:30 101 To the station
6:48 201 JR Soubu-Rapid-Line bound for Ohfuna

17:20 310 Deadline for abstract manuscripts
```

Syntax: `[TIME] [FLAGS] [DISCRIPTION]`

### FLAG

- First Degit
  - `1` ... Scheduled
  - `2` ... Reminder
  - `3` ... DeadLine
- Secound
  - `0` is Standard, `1` is Important
- Third
  - `0` is Standard, `1` is Routine/Recurse(Take the same actions as usual)

## dependencies

- clap = "2.33.3"
- chrono = "0.4.6"
