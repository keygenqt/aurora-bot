use std::fs;
use std::path::PathBuf;

use yaml_rust::YamlLoader;

use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::elements;
use genpdf::style;

use crate::service::requests::client::ClientRequest;
use crate::tools::macros::tr;

#[derive(Clone, Debug)]
pub struct PubspecModel {
    pub name: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub is_plugin: bool,
}

impl PubspecModel {
    pub fn parse_model(path_pubspec: &PathBuf) -> Result<PubspecModel, Box<dyn std::error::Error>> {
        // Load file
        let content = fs::read_to_string(path_pubspec)?;
        // Parse yaml
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let doc = &docs[0];
        // Get data
        let name = match doc["name"].as_str() {
            Some(value) => value.to_string(),
            None => Err(tr!("не удалось найти поле 'name'"))?,
        };
        let description = match doc["description"].as_str() {
            Some(value) => Some(value.to_string()),
            None => None,
        };
        let repository = match doc["repository"].as_str() {
            Some(value) => Some(value.to_string()),
            None => None,
        };
        let mut names_dependencies: Vec<String> = vec![];
        if let Some(dependencies) = doc["dependencies"].as_hash() {
            for (key, _) in dependencies.into_iter() {
                names_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        let mut names_dev_dependencies: Vec<String> = vec![];
        if let Some(dev_dependencies) = doc["dev_dependencies"].as_hash() {
            for (key, _) in dev_dependencies.into_iter() {
                names_dev_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        let is_plugin = match doc["flutter"].as_hash() {
            Some(value) => value.iter().any(|e| e.0.as_str() == Some("plugin")),
            None => false,
        };
        Ok(PubspecModel {
            name,
            description,
            repository,
            is_plugin,
        })
    }

    pub fn search_dependencies<T: Fn(i32) + Send + Copy + Sync + 'static>(
        path_pubspec: &PathBuf,
        state: T,
    ) -> Result<Vec<PubspecModel>, Box<dyn std::error::Error>> {
        // Load file
        let content = fs::read_to_string(path_pubspec)?;
        // Parse yaml
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let doc = &docs[0];
        // Get data
        let mut names_dependencies: Vec<String> = vec![];
        if let Some(dependencies) = doc["dependencies"].as_hash() {
            for (key, _) in dependencies.into_iter() {
                names_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        // Result
        Ok(ClientRequest::new(None).get_dart_packages(&names_dependencies, state)?)
    }

    pub fn gen_report_pdf(
        project: PubspecModel,
        dependencies: Vec<PubspecModel>,
        path_save: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let name_color: style::Color = style::Color::Rgb(0, 87, 155);
        let link_color: style::Color = style::Color::Rgb(25, 103, 210);

        // Get data list package
        fn get_package_list_data(package: &PubspecModel, link_color: style::Color) -> elements::UnorderedList {
            // Data
            let mut list = elements::UnorderedList::new();
            // Gen list
            list.push(
                elements::Paragraph::default()
                    .styled_string(format!("https://pub.dev/packages/{}", package.name), link_color),
            );
            if let Some(repository) = &package.repository {
                list.push(elements::Paragraph::default().styled_string(repository, link_color));
            }
            // Result
            list
        }

        fn get_package_data(
            package: &PubspecModel,
            name_color: style::Color,
            link_color: style::Color,
            is_name: bool,
        ) -> elements::LinearLayout {
            // Data
            let mut layout = elements::LinearLayout::vertical();
            // Gen layout
            if is_name {
                layout.push(
                    elements::Paragraph::default()
                        .styled_string(&package.name, name_color)
                        .styled(style::Style::new().bold()),
                );
            }
            if let Some(description) = &package.description {
                if is_name {
                    layout.push(elements::Break::new(0.5));
                }
                layout.push(elements::Paragraph::default().string(description));
            }
            layout.push(elements::Break::new(0.5));
            layout.push(elements::Paragraph::default().string("Links"));
            layout.push(get_package_list_data(package, link_color));
            // Result
            layout
        }

        // Base configuration file
        let font_family = match genpdf::fonts::from_files("/usr/share/fonts/liberation", "LiberationSans", None) {
            Ok(value) => value,
            Err(_) => genpdf::fonts::from_files("/usr/share/fonts/truetype/liberation", "LiberationSans", None)?,
        };
        let mut doc = genpdf::Document::new(font_family);
        doc.set_title(tr!("Отчет pubspec.yaml"));
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_line_spacing(1.25);
        doc.set_page_decorator(decorator);

        // Base info
        doc.push(
            elements::Paragraph::new("Отчет по зависимостям")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(20)),
        );
        doc.push(elements::Break::new(1.5));
        doc.push(
            elements::Paragraph::default()
                .string("Отчет по зависимостям пакета ")
                .styled_string(&project.name, style::Style::from(name_color).bold())
                .string(". Зависимости на платформа зависимые и платформа не зависимые, это должно упростить процесс портирование приложения Flutter под платформу ОС Аврора.")
        );
        doc.push(elements::Break::new(1.5));

        // About package
        doc.push(
            elements::Paragraph::new("Данные пакета")
                .aligned(Alignment::Center)
                .styled(style::Style::new().bold().with_font_size(16)),
        );
        doc.push(elements::Break::new(1));
        doc.push(get_package_data(&project, name_color, link_color, false).padded(1.5));
        doc.push(elements::Break::new(1.5));

        // Simple package
        if dependencies.clone().iter().any(|e| !e.is_plugin) {
            doc.push(
                elements::Paragraph::new("Пакеты")
                    .aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(16)),
            );
            doc.push(elements::Paragraph::new("(платформа не зависимые)").aligned(Alignment::Center));
            doc.push(elements::Break::new(1.5));
            // Table simple package
            let mut table = elements::TableLayout::new(vec![1]);
            table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
            for item in dependencies.clone() {
                if !item.is_plugin {
                    table
                        .row()
                        .element(
                            elements::LinearLayout::vertical()
                                .element(get_package_data(&item, name_color, link_color, true).padded(1.5))
                                .element(elements::Break::new(0.5))
                                .padded(1),
                        )
                        .push()
                        .expect("Invalid table row");
                }
            }
            doc.push(table);
        }

        // Plugin package
        doc.push(elements::Break::new(1.5));
        if dependencies.clone().iter().any(|e| e.is_plugin) {
            doc.push(
                elements::Paragraph::new("Плагины")
                    .aligned(Alignment::Center)
                    .styled(style::Style::new().bold().with_font_size(16)),
            );
            doc.push(elements::Paragraph::new("(платформа зависимые)").aligned(Alignment::Center));
            doc.push(elements::Break::new(1.5));
            // Table simple package
            let mut table = elements::TableLayout::new(vec![1]);
            table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
            for item in dependencies.clone() {
                if item.is_plugin {
                    table
                        .row()
                        .element(
                            elements::LinearLayout::vertical()
                                .element(get_package_data(&item, name_color, link_color, true).padded(1.5))
                                .element(elements::Break::new(0.5))
                                .padded(1),
                        )
                        .push()
                        .expect("Invalid table row");
                }
            }
            doc.push(table);
        }

        // Render the document and write it to a file
        doc.render_to_file(path_save)?;
        // Result
        Ok(())
    }
}
