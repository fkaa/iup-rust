#![allow(dead_code, non_camel_case_types, non_snake_case)]
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_ulong, c_void};

pub type cdCallback = Option<unsafe extern "C" fn(canvas: *mut cdCanvas, ...) -> c_int>;
pub enum cdContext {}
pub enum cdCanvas {}
pub enum cdState {}
pub enum cdImage {}
pub const CD_QUERY: i32 = -1;
pub const CD_POLYCUSTOM: u32 = 10;
pub const CD_CAP_NONE: u32 = 0;
pub const CD_CAP_FLUSH: u32 = 1;
pub const CD_CAP_CLEAR: u32 = 2;
pub const CD_CAP_PLAY: u32 = 4;
pub const CD_CAP_YAXIS: u32 = 8;
pub const CD_CAP_CLIPAREA: u32 = 16;
pub const CD_CAP_CLIPPOLY: u32 = 32;
pub const CD_CAP_REGION: u32 = 64;
pub const CD_CAP_RECT: u32 = 128;
pub const CD_CAP_CHORD: u32 = 256;
pub const CD_CAP_IMAGERGB: u32 = 512;
pub const CD_CAP_IMAGERGBA: u32 = 1024;
pub const CD_CAP_IMAGEMAP: u32 = 2048;
pub const CD_CAP_GETIMAGERGB: u32 = 4096;
pub const CD_CAP_IMAGESRV: u32 = 8192;
pub const CD_CAP_BACKGROUND: u32 = 16384;
pub const CD_CAP_BACKOPACITY: u32 = 32768;
pub const CD_CAP_WRITEMODE: u32 = 65536;
pub const CD_CAP_LINESTYLE: u32 = 131072;
pub const CD_CAP_LINEWITH: u32 = 262144;
pub const CD_CAP_FPRIMTIVES: u32 = 524288;
pub const CD_CAP_HATCH: u32 = 1048576;
pub const CD_CAP_STIPPLE: u32 = 2097152;
pub const CD_CAP_PATTERN: u32 = 4194304;
pub const CD_CAP_FONT: u32 = 8388608;
pub const CD_CAP_FONTDIM: u32 = 16777216;
pub const CD_CAP_TEXTSIZE: u32 = 33554432;
pub const CD_CAP_TEXTORIENTATION: u32 = 67108864;
pub const CD_CAP_PALETTE: u32 = 134217728;
pub const CD_CAP_LINECAP: u32 = 268435456;
pub const CD_CAP_LINEJOIN: u32 = 536870912;
pub const CD_CAP_PATH: u32 = 1073741824;
pub const CD_CAP_BEZIER: u32 = 2147483648;
pub const CD_CAP_ALL: u32 = 4294967295;
pub const CD_SIZECB: u32 = 0;
pub const CD_ABORT: u32 = 1;
pub const CD_CONTINUE: u32 = 0;
pub const CD_SIM_NONE: u32 = 0;
pub const CD_SIM_LINE: u32 = 1;
pub const CD_SIM_RECT: u32 = 2;
pub const CD_SIM_BOX: u32 = 4;
pub const CD_SIM_ARC: u32 = 8;
pub const CD_SIM_SECTOR: u32 = 16;
pub const CD_SIM_CHORD: u32 = 32;
pub const CD_SIM_POLYLINE: u32 = 64;
pub const CD_SIM_POLYGON: u32 = 128;
pub const CD_SIM_TEXT: u32 = 256;
pub const CD_SIM_ALL: u32 = 65535;
pub const CD_SIM_LINES: u32 = 75;
pub const CD_SIM_FILLS: u32 = 180;
pub const CD_RED: u32 = 16711680;
pub const CD_DARK_RED: u32 = 8388608;
pub const CD_GREEN: u32 = 65280;
pub const CD_DARK_GREEN: u32 = 32768;
pub const CD_BLUE: u32 = 255;
pub const CD_DARK_BLUE: u32 = 128;
pub const CD_YELLOW: u32 = 16776960;
pub const CD_DARK_YELLOW: u32 = 8421376;
pub const CD_MAGENTA: u32 = 16711935;
pub const CD_DARK_MAGENTA: u32 = 8388736;
pub const CD_CYAN: u32 = 65535;
pub const CD_DARK_CYAN: u32 = 32896;
pub const CD_WHITE: u32 = 16777215;
pub const CD_BLACK: u32 = 0;
pub const CD_DARK_GRAY: u32 = 8421504;
pub const CD_GRAY: u32 = 12632256;
pub const CD_MM2PT: f64 = 2.834645669;
pub const CD_RAD2DEG: f64 = 57.295779513;
pub const CD_DEG2RAD: f64 = 0.01745329252;

