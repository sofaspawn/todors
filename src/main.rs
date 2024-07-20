use crossterm::cursor::{self, MoveTo};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Print, Color, SetForegroundColor, ResetColor};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::ExecutableCommand;
//use std::intrinsics::mir::Move;
use std::io::{stdout, Stdout};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();

    let mut todos = Vec::from(["Jumping", "eating", "drinking", "studying", "gaming"]);
    let mut done = Vec::new();

    //enabling raw mode and hiding the cursor
    let _ = enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    //getting the terminal size
    let (mut width, mut height) = terminal::size().unwrap();


    //help prompt
    let movement = "'k' and 'j' for up and down";
    let complete_task = "ENTER: mark the task as done";
    let delete_task = "BACKSPACE: delete the current task";
    let recover_task = "'r': recover task from done";
    let exit_help = "Press 'q' to exit";

    //quit state
    let mut quit = false;

    let mut curr = 0;


    let mut add = false;

    while !quit {
        //non blocking way to read keyevents
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    width = nw;
                    height = nh;
                }
                Event::Key(event) => match event.code {
                    KeyCode::Char(x) => {
                        if x == 'q' && event.modifiers == KeyModifiers::NONE {
                            quit = true;
                        }
                        if x == 'j' && event.modifiers == KeyModifiers::NONE {
                            if todos.len()==0{continue;}
                            if curr<todos.len()-1{
                                curr +=1;
                            }
                        }
                        if x == 'k' && event.modifiers == KeyModifiers::NONE {
                            if todos.len()==0{continue;}
                            if curr>0{
                                curr -=1;
                            }
                        }
                        if x == 'r' && event.modifiers == KeyModifiers::NONE {
                            if done.len()==0{continue;}
                            let task = done.pop().unwrap();
                            todos.push(task);
                        }
                        if x == 'a' && event.modifiers == KeyModifiers::NONE {
                            add = true;
                        }
                    },
                    KeyCode::Enter => {
                        if todos.len()==0{continue;}
                        let task = todos[curr];
                        todos.remove(curr);
                        curr = 0;
                        done.push(task);
                    },
                    KeyCode::Backspace => {
                        if todos.len()==0{continue;}
                        todos.remove(curr);
                        curr = 0;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        //clear the screen
        stdout.execute(Clear(ClearType::All)).unwrap();

        if add{
            add_todo(&mut stdout, height);
            //stdout.execute(MoveTo(0, 0)).unwrap();
            //stdout.execute(Print("add todo: ")).unwrap();
        }

        banner(&mut stdout, width);

        todo_head_placement(&mut stdout, width, height);
        done_head_placement(&mut stdout, width, height);

        stdout.execute(MoveTo(width/4, height/4)).unwrap();

        for (row, line) in todos.iter().enumerate(){
            let line = format!("[{i}] {line}", i=row+1);
            if curr==row{
                stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
                //stdout.execute(SetBackgroundColor(Color::White)).unwrap();
            }
            stdout.execute(MoveTo(width/4 - (line.len()/2) as u16, height/4+row as u16)).unwrap();
            stdout.execute(Print(line)).unwrap();
            stdout.execute(ResetColor).unwrap();
        }

        stdout.execute(MoveTo(width - width/4, height/4)).unwrap();

        for (row, line) in done.iter().enumerate(){
            //let line = format!("[{i}] {line}", i=row+1);
            stdout.execute(SetForegroundColor(Color::Green)).unwrap();
            stdout.execute(MoveTo(width - width/4 - (line.len()/2) as u16, height/4+row as u16)).unwrap();
            stdout.execute(Print(line)).unwrap();
            //stdout.execute(ResetColor).unwrap();
        }

        stdout.execute(ResetColor).unwrap();

        //move cursor and print the help message
        stdout
            .execute(MoveTo(width / 2 - movement.len() as u16 / 2, height-height/4))
            .unwrap();
        stdout.execute(Print(movement)).unwrap();
        stdout.execute(MoveTo(width / 2 - complete_task.len() as u16 / 2, height-height/4+2)).unwrap();
        stdout.execute(Print(complete_task)).unwrap();
        stdout.execute(MoveTo(width / 2 - delete_task.len() as u16 / 2, height-height/4+4)).unwrap();
        stdout.execute(Print(delete_task)).unwrap();
        stdout.execute(MoveTo(width / 2 - recover_task.len() as u16 / 2, height-height/4+6)).unwrap();
        stdout.execute(Print(recover_task)).unwrap();
        stdout.execute(MoveTo(width / 2 - exit_help.len() as u16 / 2, height-height/4+8)).unwrap();
        stdout.execute(Print(exit_help)).unwrap();

        //30fps rendering
        thread::sleep(Duration::from_millis(33));
    }

    //disabling raw mode
    let _ = disable_raw_mode().unwrap();

    let goodbye = "Keep up the good work (づ ◕‿◕ )づ";
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();
    stdout.execute(MoveTo(width/2-(goodbye.len()/2) as u16, height/2)).unwrap();
    stdout.execute(Print(goodbye)).unwrap();
    stdout.execute(ResetColor).unwrap();

    //clearing the terminal before exiting and some other things
    stdout.execute(MoveTo(0, height-1)).unwrap();
    stdout.execute(cursor::Show).unwrap();
}


fn todo_head_placement(stdout: &mut Stdout, width: u16, height:u16){
    let todo_head = "TODO";
    let sep = "═".repeat(todo_head.len());
    stdout.execute(MoveTo(width/4 - (todo_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(Print(todo_head)).unwrap();
    stdout.execute(MoveTo(width/4 - (todo_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(Print("\n")).unwrap();
    stdout.execute(Print(sep)).unwrap();
}

fn done_head_placement(stdout: &mut Stdout, width: u16, height:u16){
    let done_head = "DONE";
    let sep = "═".repeat(done_head.len());
    //stdout.execute(MoveTo(width/4 - (done_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(MoveTo(width - width/4 - (done_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(Print(done_head)).unwrap();
    //stdout.execute(MoveTo(width/4 - (done_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(MoveTo(width - width/4 - (done_head.len()/2) as u16, height/4-3 as u16)).unwrap();
    stdout.execute(Print("\n")).unwrap();
    stdout.execute(Print(sep)).unwrap();
}

fn banner(stdout: &mut Stdout, width:u16){
    let banner_head = "TODO LIST APPLICATION";
    stdout.execute(MoveTo(width/2 - (banner_head.len()/2) as u16, 0)).unwrap();
    stdout.execute(Print(banner_head)).unwrap();
}

fn add_todo(stdout: &mut Stdout, height:u16){
    stdout.execute(MoveTo(0, height-1)).unwrap();
    stdout.execute(Print("add todo: ")).unwrap();
    stdout.execute(MoveTo(5, height-1)).unwrap();
}
//TODO: implement add_todo function that lets the user add a task to the todo list
