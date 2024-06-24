use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line};
use ratatui::style::Style;
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, CurrentlyEditing, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Ratatui Json Editor",
        Style::default().fg(Color::Green),
    ))
        .block(title_block);
    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <15}: {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let list = List::new(list_items).block(list_block);

    f.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Home => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editor => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exit => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
            .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Key", Style::default().fg(Color::Yellow))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Value", Style::default().fg(Color::Yellow))
                    }
                }
            } else {
                Span::styled("Normal Mode", Style::default().fg(Color::Green))
            }
        },
    ];

    let mood_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL).style(Style::default()));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Home => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editor => Span::styled(
                "(esc) to cancel/(tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => Span::styled(
                "(q) to quit / (y) to print json/ (esc) to cancel",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mood_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);


    if let Some(editing) = &app.currently_editing {
        let popup_title = Paragraph::new(Span::styled("Enter a new key-value pair", Style::default()));

        let area = centered_rect(60, 30, f.size());
        let background_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(background_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        f.render_widget(popup_title, popup_chunks[0]);

        let popup_main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(popup_chunks[1]);
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        f.render_widget(key_text, popup_main_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        f.render_widget(value_text, popup_main_chunks[1]);

        let hint_text = Paragraph::new(Span::styled(
            "Press Enter to save, Esc to cancel, Tab to switch",
            Style::default().fg(Color::Gray),
        ));
        f.render_widget(hint_text, popup_chunks[2]);
    }

    if let CurrentScreen::Exit = app.current_screen {
        f.render_widget(Clear, f.size());

        let area = centered_rect(60, 25, f.size());

        let popup_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(popup_block, area);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(area);


        let exit_text = Text::styled(
            "\nWould you like to output the buffer as json?",
            Style::default().fg(Color::Red),
        );
        let information_block = Block::default()
            .title("Exit Confirmation")
            .borders(Borders::ALL);
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(information_block)
            .wrap(Wrap { trim: false });

        f.render_widget(exit_paragraph, content_chunks[0]);

        let hint_block = Block::default().borders(Borders::ALL);
        let hint_text = Paragraph::new(Span::styled(
            "Press Y to output json, Q to quit, Esc to cancel",
            Style::default().fg(Color::Gray),
        )).block(hint_block);
        f.render_widget(hint_text, content_chunks[1]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    let popup_layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1];
    return popup_layout_rect;
}
