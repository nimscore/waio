#![allow(dead_code)]

use slint::platform::software_renderer::{
    MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel, SoftwareRenderer,
};
use slint::PhysicalSize;
use slint_interpreter::{Compiler, ComponentInstance};
use tracing::{error, info, warn};

pub struct SoftwareRenderHandler {
    window: Option<std::rc::Rc<MinimalSoftwareWindow>>,
    instance: Option<ComponentInstance>,
    width: u32,
    height: u32,
}

impl SoftwareRenderHandler {
    pub fn new() -> Self {
        Self {
            window: None,
            instance: None,
            width: 0,
            height: 0,
        }
    }

    pub fn create_component(
        &mut self,
        slint_code: &str,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        info!("SoftwareRenderer: creating component {}x{}", width, height);

        self.width = width;
        self.height = height;

        let compiler = Compiler::default();
        let result = spin_on::spin_on(compiler.build_from_source(
            slint_code.to_string(),
            std::path::PathBuf::from("virtual.slint"),
        ));

        for diag in result.diagnostics() {
            match diag.level() {
                slint_interpreter::DiagnosticLevel::Error => error!("Slint: {}", diag),
                slint_interpreter::DiagnosticLevel::Warning => warn!("Slint: {}", diag),
                _ => info!("Slint: {}", diag),
            }
        }

        let definition = match result
            .component("AuraBar")
            .or_else(|| result.components().next())
        {
            Some(d) => d,
            None => return Err("No component found".to_string()),
        };

        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        window.set_size(PhysicalSize::new(width, height));

        let instance = definition
            .create_with_existing_window(&window)
            .map_err(|e| e.to_string())?;

        self.window = Some(window);
        self.instance = Some(instance);

        info!("SoftwareRenderer: component created successfully");
        Ok(())
    }

    pub fn set_property(&self, property: &str, value: &str) {
        if let Some(ref instance) = self.instance {
            if let Err(e) = instance.set_property(
                property,
                slint_interpreter::Value::String(slint::SharedString::from(value)),
            ) {
                warn!("Failed to set property {}: {}", property, e);
            } else {
                info!("SoftwareRenderer: property set: {} = {}", property, value);
            }
        } else {
            warn!("No component instance to set property: {}", property);
        }
    }

    pub fn render(&self) -> Result<Vec<u8>, String> {
        let Some(ref window) = self.window else {
            return Err("No window created".to_string());
        };

        window.request_redraw();

        let width = self.width as usize;
        let height = self.height as usize;

        let mut pixels: Vec<Rgb565Pixel> = vec![Rgb565Pixel::default(); width * height];

        window.draw_if_needed(|renderer: &SoftwareRenderer| {
            renderer.render(&mut pixels, width);
        });

        let bytes: Vec<u8> = pixels
            .iter()
            .flat_map(|p| {
                let r = (p.0 >> 11) & 0x1F;
                let g = (p.0 >> 5) & 0x3F;
                let b = p.0 & 0x1F;
                vec![
                    ((r << 3) | (r >> 2)) as u8,
                    ((g << 2) | (g >> 4)) as u8,
                    ((b << 3) | (b >> 2)) as u8,
                    0xFF,
                ]
            })
            .collect();

        info!(
            "SoftwareRenderer: rendered {}x{} ({} bytes)",
            self.width,
            self.height,
            bytes.len()
        );
        Ok(bytes)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
