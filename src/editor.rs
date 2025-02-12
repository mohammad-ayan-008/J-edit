use std::io::{stdout, Write};
use event::{Event,KeyCode};
use crossterm::{
    cursor,
    event::{self, read},
    style, terminal, ExecutableCommand, QueueableCommand,
};

enum Action{
    QUIT,
    MOVEUP,
    MOVEDOWN,
    MOVELEFT,
    MOVERIGHT,
    EntreMode(Mode),
    AddChar(char)
}
enum Mode{
    Normal,
    Insert
}
pub struct Editor{
    stdout: std::io::Stdout,
    size:(u16,u16),
    cx:u16,
    cy:u16,
    mode:Mode
}

impl Drop for Editor{
    fn drop(&mut self) {
        self.stdout.flush().unwrap();
        self.stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout= stdout();
        terminal::enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(Editor {
            stdout,
            size : terminal::size()?,
            cx: 0,
            cy: 0,
            mode: Mode::Normal,
        })
    }


    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_statusline()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;
        Ok(())
    }
    pub fn draw_statusline(&mut self)->anyhow::Result<()>{
        self.stdout.queue(cursor::MoveTo(0,self.size.1-2))?;
        self.stdout.queue(style::Print("StatusLine"))?;
        self.stdout.flush()?;
        Ok(())
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;
            if let Some(action) = self.handle_event(read()?)? {
                match action {
                    Action::QUIT => {
                        break;
                    },
                    Action::MOVEUP => {
                        self.cy = self.cy.saturating_sub(1);
                    },
                    Action::MOVEDOWN => {
                        self.cy += 1u16;
                    },
                    Action::MOVELEFT => {
                        self.cx = self.cx.saturating_sub(1);
                    },
                    Action::MOVERIGHT => {
                        self.cx += 1u16;
                    },
                    Action::EntreMode(new_mode) => {
                        self.mode = new_mode;
                    },
                    Action::AddChar(cr) => {
                        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
                        self.stdout.queue(style::Print(cr))?;
                        self.cx += 1;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn handle_event(&mut self, ev: Event) -> anyhow::Result<Option<Action>> {
        match self.mode {
            Mode::Normal => self.handle_normal_event(ev),
            Mode::Insert => self.handle_insert_event(ev)
        }
    }

    pub fn handle_normal_event(&self,event: Event) -> anyhow::Result<Option<Action>> {
        let action = match event {
            Event::Key(ev) => match ev.code {
                KeyCode::Char('q') => Some(Action::QUIT),
                KeyCode::Char('i') => Some(Action::EntreMode(Mode::Insert)),
                KeyCode::Up => Some(Action::MOVEUP),
                KeyCode::Down => Some(Action::MOVEDOWN),
                KeyCode::Left => Some(Action::MOVELEFT),
                KeyCode::Right => Some(Action::MOVERIGHT),

                _ => None
            }
            _ => None
        };
        Ok(action)
    }
    pub fn handle_insert_event(&self,event: Event) -> anyhow::Result<Option<Action>> {
        let action = match event {
            Event::Key(ev) => match ev.code {
                KeyCode::Esc => Some(Action::EntreMode(Mode::Normal)),
                KeyCode::Char(c) => Some(Action::AddChar(c)),
                _ => None,
            },
            _ => None,
        };
        Ok(action)
    }
}