use std::io::{stdout, Stdout, Write};
use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::EnterAlternateScreen;

enum Action{
  QUIT,
  MOVEUP,
  MOVEDOWN,
  MOVELEFT,
  MOVERIGHT,
  EntreMode(Mode)
}
enum Mode{
  Normal,
  Insert
}
  fn handle_event(cx: &mut u16,stdout: &mut Stdout,mode: &Mode,ev:crossterm::event::Event)->anyhow::Result<Option<Action>>{
   match mode {
     Mode::Normal => {handle_normal_event(ev)}
     Mode::Insert => {handle_insert_event(cx,stdout,ev)}
   }
  }

fn handle_normal_event(event: Event)->anyhow::Result<Option<Action>>{
 match event {
   Event::Key(ev) => match ev.code {
     KeyCode::Char('q')=> { Ok(Some(Action::QUIT))}
     KeyCode::Char('i') => Ok(Some(Action::EntreMode(Mode::Insert))),
     KeyCode::Up=>  Ok(Some(Action::MOVEUP)),
     KeyCode::Down  =>  Ok(Some(Action::MOVEDOWN)),
     KeyCode::Left=>  Ok(Some(Action::MOVELEFT)),
     KeyCode::Right =>  Ok(Some(Action::MOVERIGHT)),

     _=>{Ok(None)}
   }
   _=>Ok(None)
 }
}
fn handle_insert_event(cx: &mut u16,stdout: &mut Stdout, event: Event) ->anyhow::Result<Option<Action>>{
  match event {
    Event::Key(ev) => match ev.code {
      KeyCode::Esc => Ok(Some(Action::EntreMode(Mode::Normal))),
      KeyCode::Char(c) => {
        stdout.queue(style::Print(c))?;
        *cx += 1;
        Ok(None)
      }
      _ => Ok(None),
    },
    _ => Ok(None),
  }

}



fn main() -> anyhow::Result<()>{
  let mut stdout = stdout();
  let mut  cx = 0;
  let mut  cy = 0;
  let mut mode = Mode::Normal;
  terminal::enable_raw_mode()?;
  stdout.execute(EnterAlternateScreen)?;
  stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  loop {
    stdout.queue(cursor::MoveTo(cx,cy))?;
    stdout.flush()?;
    if let Some(action) =  handle_event(&mut cx,&mut stdout,&mut mode,read()?)?{
      match action {
        Action::QUIT => {
          break;
        },
        Action::MOVEUP=> {
          cy = cy.saturating_sub(1);
        },
        Action::MOVEDOWN=>{
          cy += 1u16;
        },
        Action::MOVELEFT => {
          cx = cx.saturating_sub(1);
        },
        Action::MOVERIGHT =>{
          cx += 1u16;
        },
        Action::EntreMode(new_mode)=> {
          mode = new_mode;
        }
        _=>{}
      }
    }

  }
  stdout.execute(terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;
  Ok(())
}
