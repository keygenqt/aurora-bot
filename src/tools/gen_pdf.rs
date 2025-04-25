use std::path::PathBuf;

use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::elements;
use genpdf::style;

use crate::models::pubspec::model::PubspecModel;

/// Generate report about dart packager in Flutter project
pub fn gen_report_dart_packages(
    project: PubspecModel,
    dependencies: Vec<PubspecModel>,
    path_save: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Filter and sort by level
    let mut packages: Vec<PubspecModel> = dependencies.iter().filter(|e| !e.is_plugin).cloned().collect();
    packages.sort_by_key(|e| e.level);
    let mut plugins: Vec<PubspecModel> = dependencies.iter().filter(|e| e.is_plugin).cloned().collect();
    plugins.sort_by_key(|e| e.level);
    // Base configuration file
    let font_family = match genpdf::fonts::from_files("/usr/share/fonts/liberation", "LiberationSans", None) {
        Ok(value) => value,
        Err(_) => genpdf::fonts::from_files("/usr/share/fonts/truetype/liberation", "LiberationSans", None)?,
    };
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Отчет pubspec.yaml");
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
            .styled_string(&project.name, style::Style::from(style::Color::Rgb(0, 87, 155)).bold())
            .string(". Пакеты разделены на платформа не зависимые и платформа зависимые (плагины). Такой отчет упрощает процесс портирование приложения Flutter под платформу ОС Аврора.")
    );
    // Simple package
    if packages.len() != 0 {
        doc.push(elements::Break::new(1.5));
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
        for item in packages {
            table
                .row()
                .element(
                    elements::LinearLayout::vertical()
                        .element(_gen_layout_dart_package(&item).padded(1.5))
                        .element(elements::Break::new(0.5))
                        .padded(1),
                )
                .push()
                .expect("Invalid table row");
        }
        doc.push(table);
    }
    // Plugin package
    if plugins.len() != 0 {
        doc.push(elements::Break::new(1.5));
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
        for item in plugins {
            table
                .row()
                .element(
                    elements::LinearLayout::vertical()
                        .element(_gen_layout_dart_package(&item).padded(1.5))
                        .element(elements::Break::new(0.5))
                        .padded(1),
                )
                .push()
                .expect("Invalid table row");
        }
        doc.push(table);
    }
    // Footer
    doc.push(elements::Break::new(0.5));
    doc.push(
        elements::Paragraph::default().styled_string(
            "*Отчёт генерируется на основе версий актуальных пакетов.",
            style::Style::from(style::Color::Rgb(109, 109, 109))
                .italic()
                .with_font_size(10),
        ),
    );
    doc.push(elements::Break::new(0.2));
    doc.push(elements::Paragraph::new(format!(
        "{}",
        chrono::offset::Local::now().format("%d-%m-%Y %H:%M:%S")
    )));
    // Render the document and write it to a file
    doc.render_to_file(path_save)?;
    // Result
    Ok(())
}

fn _gen_layout_dart_package(package: &PubspecModel) -> elements::LinearLayout {
    let name_color: style::Color = style::Color::Rgb(0, 87, 155);
    let link_color: style::Color = style::Color::Rgb(25, 103, 210);
    let info_color: style::Color = style::Color::Rgb(109, 109, 109);
    // Package data layout
    let mut layout = elements::LinearLayout::vertical();
    // Set name
    layout.push(
        elements::Paragraph::default()
            .styled_string(&package.name, name_color)
            .styled(style::Style::new().bold()),
    );
    layout.push(elements::Break::new(0.2));
    // Info list
    let mut list = elements::UnorderedList::new();
    list.push(
        elements::Paragraph::default()
            .styled_string(format!("Latest: v{}", &package.version), info_color)
            .styled(style::Style::new().with_font_size(10)),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string(format!("Level: {}", &package.level), info_color)
            .styled(style::Style::new().with_font_size(10)),
    );
    layout.push(list);
    // Desc
    if let Some(description) = &package.description {
        layout.push(elements::Break::new(0.2));
        layout.push(elements::Paragraph::default().string(description));
    }
    // Package data list
    if package.pub_dev.is_some() || package.repository.is_some() {
        let mut list = elements::UnorderedList::new();
        layout.push(elements::Break::new(0.5));
        layout.push(elements::Paragraph::default().string("Links"));
        if let Some(pub_dev) = &package.pub_dev {
            list.push(
                elements::Paragraph::default()
                    .styled_string(pub_dev, link_color)
                    .styled(style::Style::new().with_font_size(10)),
            );
        }
        if let Some(repository) = &package.repository {
            list.push(
                elements::Paragraph::default()
                    .styled_string(repository, link_color)
                    .styled(style::Style::new().with_font_size(10)),
            );
        }
        layout.push(list);
    }
    // Result
    layout
}
