use crate::app_config::AppType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Main,
    Providers,
    Usage,
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

impl NavItem {
    pub const ALL: [NavItem; 8] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Sessions,
        NavItem::Usage,
        NavItem::Config,
        NavItem::Settings,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const OPENCLAW_ALL: [NavItem; 8] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Sessions,
        NavItem::Usage,
        NavItem::Config,
        NavItem::Settings,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const HERMES_ALL: [NavItem; 8] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Sessions,
        NavItem::Usage,
        NavItem::Config,
        NavItem::Settings,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const PI_ALL: [NavItem; 7] = [
        NavItem::Main,
        NavItem::Providers,
        NavItem::Sessions,
        NavItem::Usage,
        NavItem::Settings,
        NavItem::More,
        NavItem::Exit,
    ];

    pub const MORE_ALL: [NavItem; 3] = [NavItem::Mcp, NavItem::Skills, NavItem::Prompts];
    pub const OPENCLAW_MORE: [NavItem; 4] = [
        NavItem::OpenClawWorkspace,
        NavItem::OpenClawEnv,
        NavItem::OpenClawTools,
        NavItem::OpenClawAgents,
    ];
    pub const HERMES_MORE: [NavItem; 3] = [NavItem::Mcp, NavItem::Skills, NavItem::HermesMemory];
    pub const PI_MORE: [NavItem; 4] = [
        NavItem::Skills,
        NavItem::Prompts,
        NavItem::PiSystemPrompts,
        NavItem::PiPromptTemplates,
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
            NavItem::More => None,
            NavItem::Exit => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app_config::AppType;
    use super::{NavItem, Route};

    #[test]
    fn advanced_items_are_grouped_under_more() {
        let more = NavItem::ALL
            .iter()
            .position(|item| matches!(item, NavItem::More))
            .expect("more nav item should exist");

        assert_eq!(more, 6);
        assert_eq!(NavItem::more_for_app(&AppType::Claude), &NavItem::MORE_ALL);
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
    fn openclaw_nav_keeps_generic_config_entry() {
        assert!(NavItem::OPENCLAW_ALL
            .iter()
            .any(|item| matches!(item, NavItem::Config)));
    }
}
