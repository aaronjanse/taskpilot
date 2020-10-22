use std::env;

use rusqlite::{params, Connection, Result};


use prettytable::format;
use prettytable::{Cell, Row, Table};

use chrono::offset::Local;
use chrono::{DateTime};

#[derive(Debug)]
struct Task {
    id: i64,
    desc: String,
    due: DateTime<Local>,
}

// mod help;
mod time;

fn main() {
    let db = Connection::open("/home/ajanse/.due.db").unwrap();

    if let Err(_) = db.execute(
        "CREATE TABLE tasks (
         id    INTEGER PRIMARY KEY,
         desc  TEXT NOT NULL,
         due   DATE)", params![],
    ) {}

    if env::args().any(|e| e == "@") {
        let mut past_sep = false;
        let mut desc: String = "".to_string();
        let mut when: String = "".to_string();

        let mut args = env::args();
        args.next();
        for arg in args {
            if arg == "@" {
                past_sep = true;
                continue;
            }
            if past_sep {
                when.push_str(&arg);
                when.push_str(" ");
            } else {
                desc.push_str(&arg);
                desc.push_str(" ");
            }
        }

        let due = match time::parse_time(&when) {
            Ok(x) => x,
            Err(s) => {
                println!("Couldn't parse timestamp `{}`.", when);
                println!("tip: [time...] [date...]");
                println!("error: {}", s);
                return;
            }
        };

        println!("Title: {}", desc);
        println!("Deadline: {}", due.format("%c").to_string());
        println!("Distance: {}", time::format_relative_time(due));

        db.execute(
            "INSERT INTO tasks (desc, due) VALUES (?1, ?2)",
            params![desc, due],
        ).unwrap();
    } else if env::args().nth(1).unwrap_or("".to_string()) == "done" {
        let id = env::args().nth(2).unwrap().parse::<i32>().unwrap();
        db.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![id],
        ).unwrap();
    } else if env::args().nth(1).unwrap_or("".to_string()) == "when" {
        let when = env::args().collect::<Vec<String>>()[2..].join(" ");
        let due = match time::parse_time(&when) {
            Ok(x) => x,
            Err(s) => {
                println!("Couldn't parse timestamp `{}`.", when);
                println!("tip: [time...] [date...]");
                println!("error: {}", s);
                return;
            }
        };
        println!("due: {:?}", due.format("%c").to_string());
    } else {
        let mut tasks: Vec<Task> = db
            .prepare("SELECT id, desc, due FROM tasks")
            .unwrap()
            .query_map(params![], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    desc: row.get(1)?,
                    due: row.get(2)?,
                })
            })
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        
        tasks.sort_by(|a, b| a.due.partial_cmp(&b.due).unwrap());

        let mut table = Table::new();
        table.set_titles(Row::new(vec![
            Cell::new("ID").style_spec("bFc"),
            Cell::new("Due").style_spec("bFc").with_hspan(2),
            Cell::new("Desc").style_spec("bFc"),
        ]));

        let now = chrono::offset::Local::now();

        for task in tasks {
            let colorize = |c: Cell| if now > task.due { c.style_spec("Fr") } else { c };
            let id = colorize(Cell::new(&format!("{}", task.id)));
            let dist = colorize(Cell::new(&time::format_time_distance(task.due)));
            let due = colorize(Cell::new(&time::format_relative_time(task.due)));
            let desc = colorize(Cell::new(&task.desc));
            table.add_row(Row::new(vec![id, dist, due, desc]));
        }

        table.set_format(
            format::FormatBuilder::new()
                .column_separator(' ')
                .padding(0, 1)
                .build(),
        );
        table.printstd();
    }

    // let args_vec: Vec<String> = env::args().collect();
    // println!("{:?}", args_vec);

    // println!("{}", help::HELP_MAIN)
}
