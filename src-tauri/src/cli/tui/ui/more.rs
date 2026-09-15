use super::*;

pub(super) fn render_more(
    frame: &mut Frame<'_>,
    app: &App,
    area: Rect,
    theme: &super::theme::Theme,
) {
    let items = NavItem::more_for_app(&app.app_type);
    let body = render_page_frame(
        frame,
        area,
        theme,
        app,
        texts::menu_more(),
        &[
            ("↑↓", texts::tui_key_select()),
            ("Enter", texts::tui_key_open()),
            ("Esc", texts::tui_key_back()),
        ],
        None,
    );

    let list = List::new(items.iter().map(|item| {
        ListItem::new(Line::from(vec![
            Span::raw(nav_label(*item)),
            Span::styled("  ›", Style::default().fg(theme.dim)),
        ]))
    }))
    .highlight_style(selection_style(theme))
    .highlight_symbol(highlight_symbol(theme));

    let mut state = ListState::default();
    state.select(Some(app.more_idx.min(items.len().saturating_sub(1))));
    frame.render_stateful_widget(list, body, &mut state);
}
