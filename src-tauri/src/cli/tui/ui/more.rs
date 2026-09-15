use super::*;
use crate::cli::tui::route::MoreGroup;

fn more_group_label(group: MoreGroup) -> &'static str {
    match group {
        MoreGroup::Records => crate::t!("Records", "记录"),
        MoreGroup::Extensions => crate::t!("Extensions", "扩展"),
        MoreGroup::Configuration => crate::t!("Configuration", "配置"),
    }
}

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

    // Section headings are plain rows the selection never lands on, so the
    // selected entry maps to a list row past every heading above it.
    let selected_item = app.more_idx.min(items.len().saturating_sub(1));
    let heading_style = Style::default()
        .fg(theme.comment)
        .add_modifier(Modifier::BOLD);
    let mut rows = Vec::new();
    let mut selected_row = None;
    let mut current_group = None;
    for (idx, item) in items.iter().enumerate() {
        let group = item.more_group();
        if current_group != Some(group) {
            if current_group.is_some() {
                rows.push(ListItem::new(Line::raw("")));
            }
            rows.push(ListItem::new(Line::from(Span::styled(
                more_group_label(group),
                heading_style,
            ))));
            current_group = Some(group);
        }
        if idx == selected_item {
            selected_row = Some(rows.len());
        }
        rows.push(ListItem::new(Line::from(vec![
            Span::raw("  "),
            Span::raw(nav_label(*item)),
            Span::styled("  ›", Style::default().fg(theme.dim)),
        ])));
    }

    let list = List::new(rows)
        .highlight_style(selection_style(theme))
        .highlight_symbol(highlight_symbol(theme));

    let mut state = ListState::default();
    state.select(selected_row);
    frame.render_stateful_widget(list, body, &mut state);
}
