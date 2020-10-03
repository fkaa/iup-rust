use std::ptr;
use std::ffi::CString;

pub type Color = i32;

#[repr(i32)]
pub enum WriteMode {
    Replace = cd_sys::CD_REPLACE,
    Xor = cd_sys::CD_XOR,
    NotXor = cd_sys::CD_NOT_XOR,
}

#[repr(i32)]
pub enum FillMode {
    EvenOdd = cd_sys::CD_EVENODD,
    Winding = cd_sys::CD_WINDING,
}

#[repr(i32)]
pub enum HatchStyle {
    Horizontal = cd_sys::CD_HORIZONTAL,
    Vertical = cd_sys::CD_VERTICAL,
    FDiagonal = cd_sys::CD_FDIAGONAL,
    BDiagonal = cd_sys::CD_BDIAGONAL,
    Cross = cd_sys::CD_CROSS,
    DiagCross = cd_sys::CD_DIAGCROSS,
}

#[repr(i32)]
pub enum FontStyle {
    Plain = cd_sys::CD_PLAIN,
    Bold = cd_sys::CD_BOLD,
    Italic = cd_sys::CD_ITALIC,
    Underline = cd_sys::CD_UNDERLINE,
    Strikeout = cd_sys::CD_STRIKEOUT,
}

#[repr(i32)]
pub enum InteriorStyle {
    Solid = cd_sys::CD_SOLID,
    Hollow = cd_sys::CD_HOLLOW,
    Hatch = cd_sys::CD_HATCH,
    Stipple = cd_sys::CD_STIPPLE,
    Pattern = cd_sys::CD_PATTERN,
}

#[repr(i32)]
pub enum LineStyle {
    Continuous = cd_sys::CD_CONTINUOUS,
    Dashed = cd_sys::CD_DASHED,
    Dotted = cd_sys::CD_DOTTED,
    DashDot = cd_sys::CD_DASH_DOT,
    DashDotDot = cd_sys::CD_DASH_DOT_DOT,
}

#[repr(i32)]
pub enum Opacity {
    Opaque = cd_sys::CD_OPAQUE,
    Transparent = cd_sys::CD_TRANSPARENT,
}

#[repr(i32)]
pub enum TextAlignment {
    North = cd_sys::CD_OPAQUE,
    South = cd_sys::CD_SOUTH,
    East = cd_sys::CD_EAST,
    West = cd_sys::CD_WEST,
    NorthWest = cd_sys::CD_NORTH_WEST,
    SouthEast = cd_sys::CD_SOUTH_EAST,
    SouthWest = cd_sys::CD_SOUTH_WEST,
    Center = cd_sys::CD_CENTER,
    BaseLeft = cd_sys::CD_BASE_LEFT,
    BaseCenter = cd_sys::CD_BASE_CENTER,
    BaseRight = cd_sys::CD_BASE_RIGHT,
}

pub struct Canvas(*mut cd_sys::cdCanvas);

impl Canvas {
    pub fn from_raw(ptr: *mut cd_sys::cdCanvas) -> Self {
        Canvas(ptr)
    }

    pub fn get_size(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;

        unsafe {
            cd_sys::cdCanvasGetSize(self.0, &mut w, &mut h, ptr::null_mut(), ptr::null_mut());
        }

        (w, h)
    }

    pub fn set_foreground(&self, color: Color) {
        unsafe {
            cd_sys::cdCanvasSetForeground(self.0, color);
        }
    }

    pub fn set_background(&self, color: Color) {
        unsafe {
            cd_sys::cdCanvasSetBackground(self.0, color);
        }
    }

    pub fn set_write_mode(&self, write_mode: WriteMode) {
        unsafe {
            cd_sys::cdCanvasWriteMode(self.0, write_mode as _);
        }
    }

    pub fn set_line_style(&self, line_style: LineStyle) {
        unsafe {
            cd_sys::cdCanvasLineStyle(self.0, line_style as _);
        }
    }

    pub fn set_line_width(&self, line_width: i32) {
        unsafe {
            cd_sys::cdCanvasLineWidth(self.0, line_width);
        }
    }

    pub fn set_back_opacity(&self, opacity: Opacity) {
        unsafe {
            cd_sys::cdCanvasBackOpacity(self.0, opacity as _);
        }
    }

    pub fn set_fill_mode(&self, fill_mode: FillMode) {
        unsafe {
            cd_sys::cdCanvasFillMode(self.0, fill_mode as _);
        }
    }

    pub fn set_interior_style(&self, interior_style: InteriorStyle) {
        unsafe {
            cd_sys::cdCanvasInteriorStyle(self.0, interior_style as _);
        }
    }

    pub fn set_hatch_style(&self, hatch_style: HatchStyle) {
        unsafe {
            cd_sys::cdCanvasHatch(self.0, hatch_style as _);
        }
    }

    pub fn set_native_font(&self, native_font: &str) {
        let native_font_c = CString::new(native_font).unwrap();

        unsafe {
            cd_sys::cdCanvasNativeFont(self.0, native_font_c.as_ptr() as _);
        }
    }

    pub fn set_text_alignment(&self, alignment: TextAlignment) {
        unsafe {
            cd_sys::cdCanvasTextAlignment(self.0, alignment as _);
        }
    }

    pub fn get_text_size(&self, text: &str) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;

        let text_c = CString::new(text).unwrap();

        unsafe {
            cd_sys::cdCanvasGetTextSize(self.0, text_c.as_ptr() as _, &mut w, &mut h);
        }

        (w, h)
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            cd_sys::cdCanvasLine(self.0, x1, y1, x2, y2);
        }
    }

    pub fn draw_rect(&self, xmin: i32, xmax: i32, ymin: i32, ymax: i32) {
        unsafe {
            cd_sys::cdCanvasRect(self.0, xmin, xmax, ymin, ymax);
        }
    }

    pub fn draw_box(&self, xmin: i32, xmax: i32, ymin: i32, ymax: i32) {
        unsafe {
            cd_sys::cdCanvasBox(self.0, xmin, xmax, ymin, ymax);
        }
    }
}
