use std::{collections::HashMap, convert::TryFrom, sync::Arc};

use serde::Deserialize;
use ybc::{TileCtx::{Ancestor, Child, Parent}};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Debug, Clone, Deserialize)]
struct GitHubReleases {
    assets: Vec<GitHubReleaseAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubReleaseAsset {
    name: Arc<str>,
    browser_download_url: Arc<str>,
}

#[derive(Properties, PartialEq)]
struct DownloadLinkParams {
    name: String,
    link: Option<Arc<str>>,
}

#[function_component(DownloadLink)]
fn download_link(params: &DownloadLinkParams) -> Html {
    if let Some(link) = &params.link {
        html! {
            <a download="true" href={String::from(&**link)}>
                <ybc::Button classes="is-fullwidth is-link">
                    {&params.name}
                </ybc::Button>
            </a>
        }
    } else {
        html! {
            <ybc::Button classes="is-fullwidth is-link">
                {&params.name}
            </ybc::Button>
        }
    }
}

#[derive(PartialEq)]
enum OperatingSystem {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

#[function_component(Home)]
pub fn home() -> Html {
    let releases = use_async_with_options(
        async {
            let releases: Option<GitHubReleases> = crate::services::request::get("https://api.github.com/repos/Moulberry/PandoraLauncher/releases/latest").await;
            releases.ok_or(())
        },
        UseAsyncOptions::enable_auto()
    );

    let mut releases_by_filename = HashMap::new();

    if let Some(data) = &releases.data {
        for asset in &data.assets {
            releases_by_filename.insert(asset.name.clone(), asset.browser_download_url.clone());
        }
    }

    let operating_system = if let Ok(user_agent) = web_sys::window().unwrap().navigator().user_agent() {
        if user_agent.contains("Mac") {
            OperatingSystem::MacOS
        } else if user_agent.contains("Win") {
            OperatingSystem::Windows
        } else if user_agent.contains("Linux") {
            OperatingSystem::Linux
        } else {
            OperatingSystem::Unknown
        }
    } else {
        OperatingSystem::Unknown
    };

    html! {
        <>

        <ybc::Hero
            classes="is-dark"
            size={ybc::HeroSize::FullheightWithNavbar}
            body={html!{
                <ybc::Container classes="is-centered">
                <ybc::Section>
                <ybc::Container classes="has-text-centered">
                    <span class={classes!("hero-banner-text")}>
                        <img src="pandora_main.svg" style="width: 45%"/>
                    </span>
                    <ybc::Subtitle size={ybc::HeaderSize::Is3}>
                        {"Pandora is a launcher that launches Minecraft"}
                    </ybc::Subtitle>
                </ybc::Container>
                </ybc::Section>

                {{
                    if operating_system == OperatingSystem::Windows {
                        html! {
                            <div style="display: flex; justify-content: center;">
                                <div style="width: 30%">
                                    <DownloadLink name="Download Windows Installer (.exe)" link={releases_by_filename.get("").cloned()}/>
                                </div>
                            </div>
                        }
                    } else if operating_system == OperatingSystem::MacOS {
                        html! {
                            <div style="display: flex; justify-content: center;">
                                <div style="width: 30%">
                                    <DownloadLink name="Download macOS Installer (.dmg)" link={releases_by_filename.get("").cloned()}/>
                                </div>
                            </div>
                        }
                    } else {
                        Default::default()
                    }
                }}

                // {
                //     format!("{:?}", releases.data)
                // }
                <ybc::Section>
                <ybc::Tile ctx={Ancestor}>
                    <ybc::Tile classes="is-vertical">
                        <ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"Windows x86_64"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Installer .exe (x64)" link={releases_by_filename.get("").cloned()}/>
                                    <DownloadLink name="Portable Executable .exe (x64)" link={releases_by_filename.get("PandoraLauncher-Windows-x86_64.exe").cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"Linux x86_64"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Debian Installer .deb (x64)" link={releases_by_filename.get("").cloned()}/>
                                    <DownloadLink name="AppImage .AppImage (x64)" link={releases_by_filename.get("").cloned()}/>
                                    <DownloadLink name="Portable Executable (x64)" link={releases_by_filename.get("PandoraLauncher-Linux-x86_64").cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent} size={ybc::TileSize::Four}>
                                <ybc::Tile ctx={Child} classes="notification is-primary">
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes="has-text-white">
                                        {"macOS"}
                                    </ybc::Subtitle>
                                    <div style="display: flex; flex-direction: column; gap: 10px">
                                    <DownloadLink name="Installer .dmg" link={releases_by_filename.get("").cloned()}/>
                                    <DownloadLink name="Portable App .app" link={releases_by_filename.get("").cloned()}/>
                                    <DownloadLink name="Portable Executable" link={releases_by_filename.get("PandoraLauncher-MacOS-arm64").cloned()}/>
                                    </div>
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                </ybc::Tile>
                </ybc::Section>
                </ybc::Container>
            }}>
        </ybc::Hero>

        </>
    }
}
