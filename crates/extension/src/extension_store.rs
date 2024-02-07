use anyhow::Result;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::StreamExt as _;
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, Task};
use language::{
    LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry, QUERY_FILENAME_PREFIXES,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::ThemeRegistry;
use util::{
    paths::{EXTENSIONS_DIR, EXTENSIONS_MANIFEST_PATH},
    ResultExt,
};

#[cfg(test)]
mod extension_store_test;

pub struct ExtensionStore {
    manifest: Arc<RwLock<Manifest>>,
    fs: Arc<dyn Fs>,
    extensions_dir: PathBuf,
    manifest_path: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GrammarManifestEntry {
    extension: String,
    grammar_name: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct LanguageManifestEntry {
    extension: String,
    language_dir: String,
    name: Arc<str>,
    matcher: LanguageMatcher,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThemeLocation {
    extension: String,
    filename: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Manifest {
    pub grammars: Vec<GrammarManifestEntry>,
    pub languages: Vec<LanguageManifestEntry>,
    pub themes_by_name: HashMap<String, ThemeLocation>,
}

actions!(extensions, [RebuildManifest]);

pub fn init(
    fs: Arc<fs::RealFs>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    let store = cx.new_model(|cx| {
        let mut store = ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            EXTENSIONS_MANIFEST_PATH.clone(),
            fs.clone(),
            language_registry.clone(),
            theme_registry,
        );
        store.load(cx).log_err();
        let manifest = store.manifest.clone();
        cx.background_executor()
            .spawn(async move {
                let mut events = fs.watch(&*EXTENSIONS_DIR, Duration::from_millis(250)).await;

                let mut changed_languages = HashSet::default();
                while let Some(events) = events.next().await {
                    let manifest = manifest.read();
                    for event in events {
                        for language in &manifest.languages {
                            let mut language_path = EXTENSIONS_DIR.clone();
                            language_path.extend([
                                &language.extension,
                                "languages",
                                &language.language_dir,
                            ]);
                            if event.path.starts_with(&language_path) || event.path == language_path
                            {
                                changed_languages.insert(language.name.clone());
                            }
                        }
                    }

                    language_registry.reload_languages(&changed_languages);
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        store
    });

    cx.on_action(|_: &RebuildManifest, cx| {
        let store = cx.global::<GlobalExtensionStore>().0.clone();
        store
            .update(cx, |store, cx| store.rebuild_manifest(cx))
            .detach_and_log_err(cx);
    });

    cx.set_global(GlobalExtensionStore(store));
}

impl ExtensionStore {
    pub fn new(
        extensions_dir: PathBuf,
        manifest_path: PathBuf,
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
    ) -> Self {
        Self {
            manifest: Default::default(),
            extensions_dir,
            manifest_path,
            fs,
            language_registry,
            theme_registry,
        }
    }

    pub fn load(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let manifest = cx
            .background_executor()
            .block(self.fs.load(&self.manifest_path))?;
        let manifest: Manifest = serde_json::from_str(&manifest)?;

        for grammar in &manifest.grammars {
            let mut grammar_path = self.extensions_dir.clone();
            grammar_path.extend([
                grammar.extension.as_str(),
                "grammars",
                &grammar.grammar_name,
            ]);
            grammar_path.set_extension("wasm");
            self.language_registry
                .register_grammar(grammar.grammar_name.clone(), grammar_path);
        }

        for language in &manifest.languages {
            let mut language_path = self.extensions_dir.clone();
            language_path.extend([
                language.extension.as_str(),
                "languages",
                &language.language_dir,
            ]);
            self.language_registry.register_extension(
                language_path.into(),
                language.name.clone(),
                language.matcher.clone(),
                load_plugin_queries,
            );
        }

        let fs = self.fs.clone();
        let root_dir = self.extensions_dir.clone();
        let theme_registry = self.theme_registry.clone();
        let themes = manifest.themes_by_name.clone();
        cx.background_executor()
            .spawn(async move {
                for theme in themes.values() {
                    let mut theme_path = root_dir.clone();
                    theme_path.extend([theme.extension.as_str(), "themes", &theme.filename]);

                    theme_registry
                        .load_user_theme(&theme_path, fs.clone())
                        .await
                        .log_err();
                }
            })
            .detach();

        *self.manifest.write() = manifest;
        Ok(())
    }

    pub fn rebuild_manifest(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        let extensions_dir = self.extensions_dir.clone();
        let manifest_path = self.manifest_path.clone();
        cx.spawn(|this, mut cx| async move {
            let manifest = cx
                .background_executor()
                .spawn(async move {
                    let mut manifest = Manifest::default();

                    let mut extension_paths = fs.read_dir(&extensions_dir).await?;
                    while let Some(extension_dir) = extension_paths.next().await {
                        let extension_dir = extension_dir?;
                        let Some(extension_name) =
                            extension_dir.file_name().and_then(OsStr::to_str)
                        else {
                            continue;
                        };

                        if let Ok(mut grammar_paths) =
                            fs.read_dir(&extension_dir.join("grammars")).await
                        {
                            while let Some(grammar_path) = grammar_paths.next().await {
                                let grammar_path = grammar_path?;
                                let Some(grammar_name) =
                                    grammar_path.file_stem().and_then(OsStr::to_str)
                                else {
                                    continue;
                                };

                                manifest.grammars.push(GrammarManifestEntry {
                                    extension: extension_name.into(),
                                    grammar_name: grammar_name.into(),
                                });
                            }
                        }

                        if let Ok(mut language_paths) =
                            fs.read_dir(&extension_dir.join("languages")).await
                        {
                            while let Some(language_path) = language_paths.next().await {
                                let language_path = language_path?;
                                let Some(dir_name) =
                                    language_path.file_name().and_then(OsStr::to_str)
                                else {
                                    continue;
                                };
                                let config = fs.load(&language_path.join("config.toml")).await?;
                                let config = ::toml::from_str::<LanguageConfig>(&config)?;

                                manifest.languages.push(LanguageManifestEntry {
                                    name: config.name.clone(),
                                    extension: extension_name.into(),
                                    language_dir: dir_name.into(),
                                    matcher: config.matcher,
                                });
                            }
                        }

                        if let Ok(mut theme_paths) =
                            fs.read_dir(&extension_dir.join("themes")).await
                        {
                            while let Some(theme_path) = theme_paths.next().await {
                                let theme_path = theme_path?;
                                let Some(theme_filename) =
                                    theme_path.file_name().and_then(OsStr::to_str)
                                else {
                                    continue;
                                };

                                let Some(theme_family) =
                                    ThemeRegistry::read_user_theme(&theme_path, fs.clone())
                                        .await
                                        .log_err()
                                else {
                                    continue;
                                };

                                for theme in theme_family.themes {
                                    let location = ThemeLocation {
                                        extension: extension_name.into(),
                                        filename: theme_filename.into(),
                                    };

                                    manifest.themes_by_name.insert(theme.name, location);
                                }
                            }
                        }
                    }

                    manifest.grammars.sort();
                    manifest.languages.sort();

                    fs.save(
                        &manifest_path,
                        &serde_json::to_string_pretty(&manifest)?.as_str().into(),
                        Default::default(),
                    )
                    .await?;

                    anyhow::Ok(manifest)
                })
                .await?;
            this.update(&mut cx, |this, _| *this.manifest.write() = manifest)
        })
    }
}

fn load_plugin_queries(root_path: &Path) -> LanguageQueries {
    let mut result = LanguageQueries::default();
    if let Some(entries) = std::fs::read_dir(root_path).log_err() {
        for entry in entries {
            let Some(entry) = entry.log_err() else {
                continue;
            };
            let path = entry.path();
            if let Some(remainder) = path.strip_prefix(root_path).ok().and_then(|p| p.to_str()) {
                if !remainder.ends_with(".scm") {
                    continue;
                }
                for (name, query) in QUERY_FILENAME_PREFIXES {
                    if remainder.starts_with(name) {
                        if let Some(contents) = std::fs::read_to_string(&path).log_err() {
                            match query(&mut result) {
                                None => *query(&mut result) = Some(contents.into()),
                                Some(r) => r.to_mut().push_str(contents.as_ref()),
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    result
}
