# Taskpilot
> The task manager that embraces procrastination

Taskpilot is an opinionated command-line tool built for people who procrastinate. All tasks must have a deadline, and taskpilot makes it easy to specify the exact time of day when something is due. Because customization can distracting, Taskpilot currently has no configuration.

## Usage

```bash
$ due english essay @ friday 8 pm
Added task 17.
$ # don't forget what you're working on
$ due start 17
$ # record distractions as rewards for finishing a task
$ due after 17 - read hacker news
$ # yay!
$ due finish 17
"Important" tasks up next:
- read hacker news
```

## FAQ

### Why are `due` and `do` used interchangibly?

Strange indeed.

### Why can't list something without a due date?

If a task is not important and not time-sensitive, don't do it.

If a task is important and not time-sensitive, do it right now. If you can't do it right now, mark the deadline in taskpilot as the earliest day you can work on the task.

At least for me, many "todo items" are actually project ideas. I record these ideas in a personal `~/ideas.txt`.

