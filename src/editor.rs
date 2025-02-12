use std::fmt::format;
use std::io::{stdout, Write};
use event::{Event,KeyCode};
use crossterm::{
    cursor,
    event::{self, read},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use crossterm::style::{Color, Stylize};
use crossterm::terminal::ClearType;
use crate::buffer::Buffer;

enum Action{
    QUIT,
    MOVEUP,
    MOVEDOWN,
    MOVELEFT,
    MOVERIGHT,
    EntreMode(Mode),
    AddChar(char)
}
#[derive(Debug)]
enum Mode{
    Normal,
    Insert
}
pub struct Editor{
    buffer: Buffer,
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
    pub fn new(buffer:Buffer) -> anyhow::Result<Self> {
        let mut stdout= stdout();
        terminal::enable_raw_mode()?;
        stdout
            .execute(terminal::EnterAlternateScreen)?
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(Editor {
            buffer,
            stdout,
            size : terminal::size()?,
            cx: 0,
            cy: 0,
            mode: Mode::Normal,
        })
    }

    pub fn draw_buffer(&mut self)->anyhow::Result<()>{
        for (i,lines) in self.buffer.lines.iter().enumerate(){
            self.stdout.queue(cursor::MoveTo(0,i  as u16))?;
            self.stdout.queue(style::Print(line!()))?;
        }
        Ok(())
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.draw_buffer()?;
        self.draw_statusline()?;
        self.stdout.queue(cursor::MoveTo(self.cx, self.cy))?;
        self.stdout.flush()?;
        Ok(())
    }
    pub fn draw_statusline(&mut self)->anyhow::Result<()>{
        let mode = format!("{:?}",self.mode).to_uppercase();
        let pos = format!("{}:{}",self.cx,self.cy);
        let file = "src/main.rs";
        let file_width = self.size.0 - file.len() as u16 - mode.len() as u16 -5;

        self.stdout.queue(cursor::MoveTo(0,self.size.1-2))?;
        self.stdout.queue(style::PrintStyledContent(
            mode.with(Color::Rgb {r:0,g:0,b:0}).bold().on(
                Color::Rgb {
                    r:184,
                    g:144,
                    b:243,
                }
            )
        ))?;
        self.stdout.queue(style::PrintStyledContent(
            ""
                .with(Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 243,
                })
                .on(Color::Rgb {
                    r: 67,
                    g: 70,
                    b: 89,
                }),
        ))?;
        self.stdout.queue(style::PrintStyledContent(
            file
                .with(Color::Rgb {r:255,g:255,b:255})
                .on(Color::Rgb {
            r:67,
            g:70,
            b:89
        })))?;
        self.stdout.queue(style::PrintStyledContent(
            format!("{:>width$}","",width = file_width as usize)
                .with(Color::Rgb {
                    r:255,
                    g:255,
                    b:255
                }).bold()
                .on(Color::Rgb {
                    r:67,
                    g:70,
                    b:89
                })
            ))?;

        self.stdout.queue(style::PrintStyledContent(
            ""
                .with(Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 243,
                })
                .on(Color::Rgb {
                    r: 67,
                    g: 70,
                    b: 89,
                }),
        ))?;
        self.stdout.queue(style::PrintStyledContent(
            pos.with(Color::Rgb {
                 r:0,
                 g:0,
                 b:0
            }).bold().on(Color::Rgb {
                r:184,
                g:144,
                b:243
            })
        ))?;
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
        if matches!(ev,Event::Resize(_,_)) {
           self.size = terminal::size()?
        }
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