use crate::app_config::AppType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Main,
    Providers,
    Usage,
    More,
    UsageLogs,
    UsageLogDetail { rowid: i64 },
    Pricing,
    Sessions,
    Mcp,
    Prompts,
    PiSystemPrompts,
    PiPromptTemplates,
    HermesMemory,
    Config,
    ConfigOpenClawWorkspace,
    ConfigOpenClawDailyMemory,
    ConfigOpenClawEnv,
    ConfigOpenClawTools,
    ConfigOpenClawAgents,
    ConfigCloudSync,
    ConfigWebDav,
    ConfigS3,
    Skills,
    SkillsDiscover,
    SkillsRepos,
    SkillDetail { directory: String },
    Settings,
    SettingsProxy,
    SettingsOutboundProxy,
    SettingsManagedAccounts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavItem {
    Main,
    Providers,
    Usage,
    Sessions,
    Mcp,
    Prompts,
    PiSystemPrompts,
    PiPromptTemplates,
    HermesMemory,
    Config,
    Skills,
    OpenClawWorkspace,
    OpenClawEnv,
    OpenClawTools,
    OpenClawAgents,
    Settings,
    More,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoreGroup {
    Records,
    Extensions,
    Configuration,
}

impl NavItem {
    pub const ALL: [NavItem; 5] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Usage,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const OPENCLAW_ALL: [NavItem; 5] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Usage,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const HERMES_ALL: [NavItem; 5] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Usage,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const PI_ALL: [NavItem; 5] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Usage,
        NavItem::More,
        NavItem::Exit,
    ];

    // More lists are ordered by `more_group` so each section is contiguous.
    pub const MORE_ALL: [NavItem; 6] = [
        NavItem::Sessions,
        NavItem::Mcp,
        NavItem::Skills,
        NavItem::Prompts,
        NavItem::Config,
        NavItem::Settings,
    ];
    pub const OPENCLAW_MORE: [NavItem; 7] = [
        NavItem::Sessions,
        NavItem::OpenClawWorkspace,
        NavItem::OpenClawEnv,
        NavItem::OpenClawTools,
        NavItem::OpenClawAgents,
        NavItem::Config,
        NavItem::Settings,
    ];
    pub const HERMES_MORE: [NavItem; 6] = [
        NavItem::Sessions,
        NavItem::Mcp,
        NavItem::Skills,
        NavItem::HermesMemory,
        NavItem::Config,
        NavItem::Settings,
    ];
    pub const PI_MORE: [NavItem; 6] = [
        NavItem::Sessions,
        NavItem::Skills,
        NavItem::Prompts,
        NavItem::PiSystemPrompts,
        NavItem::PiPromptTemplates,
        NavItem::Settings,
    ];

    pub fn all_for_app(app_type: &AppType) -> &'static [NavItem] {
        match app_type {
            AppType::OpenClaw => &Self::OPENCLAW_ALL,
            AppType::Hermes => &Self::HERMES_ALL,
            AppType::Pi => &Self::PI_ALL,
            _ => &Self::ALL,
        }
    }

    pub fn more_for_app(app_type: &AppType) -> &'static [NavItem] {
        match app_type {
            AppType::OpenClaw => &Self::OPENCLAW_MORE,
            AppType::Hermes => &Self::HERMES_MORE,
            AppType::Pi => &Self::PI_MORE,
            _ => &Self::MORE_ALL,
        }
    }

    /// Section a More-page entry is listed under.
    pub fn more_group(self) -> MoreGroup {
        match self {
            NavItem::Sessions => MoreGroup::Records,
            NavItem::Config | NavItem::Settings => MoreGroup::Configuration,
            _ => MoreGroup::Extensions,
        }
    }

    pub fn to_route(self) -> Option<Route> {
        match self {
            NavItem::Main => Some(Route::Main),
            NavItem::Providers => Some(Route::Providers),
            NavItem::Usage => Some(Route::Usage),
            NavItem::Sessions => Some(Route::Sessions),
            NavItem::Mcp => Some(Route::Mcp),
            NavItem::Prompts => Some(Route::Prompts),
            NavItem::PiSystemPrompts => Some(Route::PiSystemPrompts),
            NavItem::PiPromptTemplates => Some(Route::PiPromptTemplates),
            NavItem::HermesMemory => Some(Route::HermesMemory),
            NavItem::Config => Some(Route::Config),
            NavItem::Skills => Some(Route::Skills),
            NavItem::OpenClawWorkspace => Some(Route::ConfigOpenClawWorkspace),
            NavItem::OpenClawEnv => Some(Route::ConfigOpenClawEnv),
            NavItem::OpenClawTools => Some(Route::ConfigOpenClawTools),
            NavItem::OpenClawAgents => Some(Route::ConfigOpenClawAgents),
            NavItem::Settings => Some(Route::Settings),
            NavItem::More => Some(Route::More),
            NavItem::Exit => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NavItem, Route};
    use crate::app_config::AppType;

    #[test]
    fn advanced_items_are_grouped_under_more() {
        let more = NavItem::ALL
            .iter()
            .position(|item| matches!(item, NavItem::More))
            .expect("more nav item should exist");

        assert_eq!(more, 3);
        assert_eq!(NavItem::more_for_app(&AppType::Claude), &NavItem::MORE_ALL);
        assert_eq!(
            NavItem::ALL[..3],
            [NavItem::Main, NavItem::Providers, NavItem::Usage]
        );
    }

    #[test]
    fn pricing_is_not_a_top_level_nav_item() {
        for nav_items in [
            NavItem::ALL.as_slice(),
            NavItem::OPENCLAW_ALL.as_slice(),
            NavItem::HERMES_ALL.as_slice(),
            NavItem::PI_ALL.as_slice(),
        ] {
            assert!(nav_items
                .iter()
                .all(|item| item.to_route() != Some(Route::Pricing)));
        }
    }

    #[test]
    fn pi_nav_exposes_native_prompt_pages_without_mcp_or_generic_config() {
        assert!(NavItem::PI_MORE
            .iter()
            .any(|item| matches!(item, NavItem::PiSystemPrompts)));
        assert!(NavItem::PI_MORE
            .iter()
            .any(|item| matches!(item, NavItem::PiPromptTemplates)));
        assert!(!NavItem::PI_MORE
            .iter()
            .any(|item| matches!(item, NavItem::Mcp | NavItem::Config)));
    }

    #[test]
    fn hermes_nav_uses_memory_instead_of_prompts() {
        assert!(NavItem::HERMES_MORE
            .iter()
            .any(|item| matches!(item, NavItem::HermesMemory)));
        assert!(!NavItem::HERMES_MORE
            .iter()
            .any(|item| matches!(item, NavItem::Prompts)));
    }

    #[test]
    fn openclaw_more_page_keeps_generic_config_entry() {
        assert!(NavItem::OPENCLAW_MORE
            .iter()
            .any(|item| matches!(item, NavItem::Config)));
    }

    #[test]
    fn more_lists_keep_each_group_contiguous() {
        for list in [
            NavItem::MORE_ALL.as_slice(),
            NavItem::OPENCLAW_MORE.as_slice(),
            NavItem::HERMES_MORE.as_slice(),
            NavItem::PI_MORE.as_slice(),
        ] {
            let mut seen = Vec::new();
            for group in list.iter().map(|item| item.more_group()) {
                if seen.last() != Some(&group) {
                    assert!(!seen.contains(&group), "{list:?} splits {group:?}");
                    seen.push(group);
                }
            }
        }
    }
}
