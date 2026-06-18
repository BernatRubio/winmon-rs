use ab_glyph::{FontRef, PxScale};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use std::collections::HashMap;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateCompatibleBitmap, DeleteObject, GetDC, HBITMAP, HDC, ReleaseDC,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconIndirect, DestroyIcon, HICON, ICONINFO};
use windows::core::BOOL;

static FONT_REGULAR: &[u8] = include_bytes!(concat!(env!("SystemRoot"), r"\Fonts\segoeui.ttf"));
static FONT_BOLD: &[u8] = include_bytes!(concat!(env!("SystemRoot"), r"\Fonts\segoeuib.ttf"));

pub struct IconGenerator {
    label: &'static str,
    x: i32,
    font_regular: FontRef<'static>,
    font_bold: FontRef<'static>,
    cache: HashMap<u8, GeneratedIcon>,
}

impl IconGenerator {
    pub fn new(label: &'static str, x: i32) -> Self {
        Self {
            label,
            x,
            font_regular: FontRef::try_from_slice(FONT_REGULAR).unwrap(),
            font_bold: FontRef::try_from_slice(FONT_BOLD).unwrap(),
            cache: HashMap::with_capacity(100),
        }
    }

    fn generate_base_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(32, 32);

        let scale = PxScale { x: 18.0, y: 18.0 };

        let white = Rgba([255u8, 255u8, 255u8, 255u8]);

        draw_text_mut(
            &mut image,
            white,
            self.x,
            -4,
            scale,
            &self.font_regular,
            self.label,
        );
        return image;
    }

    fn scale_params(n: usize) -> (i32, i32, PxScale) {
        match n {
            1 => (10, 8, PxScale { x: 28.0, y: 28.0 }),
            2 => (4, 8, PxScale { x: 28.0, y: 28.0 }),
            _ => (0, 10, PxScale { x: 24.0, y: 24.0 }),
        }
    }

    fn create_icon(&self, value: u8) -> HICON {
        let value_to_draw = value.to_string();

        let mut image = self.generate_base_image();

        let (x, y, scale) = IconGenerator::scale_params(value_to_draw.len());

        let white = Rgba([255u8, 255u8, 255u8, 255u8]);

        draw_text_mut(
            &mut image,
            white,
            x,
            y,
            scale,
            &self.font_bold,
            &value_to_draw,
        );

        unsafe {
            let icon_resources = IconResources::new(image.into_raw(), GetDC(None));

            let h_icon = ICONINFO {
                fIcon: BOOL(1),
                hbmColor: icon_resources.hbm_color,
                hbmMask: icon_resources.hbm_mask,
                xHotspot: 0,
                yHotspot: 0,
            };

            match CreateIconIndirect(&h_icon) {
                Ok(hicon) => hicon,
                Err(error) => {
                    panic!("Icon could not be created: {:?}", error)
                }
            }
        }
    }

    pub fn generate(&mut self, value: u8) -> &GeneratedIcon {
        if !self.cache.contains_key(&value) {
            let icon = GeneratedIcon {
                hicon: self.create_icon(value),
            };
            self.cache.insert(value, icon);
        }

        self.cache.get(&value).unwrap()
    }
}

struct IconResources {
    hdc: HDC,
    hbm_color: HBITMAP,
    hbm_mask: HBITMAP,
}

impl IconResources {
    fn new(image_container: Vec<u8>, hdc: HDC) -> Self {
        unsafe {
            let hbm_mask = CreateCompatibleBitmap(hdc, 32, 32);

            let bytes = image_container;
            let bytes_ptr = bytes.as_ptr() as *const _;

            let hbm_color = CreateBitmap(32, 32, 1, 32, Some(bytes_ptr));

            Self {
                hdc,
                hbm_color,
                hbm_mask,
            }
        }
    }
}

impl Drop for IconResources {
    fn drop(&mut self) {
        unsafe {
            if !DeleteObject(self.hbm_color.into()).as_bool() {
                eprintln!("Warning: failed to delete hbm_color: {:?}", GetLastError());
            }
            if !DeleteObject(self.hbm_mask.into()).as_bool() {
                eprintln!("Warning: failed to delete hbm_mask: {:?}", GetLastError());
            }
            ReleaseDC(None, self.hdc);
        }
    }
}

pub struct GeneratedIcon {
    pub hicon: HICON,
}

impl Drop for GeneratedIcon {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyIcon(self.hicon);
        }
    }
}
