# Apo

> Now, Learning Rust Languages.
> It is not complete first missions.
> Now that I'm not done with My Main Mission(Research at Univ.)
> If it calm down research, I want to resume this work.

termBased Schedule Manager with git-based sync and Simple editable.

## Concept

- rich CLI
- git-sync
- git-branches is another calendar (easy to merge)
- push to remote
- CLI-One-Line editable.
- VIM (or env:EDTIOR) editable.
- manage todays doing 

### Inspired

- `dstask`
- `pal`
- `todo.txt`
- `crond`

## Todo

- colornize and rich CLI
- git sync imp
- git branches management

## Future

Porting to www service.


## Writable Syntax in File

```
%WORK

2020-10-10 110 Come CEO
2020-10-10 100 Party

2020-10-10 4:40 !S = Wake Up
           5:00 S = Walking
           5:30 S = Morning Coffee Time
           5:40 R = Task Management
           5:50 R = Mail Check
           6:20 S = Preparation for work
           6:30 R = To the station

           6:48:40 !D = JR Soubu-Rapid-Line bound for Ohfuna

2020-10-11 17:20 310 Deadline for abstract manuscripts


```
Syntax: `[TIME] [FLAGS] [DESCRIPTION]`

### Syntaxes

- One Apointment - One LINE

```
[DATE] [WEEK] [TIME] [FLAG] [DESCRIPT]

[DATE] [WEEK]
  [TIME] [FLAG] [DESC]
  [TIME] [FLAG] [DESC]

```

#### DATE

- `YYYY-MM-DD`

#### WEEK

- `0-7`

#### TIME

- `HH:MM(:SS)

##### FUTURE/THINKING

- META(Next,Day,Night)

### FLAG

FLAG|MEAN|
----|----|
`S`|Scheduled
`R`|Reminder
`D`|DeadLine
`!`|Important
`&`|Background

### DESC

- `@you` Mension(e.g. Partcipant)
- `$val  Variable?

## dependencies

- clap = "2.33.3"
- chrono = "0.4.6"
