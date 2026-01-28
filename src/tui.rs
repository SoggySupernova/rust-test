use ratatui::style::Stylize;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::widgets::{Block, Paragraph};

pub fn main(text: String, window_title: String, outline_style: symbols::border::Set<'_>) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let block =
            Block::bordered()
            .border_set(outline_style)
            .border_style(
                Style::
                new()
                .blue()
            )
            .title(window_title);
            let greeting = Paragraph::new(text)
                .centered()
                .yellow()
                .block(block);
            frame.render_widget(greeting, frame.area());
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