#[repr(C)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cdBitmap {
    pub w: c_int,
    pub h: c_int,
    pub type_: c_int,
    pub data: *mut c_void,
}
extern "C" {
    pub fn cdVersion() -> *mut c_char;
    pub fn cdVersionDate() -> *mut c_char;
    pub fn cdVersionNumber() -> c_int;

    pub fn cdCreateCanvas(context: *mut cdContext, data: *mut c_void) -> *mut cdCanvas;
    pub fn cdCreateCanvasf(context: *mut cdContext, format: *const c_char, ...) -> *mut cdCanvas;
    pub fn cdKillCanvas(canvas: *mut cdCanvas);

    pub fn cdCanvasGetContext(canvas: *mut cdCanvas) -> *mut cdContext;

    pub fn cdCanvasActivate(canvas: *mut cdCanvas) -> c_int;

    pub fn cdCanvasDeactivate(canvas: *mut cdCanvas);

    pub fn cdUseContextPlus(use_: c_int) -> c_int;

    pub fn cdInitContextPlus();

    pub fn cdFinishContextPlus();

    pub fn cdContextRegisterCallback(context: *mut cdContext, cb: c_int, func: cdCallback)
        -> c_int;

    pub fn cdContextCaps(context: *mut cdContext) -> c_ulong;

    pub fn cdContextIsPlus(context: *mut cdContext) -> c_int;

    pub fn cdContextType(context: *mut cdContext) -> c_int;

    pub fn cdCanvasSimulate(canvas: *mut cdCanvas, mode: c_int) -> c_int;

    pub fn cdCanvasFlush(canvas: *mut cdCanvas);

    pub fn cdCanvasClear(canvas: *mut cdCanvas);

    pub fn cdCanvasSaveState(canvas: *mut cdCanvas) -> *mut cdState;

    pub fn cdCanvasRestoreState(canvas: *mut cdCanvas, state: *mut cdState);

    pub fn cdReleaseState(state: *mut cdState);

    pub fn cdCanvasSetAttribute(canvas: *mut cdCanvas, name: *const c_char, data: *mut c_char);

    pub fn cdCanvasSetfAttribute(
        canvas: *mut cdCanvas,
        name: *const c_char,
        format: *const c_char,
        ...
    );

    pub fn cdCanvasGetAttribute(canvas: *mut cdCanvas, name: *const c_char) -> *mut c_char;

    pub fn cdCanvasPlay(
        canvas: *mut cdCanvas,
        context: *mut cdContext,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
        data: *mut c_void,
    ) -> c_int;

    pub fn cdCanvasGetSize(
        canvas: *mut cdCanvas,
        width: *mut c_int,
        height: *mut c_int,
        width_mm: *mut f64,
        height_mm: *mut f64,
    );

    pub fn cdCanvasUpdateYAxis(canvas: *mut cdCanvas, y: *mut c_int) -> c_int;

    pub fn cdfCanvasUpdateYAxis(canvas: *mut cdCanvas, y: *mut f64) -> f64;

    pub fn cdCanvasInvertYAxis(canvas: *mut cdCanvas, y: c_int) -> c_int;

    pub fn cdfCanvasInvertYAxis(canvas: *mut cdCanvas, y: f64) -> f64;

    pub fn cdCanvasMM2Pixel(
        canvas: *mut cdCanvas,
        mm_dx: f64,
        mm_dy: f64,
        dx: *mut c_int,
        dy: *mut c_int,
    );

    pub fn cdCanvasPixel2MM(
        canvas: *mut cdCanvas,
        dx: c_int,
        dy: c_int,
        mm_dx: *mut f64,
        mm_dy: *mut f64,
    );

    pub fn cdfCanvasMM2Pixel(
        canvas: *mut cdCanvas,
        mm_dx: f64,
        mm_dy: f64,
        dx: *mut f64,
        dy: *mut f64,
    );

    pub fn cdfCanvasPixel2MM(
        canvas: *mut cdCanvas,
        dx: f64,
        dy: f64,
        mm_dx: *mut f64,
        mm_dy: *mut f64,
    );

    pub fn cdCanvasOrigin(canvas: *mut cdCanvas, x: c_int, y: c_int);

    pub fn cdfCanvasOrigin(canvas: *mut cdCanvas, x: f64, y: f64);

    pub fn cdCanvasGetOrigin(canvas: *mut cdCanvas, x: *mut c_int, y: *mut c_int);

    pub fn cdfCanvasGetOrigin(canvas: *mut cdCanvas, x: *mut f64, y: *mut f64);

    pub fn cdCanvasTransform(canvas: *mut cdCanvas, matrix: *const f64);

    pub fn cdCanvasGetTransform(canvas: *mut cdCanvas) -> *mut f64;

    pub fn cdCanvasTransformMultiply(canvas: *mut cdCanvas, matrix: *const f64);

    pub fn cdCanvasTransformRotate(canvas: *mut cdCanvas, angle: f64);

    pub fn cdCanvasTransformScale(canvas: *mut cdCanvas, sx: f64, sy: f64);

    pub fn cdCanvasTransformTranslate(canvas: *mut cdCanvas, dx: f64, dy: f64);

    pub fn cdCanvasTransformPoint(
        canvas: *mut cdCanvas,
        x: c_int,
        y: c_int,
        tx: *mut c_int,
        ty: *mut c_int,
    );

    pub fn cdfCanvasTransformPoint(
        canvas: *mut cdCanvas,
        x: f64,
        y: f64,
        tx: *mut f64,
        ty: *mut f64,
    );

    pub fn cdCanvasClip(canvas: *mut cdCanvas, mode: c_int) -> c_int;

    pub fn cdCanvasClipArea(
        canvas: *mut cdCanvas,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasGetClipArea(
        canvas: *mut cdCanvas,
        xmin: *mut c_int,
        xmax: *mut c_int,
        ymin: *mut c_int,
        ymax: *mut c_int,
    ) -> c_int;

    pub fn cdfCanvasClipArea(canvas: *mut cdCanvas, xmin: f64, xmax: f64, ymin: f64, ymax: f64);

    pub fn cdfCanvasGetClipArea(
        canvas: *mut cdCanvas,
        xmin: *mut f64,
        xmax: *mut f64,
        ymin: *mut f64,
        ymax: *mut f64,
    ) -> c_int;

    pub fn cdCanvasIsPointInRegion(canvas: *mut cdCanvas, x: c_int, y: c_int) -> c_int;

    pub fn cdCanvasOffsetRegion(canvas: *mut cdCanvas, x: c_int, y: c_int);

    pub fn cdCanvasGetRegionBox(
        canvas: *mut cdCanvas,
        xmin: *mut c_int,
        xmax: *mut c_int,
        ymin: *mut c_int,
        ymax: *mut c_int,
    );

    pub fn cdCanvasRegionCombineMode(canvas: *mut cdCanvas, mode: c_int) -> c_int;

    pub fn cdCanvasPixel(canvas: *mut cdCanvas, x: c_int, y: c_int, color: c_long);

    pub fn cdCanvasMark(canvas: *mut cdCanvas, x: c_int, y: c_int);

    pub fn cdfCanvasPixel(canvas: *mut cdCanvas, x: f64, y: f64, color: c_long);

    pub fn cdfCanvasMark(canvas: *mut cdCanvas, x: f64, y: f64);

    pub fn cdCanvasBegin(canvas: *mut cdCanvas, mode: c_int);

    pub fn cdCanvasPathSet(canvas: *mut cdCanvas, action: c_int);

    pub fn cdCanvasEnd(canvas: *mut cdCanvas);

    pub fn cdCanvasLine(canvas: *mut cdCanvas, x1: c_int, y1: c_int, x2: c_int, y2: c_int);

    pub fn cdCanvasVertex(canvas: *mut cdCanvas, x: c_int, y: c_int);

    pub fn cdCanvasRect(canvas: *mut cdCanvas, xmin: c_int, xmax: c_int, ymin: c_int, ymax: c_int);

    pub fn cdCanvasBox(canvas: *mut cdCanvas, xmin: c_int, xmax: c_int, ymin: c_int, ymax: c_int);

    pub fn cdCanvasArc(
        canvas: *mut cdCanvas,
        xc: c_int,
        yc: c_int,
        w: c_int,
        h: c_int,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdCanvasSector(
        canvas: *mut cdCanvas,
        xc: c_int,
        yc: c_int,
        w: c_int,
        h: c_int,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdCanvasChord(
        canvas: *mut cdCanvas,
        xc: c_int,
        yc: c_int,
        w: c_int,
        h: c_int,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdCanvasText(canvas: *mut cdCanvas, x: c_int, y: c_int, s: *const c_char);

    pub fn cdfCanvasLine(canvas: *mut cdCanvas, x1: f64, y1: f64, x2: f64, y2: f64);

    pub fn cdfCanvasVertex(canvas: *mut cdCanvas, x: f64, y: f64);

    pub fn cdfCanvasRect(canvas: *mut cdCanvas, xmin: f64, xmax: f64, ymin: f64, ymax: f64);

    pub fn cdfCanvasBox(canvas: *mut cdCanvas, xmin: f64, xmax: f64, ymin: f64, ymax: f64);

    pub fn cdfCanvasArc(
        canvas: *mut cdCanvas,
        xc: f64,
        yc: f64,
        w: f64,
        h: f64,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdfCanvasSector(
        canvas: *mut cdCanvas,
        xc: f64,
        yc: f64,
        w: f64,
        h: f64,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdfCanvasChord(
        canvas: *mut cdCanvas,
        xc: f64,
        yc: f64,
        w: f64,
        h: f64,
        angle1: f64,
        angle2: f64,
    );

    pub fn cdfCanvasText(canvas: *mut cdCanvas, x: f64, y: f64, s: *const c_char);

    pub fn cdCanvasSetBackground(canvas: *mut cdCanvas, color: c_long);

    pub fn cdCanvasSetForeground(canvas: *mut cdCanvas, color: c_long);

    pub fn cdCanvasBackground(canvas: *mut cdCanvas, color: c_long) -> c_long;

    pub fn cdCanvasForeground(canvas: *mut cdCanvas, color: c_long) -> c_long;

    pub fn cdCanvasBackOpacity(canvas: *mut cdCanvas, opacity: c_int) -> c_int;

    pub fn cdCanvasWriteMode(canvas: *mut cdCanvas, mode: c_int) -> c_int;

    pub fn cdCanvasLineStyle(canvas: *mut cdCanvas, style: c_int) -> c_int;

    pub fn cdCanvasLineStyleDashes(canvas: *mut cdCanvas, dashes: *const c_int, count: c_int);

    pub fn cdCanvasLineWidth(canvas: *mut cdCanvas, width: c_int) -> c_int;

    pub fn cdCanvasLineJoin(canvas: *mut cdCanvas, join: c_int) -> c_int;

    pub fn cdCanvasLineCap(canvas: *mut cdCanvas, cap: c_int) -> c_int;

    pub fn cdCanvasInteriorStyle(canvas: *mut cdCanvas, style: c_int) -> c_int;

    pub fn cdCanvasHatch(canvas: *mut cdCanvas, style: c_int) -> c_int;

    pub fn cdCanvasStipple(canvas: *mut cdCanvas, w: c_int, h: c_int, stipple: *const c_uchar);

    pub fn cdCanvasGetStipple(canvas: *mut cdCanvas, n: *mut c_int, m: *mut c_int) -> *mut c_uchar;

    pub fn cdCanvasPattern(canvas: *mut cdCanvas, w: c_int, h: c_int, pattern: *const c_long);

    pub fn cdCanvasGetPattern(canvas: *mut cdCanvas, n: *mut c_int, m: *mut c_int) -> *mut c_long;

    pub fn cdCanvasFillMode(canvas: *mut cdCanvas, mode: c_int) -> c_int;

    pub fn cdCanvasFont(
        canvas: *mut cdCanvas,
        type_face: *const c_char,
        style: c_int,
        size: c_int,
    ) -> c_int;

    pub fn cdCanvasGetFont(
        canvas: *mut cdCanvas,
        type_face: *mut c_char,
        style: *mut c_int,
        size: *mut c_int,
    );

    pub fn cdCanvasNativeFont(canvas: *mut cdCanvas, font: *const c_char) -> *mut c_char;

    pub fn cdCanvasTextAlignment(canvas: *mut cdCanvas, alignment: c_int) -> c_int;

    pub fn cdCanvasTextOrientation(canvas: *mut cdCanvas, angle: f64) -> f64;

    pub fn cdCanvasMarkType(canvas: *mut cdCanvas, type_: c_int) -> c_int;

    pub fn cdCanvasMarkSize(canvas: *mut cdCanvas, size: c_int) -> c_int;

    pub fn cdCanvasVectorText(canvas: *mut cdCanvas, x: c_int, y: c_int, s: *const c_char);

    pub fn cdCanvasMultiLineVectorText(canvas: *mut cdCanvas, x: c_int, y: c_int, s: *const c_char);

    pub fn cdCanvasVectorFont(canvas: *mut cdCanvas, filename: *const c_char) -> *mut c_char;

    pub fn cdCanvasVectorTextDirection(
        canvas: *mut cdCanvas,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
    );

    pub fn cdCanvasVectorTextTransform(canvas: *mut cdCanvas, matrix: *const f64) -> *mut f64;

    pub fn cdCanvasVectorTextSize(
        canvas: *mut cdCanvas,
        size_x: c_int,
        size_y: c_int,
        s: *const c_char,
    );

    pub fn cdCanvasVectorCharSize(canvas: *mut cdCanvas, size: c_int) -> c_int;

    pub fn cdCanvasVectorFontSize(canvas: *mut cdCanvas, size_x: f64, size_y: f64);

    pub fn cdCanvasGetVectorFontSize(canvas: *mut cdCanvas, size_x: *mut f64, size_y: *mut f64);

    pub fn cdCanvasGetVectorTextSize(
        canvas: *mut cdCanvas,
        s: *const c_char,
        x: *mut c_int,
        y: *mut c_int,
    );

    pub fn cdCanvasGetVectorTextBounds(
        canvas: *mut cdCanvas,
        s: *const c_char,
        x: c_int,
        y: c_int,
        rect: *mut c_int,
    );

    pub fn cdCanvasGetVectorTextBox(
        canvas: *mut cdCanvas,
        x: c_int,
        y: c_int,
        s: *const c_char,
        xmin: *mut c_int,
        xmax: *mut c_int,
        ymin: *mut c_int,
        ymax: *mut c_int,
    );

    pub fn cdfCanvasVectorTextDirection(canvas: *mut cdCanvas, x1: f64, y1: f64, x2: f64, y2: f64);

    pub fn cdfCanvasVectorTextSize(
        canvas: *mut cdCanvas,
        size_x: f64,
        size_y: f64,
        s: *const c_char,
    );

    pub fn cdfCanvasGetVectorTextSize(
        canvas: *mut cdCanvas,
        s: *const c_char,
        x: *mut f64,
        y: *mut f64,
    );

    pub fn cdfCanvasVectorCharSize(canvas: *mut cdCanvas, size: f64) -> f64;

    pub fn cdfCanvasVectorText(canvas: *mut cdCanvas, x: f64, y: f64, s: *const c_char);

    pub fn cdfCanvasMultiLineVectorText(canvas: *mut cdCanvas, x: f64, y: f64, s: *const c_char);

    pub fn cdfCanvasGetVectorTextBounds(
        canvas: *mut cdCanvas,
        s: *const c_char,
        x: f64,
        y: f64,
        rect: *mut f64,
    );

    pub fn cdfCanvasGetVectorTextBox(
        canvas: *mut cdCanvas,
        x: f64,
        y: f64,
        s: *const c_char,
        xmin: *mut f64,
        xmax: *mut f64,
        ymin: *mut f64,
        ymax: *mut f64,
    );

    pub fn cdCanvasGetFontDim(
        canvas: *mut cdCanvas,
        max_width: *mut c_int,
        height: *mut c_int,
        ascent: *mut c_int,
        descent: *mut c_int,
    );

    pub fn cdCanvasGetTextSize(
        canvas: *mut cdCanvas,
        s: *const c_char,
        width: *mut c_int,
        height: *mut c_int,
    );

    pub fn cdCanvasGetTextBox(
        canvas: *mut cdCanvas,
        x: c_int,
        y: c_int,
        s: *const c_char,
        xmin: *mut c_int,
        xmax: *mut c_int,
        ymin: *mut c_int,
        ymax: *mut c_int,
    );

    pub fn cdfCanvasGetTextBox(
        canvas: *mut cdCanvas,
        x: f64,
        y: f64,
        s: *const c_char,
        xmin: *mut f64,
        xmax: *mut f64,
        ymin: *mut f64,
        ymax: *mut f64,
    );

    pub fn cdCanvasGetTextBounds(
        canvas: *mut cdCanvas,
        x: c_int,
        y: c_int,
        s: *const c_char,
        rect: *mut c_int,
    );

    pub fn cdfCanvasGetTextBounds(
        canvas: *mut cdCanvas,
        x: f64,
        y: f64,
        s: *const c_char,
        rect: *mut f64,
    );

    pub fn cdCanvasGetColorPlanes(canvas: *mut cdCanvas) -> c_int;

    pub fn cdCanvasPalette(canvas: *mut cdCanvas, n: c_int, palette: *const c_long, mode: c_int);

    pub fn cdCanvasGetImageRGB(
        canvas: *mut cdCanvas,
        r: *mut c_uchar,
        g: *mut c_uchar,
        b: *mut c_uchar,
        x: c_int,
        y: c_int,
        iw: c_int,
        ih: c_int,
    );

    pub fn cdCanvasPutImageRectRGB(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        r: *const c_uchar,
        g: *const c_uchar,
        b: *const c_uchar,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasPutImageRectRGBA(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        r: *const c_uchar,
        g: *const c_uchar,
        b: *const c_uchar,
        a: *const c_uchar,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasPutImageRectMap(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        index: *const c_uchar,
        colors: *const c_long,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdfCanvasPutImageRectRGB(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        r: *const c_uchar,
        g: *const c_uchar,
        b: *const c_uchar,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdfCanvasPutImageRectRGBA(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        r: *const c_uchar,
        g: *const c_uchar,
        b: *const c_uchar,
        a: *const c_uchar,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdfCanvasPutImageRectMap(
        canvas: *mut cdCanvas,
        iw: c_int,
        ih: c_int,
        index: *const c_uchar,
        colors: *const c_long,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasCreateImage(canvas: *mut cdCanvas, w: c_int, h: c_int) -> *mut cdImage;

    pub fn cdKillImage(image: *mut cdImage);

    pub fn cdCanvasGetImage(canvas: *mut cdCanvas, image: *mut cdImage, x: c_int, y: c_int);

    pub fn cdCanvasPutImageRect(
        canvas: *mut cdCanvas,
        image: *mut cdImage,
        x: c_int,
        y: c_int,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasScrollArea(
        canvas: *mut cdCanvas,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
        dx: c_int,
        dy: c_int,
    );

    pub fn cdCreateBitmap(w: c_int, h: c_int, type_: c_int) -> *mut cdBitmap;

    pub fn cdInitBitmap(w: c_int, h: c_int, type_: c_int, ...) -> *mut cdBitmap;

    pub fn cdKillBitmap(bitmap: *mut cdBitmap);

    pub fn cdBitmapGetData(bitmap: *mut cdBitmap, dataptr: c_int) -> *mut c_uchar;

    pub fn cdBitmapSetRect(
        bitmap: *mut cdBitmap,
        xmin: c_int,
        xmax: c_int,
        ymin: c_int,
        ymax: c_int,
    );

    pub fn cdCanvasPutBitmap(
        canvas: *mut cdCanvas,
        bitmap: *mut cdBitmap,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    );

    pub fn cdCanvasGetBitmap(canvas: *mut cdCanvas, bitmap: *mut cdBitmap, x: c_int, y: c_int);

    pub fn cdBitmapRGB2Map(bitmap_rgb: *mut cdBitmap, bitmap_map: *mut cdBitmap);

    pub fn cdEncodeColor(red: c_uchar, green: c_uchar, blue: c_uchar) -> c_long;

    pub fn cdEncodeColorAlpha(
        red: c_uchar,
        green: c_uchar,
        blue: c_uchar,
        alpha: c_uchar,
    ) -> c_long;

    pub fn cdEncodeAlpha(color: c_long, alpha: c_uchar) -> c_long;

    pub fn cdDecodeColor(color: c_long, red: *mut c_uchar, green: *mut c_uchar, blue: *mut c_uchar);

    pub fn cdDecodeColorAlpha(
        color: c_long,
        red: *mut c_uchar,
        green: *mut c_uchar,
        blue: *mut c_uchar,
        alpha: *mut c_uchar,
    );

    pub fn cdDecodeAlpha(color: c_long) -> c_uchar;

    pub fn cdRGB2Map(
        width: c_int,
        height: c_int,
        red: *const c_uchar,
        green: *const c_uchar,
        blue: *const c_uchar,
        index: *mut c_uchar,
        pal_size: c_int,
        color: *mut c_long,
    );
}

pub const CD_RGB: i32 = 0;
pub const CD_MAP: i32 = 1;
pub const CD_RGBA: i32 = 256;
pub const CD_IRED: i32 = 0;
pub const CD_IGREEN: i32 = 1;
pub const CD_IBLUE: i32 = 2;
pub const CD_IALPHA: i32 = 3;
pub const CD_INDEX: i32 = 4;
pub const CD_COLORS: i32 = 5;
pub const CD_ERROR: i32 = -1;
pub const CD_OK: i32 = 0;
pub const CD_CLIPOFF: i32 = 0;
pub const CD_CLIPAREA: i32 = 1;
pub const CD_CLIPPOLYGON: i32 = 2;
pub const CD_CLIPREGION: i32 = 3;
pub const CD_CLIPPATH: i32 = 4;
pub const CD_UNION: i32 = 0;
pub const CD_INTERSECT: i32 = 1;
pub const CD_DIFFERENCE: i32 = 2;
pub const CD_NOTINTERSECT: i32 = 3;
pub const CD_FILL: i32 = 0;
pub const CD_OPEN_LINES: i32 = 1;
pub const CD_CLOSED_LINES: i32 = 2;
pub const CD_CLIP: i32 = 3;
pub const CD_BEZIER: i32 = 4;
pub const CD_REGION: i32 = 5;
pub const CD_PATH: i32 = 6;
pub const CD_PATH_NEW: i32 = 0;
pub const CD_PATH_MOVETO: i32 = 1;
pub const CD_PATH_LINETO: i32 = 2;
pub const CD_PATH_ARC: i32 = 3;
pub const CD_PATH_CURVETO: i32 = 4;
pub const CD_PATH_CLOSE: i32 = 5;
pub const CD_PATH_FILL: i32 = 6;
pub const CD_PATH_STROKE: i32 = 7;
pub const CD_PATH_FILLSTROKE: i32 = 8;
pub const CD_PATH_CLIP: i32 = 9;
pub const CD_EVENODD: i32 = 0;
pub const CD_WINDING: i32 = 1;
pub const CD_MITER: i32 = 0;
pub const CD_BEVEL: i32 = 1;
pub const CD_ROUND: i32 = 2;
pub const CD_CAPFLAT: i32 = 0;
pub const CD_CAPSQUARE: i32 = 1;
pub const CD_CAPROUND: i32 = 2;
pub const CD_OPAQUE: i32 = 0;
pub const CD_TRANSPARENT: i32 = 1;
pub const CD_REPLACE: i32 = 0;
pub const CD_XOR: i32 = 1;
pub const CD_NOT_XOR: i32 = 2;
pub const CD_POLITE: i32 = 0;
pub const CD_FORCE: i32 = 1;
pub const CD_CONTINUOUS: i32 = 0;
pub const CD_DASHED: i32 = 1;
pub const CD_DOTTED: i32 = 2;
pub const CD_DASH_DOT: i32 = 3;
pub const CD_DASH_DOT_DOT: i32 = 4;
pub const CD_CUSTOM: i32 = 5;
pub const CD_PLUS: i32 = 0;
pub const CD_STAR: i32 = 1;
pub const CD_CIRCLE: i32 = 2;
pub const CD_X: i32 = 3;
pub const CD_BOX: i32 = 4;
pub const CD_DIAMOND: i32 = 5;
pub const CD_HOLLOW_CIRCLE: i32 = 6;
pub const CD_HOLLOW_BOX: i32 = 7;
pub const CD_HOLLOW_DIAMOND: i32 = 8;
pub const CD_HORIZONTAL: i32 = 0;
pub const CD_VERTICAL: i32 = 1;
pub const CD_FDIAGONAL: i32 = 2;
pub const CD_BDIAGONAL: i32 = 3;
pub const CD_CROSS: i32 = 4;
pub const CD_DIAGCROSS: i32 = 5;
pub const CD_SOLID: i32 = 0;
pub const CD_HATCH: i32 = 1;
pub const CD_STIPPLE: i32 = 2;
pub const CD_PATTERN: i32 = 3;
pub const CD_HOLLOW: i32 = 4;
pub const CD_CUSTOMPATTERN: i32 = 5;
pub const CD_NORTH: i32 = 0;
pub const CD_SOUTH: i32 = 1;
pub const CD_EAST: i32 = 2;
pub const CD_WEST: i32 = 3;
pub const CD_NORTH_EAST: i32 = 4;
pub const CD_NORTH_WEST: i32 = 5;
pub const CD_SOUTH_EAST: i32 = 6;
pub const CD_SOUTH_WEST: i32 = 7;
pub const CD_CENTER: i32 = 8;
pub const CD_BASE_LEFT: i32 = 9;
pub const CD_BASE_CENTER: i32 = 10;
pub const CD_BASE_RIGHT: i32 = 11;
pub const CD_PLAIN: i32 = 0;
pub const CD_BOLD: i32 = 1;
pub const CD_ITALIC: i32 = 2;
pub const CD_UNDERLINE: i32 = 4;
pub const CD_STRIKEOUT: i32 = 8;
pub const CD_SMALL: i32 = 8;
pub const CD_STANDARD: i32 = 12;
pub const CD_LARGE: i32 = 18;
pub const CD_CTX_WINDOW: i32 = 0;
pub const CD_CTX_DEVICE: i32 = 1;
pub const CD_CTX_IMAGE: i32 = 2;
pub const CD_CTX_FILE: i32 = 3;
pub type cdSizeCB = ::std::option::Option<
    unsafe extern "C" fn(canvas: *mut cdCanvas, w: c_int, h: c_int, w_mm: f64, h_mm: f64) -> c_int,
>;
pub const CD_A0: i32 = 0;
pub const CD_A1: i32 = 1;
pub const CD_A2: i32 = 2;
pub const CD_A3: i32 = 3;
pub const CD_A4: i32 = 4;
pub const CD_A5: i32 = 5;
pub const CD_LETTER: i32 = 6;
pub const CD_LEGAL: i32 = 7;
