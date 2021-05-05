use crate::imath::{Box2, Vec2};
use crate::{
    refptr::Ref, Box2iAttribute, ChannelList, Compression, Error, LineOrder,
    PreviewImage, TileDescription, TypedAttribute,
};
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

use std::ffi::{CStr, CString};

#[repr(transparent)]
pub struct Header(pub(crate) *mut sys::Imf_Header_t);

unsafe impl crate::refptr::OpaquePtr for Header {
    type SysPointee = sys::Imf_Header_t;
    type Pointee = Header;
}

pub type HeaderRef<'a, Owner, P = Header> = Ref<'a, Owner, P>;

impl Header {
    /// Construct a new [`Header`] with the given attributes.
    ///
    /// # Arguments
    /// * `data_window` - The window which contains data. Typically will be the
    /// same as the display window, but may be smaller to represent a crop region
    /// or larger to represent overscan.
    /// * `display_window` - The window which represents the size of the
    /// displayable image. Typically will be [[0, width-1], [0, height-1]].
    /// * `pixel_aspect_ratio` - The ratio of the pixel `width/height`, e.g. 2.0
    /// for anamorphic.
    /// * `screen_window_center` - The center of the screen window. Will be [0,0]
    /// for images that were not generated by perspective projection.
    /// * `screen_window_width` - The width of the screen window. Will be 1.0 for
    /// images that were not generated by perspective projection
    /// * `line_order` - The vertical order in which scanlines are stored. This
    /// is a hint for readers and may not be respected.
    /// * `compression` - The compression scheme to use to store all image data.
    ///
    pub fn new<B, V>(
        data_window: B,
        display_window: B,
        pixel_aspect_ratio: f32,
        screen_window_center: V,
        screen_window_width: f32,
        line_order: LineOrder,
        compression: Compression,
    ) -> Result<Header>
    where
        B: Box2<i32>,
        V: Vec2<f32>,
    {
        let mut header = std::ptr::null_mut();
        unsafe {
            sys::Imf_Header_ctor(
                &mut header,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                display_window.as_ptr() as *const sys::Imath_Box2i_t,
                pixel_aspect_ratio,
                screen_window_center.as_ptr() as *const sys::Imath_V2f_t,
                screen_window_width,
                line_order.into(),
                compression.into(),
            )
            .into_result()?;
        }

        Ok(Header(header))
    }

    /// Construct a new [`Header`] with the given attributes.
    ///
    /// # Arguments
    /// * `width` - The width of the image, setting both data and display
    /// windows
    /// * `height` - The height of the image, setting both data and display
    /// windows
    /// displayable image. Typically will be [[0, width-1], [0, height-1]].
    /// * `pixel_aspect_ratio` - The ratio of the pixel `width/height`, e.g. 2.0
    /// for anamorphic.
    /// * `screen_window_center` - The center of the screen window. Will be [0,0]
    /// for images that were not generated by perspective projection.
    /// * `screen_window_width` - The width of the screen window. Will be 1.0 for
    /// images that were not generated by perspective projection
    /// * `line_order` - The vertical order in which scanlines are stored. This
    /// is a hint for readers and may not be respected.
    /// * `compression` - The compression scheme to use to store all image data.
    ///
    pub fn with_dimensions<V>(
        width: i32,
        height: i32,
        pixel_aspect_ratio: f32,
        screen_window_center: V,
        screen_window_width: f32,
        line_order: LineOrder,
        compression: Compression,
    ) -> Result<Header>
    where
        V: Vec2<f32>,
    {
        let mut header = std::ptr::null_mut();
        unsafe {
            sys::Imf_Header_with_dimensions(
                &mut header,
                width,
                height,
                pixel_aspect_ratio,
                screen_window_center.as_ptr() as *const sys::Imath_V2f_t,
                screen_window_width,
                line_order.into(),
                compression.into(),
            )
            .into_result()?;
        }

        Ok(Header(header))
    }

    /// Shortcut to construct a new [`Header`] with just the dimensions and
    /// everything else Default
    ///
    pub fn from_dimensions(width: i32, height: i32) -> Header {
        let mut header = Header::default();
        header.set_dimensions(width, height);
        header
    }

    /// Examines the header and returns an error if it finds something wrong
    /// with the attributes (e.g. empty display window, negative pixel aspect
    /// ratio etc.)
    ///
    /// # Arguments
    /// * `is_tiled` - This header should represent a tiled file
    /// * `is_multi_part` - This header should represent a multi-part file
    ///
    /// # Returns
    /// * `Ok(())` - If no error is found
    /// * `Err(Error::UNIMPLEMENTED)` - If an issue is found
    ///
    pub fn sanity_check(
        &self,
        is_tiled: bool,
        is_multi_part: bool,
    ) -> Result<()> {
        unsafe {
            sys::Imf_Header_sanityCheck(self.0, is_tiled, is_multi_part)
                .into_result()?;
        }
        Ok(())
    }

    /// [`Header::sanity_check()`] will throw an exception if the width or
    /// height of the data window exceeds the maximum image width or height, or
    /// if the size of a tile exceeds the maximum tile width or height.
    ///
    /// At program startup the maximum image and tile width and height
    /// are set to zero, meaning that width and height are unlimited.
    ///
    /// Limiting image and tile width and height limits how much memory
    /// will be allocated when a file is opened.  This can help protect
    /// applications from running out of memory while trying to read
    /// a damaged image file.
    ///
    pub fn set_max_image_size(max_width: i32, max_height: i32) {
        unsafe {
            sys::Imf_3_0__Header_setMaxImageSize(max_width, max_height)
                .into_result()
                .unwrap();
        }
    }

    /// [`Header::sanity_check()`] will throw an exception if the width or
    /// height of the data window exceeds the maximum image width or height, or
    /// if the size of a tile exceeds the maximum tile width or height.
    ///
    /// At program startup the maximum image and tile width and height
    /// are set to zero, meaning that width and height are unlimited.
    ///
    /// Limiting image and tile width and height limits how much memory
    /// will be allocated when a file is opened.  This can help protect
    /// applications from running out of memory while trying to read
    /// a damaged image file.
    ///
    pub fn set_max_tile_size(max_width: i32, max_height: i32) {
        unsafe {
            sys::Imf_3_0__Header_setMaxTileSize(max_width, max_height)
                .into_result()
                .unwrap();
        }
    }

    /// Check if the header reads nothing
    ///
    /// FIXME: This should be a const method in C++ but it's not - patch OpenEXR?
    ///
    pub fn reads_nothing(&mut self) -> bool {
        let mut result = false;
        unsafe {
            sys::Imf_Header_readsNothing(self.0, &mut result)
                .into_result()
                .unwrap()
        };
        result
    }
}

impl Default for Header {
    /// Creates a default header
    ///
    /// The resulting header has parameters:
    /// * `width` = 64
    /// * `height` = 64
    /// * `pixel_aspect_ratio` = 1.0
    /// * `screen_window_center` = [0.0, 0.0]
    /// * `screen_window_width` = 1.0
    /// * `line_order` = LineOrder::IncreasingY
    /// * `compression` = Compression::Zip
    ///
    fn default() -> Header {
        Header::with_dimensions(
            64,
            64,
            1.0f32,
            [0.0f32, 0.0f32],
            1.0f32,
            LineOrder::IncreasingY,
            Compression::Zip,
        )
        .unwrap()
    }
}

impl Header {
    //! # Standard attributes
    //!
    //! These methods can be used to get and set the required attributes of a
    //! standard OpenEXR file. The attributes `name`, `type` and
    //! `maxSamplesPerPixel` are only required for deep and multi-part images.

    /// Get a reference to the display window.
    ///
    /// The display window represents the rectangular region in pixel space that
    /// we wish to display. This typically correlates to what we normally think
    /// of as the "width" and "height" of the image, such that the display
    /// window rectangle is defined as a min/max inclusive pair of points
    /// (0, 0), (width-1, height-1).
    ///
    /// The display window must be the same for all parts in a file.
    ///
    /// Pixel space is a 2D coordinate system with X increasing from left to
    /// right and Y increasing from top to bottom.
    ///
    pub fn display_window<B>(&self) -> &B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_displayWindow_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_Box2i_t as *const B)
        }
    }

    /// Get a mutable reference to the display window.
    ///
    /// The display window represents the rectangular region in pixel space that
    /// we wish to display. This typically correlates to what we normally think
    /// of as the "width" and "height" of the image, such that the display
    /// window rectangle is defined as a min/max inclusive pair of points
    /// (0, 0), (width-1, height-1).
    ///
    /// The display window must be the same for all parts in a file.
    ///
    /// Pixel space is a 2D coordinate system with X increasing from left to
    /// right and Y increasing from top to bottom.
    ///
    pub fn display_window_mut<B>(&mut self) -> &mut B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_displayWindow(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_Box2i_t as *mut B)
        }
    }

    /// Get a reference to the data window
    ///
    /// The data window represents the rectangular region of the image for which
    /// pixel data is defined in the file. Attempting to read or write data
    /// outside of that region is an error. For a "normal" image, the data
    /// window corresponds exactly to the display window, but for special cases
    /// may be different. For example it is common to only render a small section
    /// of the image ("crop region"), in which case the data window will be
    /// smaller than the display window, or to to render extra pixels outside of
    /// the display window ("overscan"), in which case the data window will be
    /// larger than the display window.
    ///
    pub fn data_window<B>(&self) -> &B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_dataWindow_const(self.0, &mut ptr)
                .into_result()
                .unwrap();

            &*(ptr as *const sys::Imath_Box2i_t as *const B)
        }
    }

    /// Get a mutable reference to the data window
    ///
    /// The data window represents the rectangular region of the image for which
    /// pixel data is defined in the file. Attempting to read or write data
    /// outside of that region is an error. For a "normal" image, the data
    /// window corresponds exactly to the display window, but for special cases
    /// may be different. For example it is common to only render a small section
    /// of the image ("crop region"), in which case the data window will be
    /// smaller than the display window, or to to render extra pixels outside of
    /// the display window ("overscan"), in which case the data window will be
    /// larger than the display window.
    ///
    pub fn data_window_mut<B>(&mut self) -> &mut B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_dataWindow(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_Box2i_t as *mut B)
        }
    }

    /// Set both display and data windows to [[0, 0], [width-1, height-1]]
    ///
    pub fn set_dimensions(&mut self, width: i32, height: i32) {
        *self.data_window_mut() = [0, 0, width - 1, height - 1];
        *self.display_window_mut() = [0, 0, width - 1, height - 1];
    }

    /// Get the pixel aspect ratio
    ///
    /// Given d_x, the difference between pixel locations (x, y) and (x+1, y),
    /// and d_y, difference between pixel locations (x, y) and (x, y+1) on the
    /// the display, the pixel aspect ratio is the ratio d_x / d_y when the image
    /// is displayed dusch that the aspect ratio width/height is as intended.
    ///
    /// The pixel aspect ratio must be the same for all parts in a file.
    ///
    /// A normal image thus has a pixel aspect ratio of 1.0, while it is 2.0
    /// for an anamorphic image.
    ///
    pub fn pixel_aspect_ratio(&self) -> f32 {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_pixelAspectRatio_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *ptr
        }
    }

    /// Set the pixel aspect ratio
    ///
    /// Given d_x, the difference between pixel locations (x, y) and (x+1, y),
    /// and d_y, difference between pixel locations (x, y) and (x, y+1) on the
    /// the display, the pixel aspect ratio is the ratio d_x / d_y when the image
    /// is displayed dusch that the aspect ratio width/height is as intended.
    ///
    /// The pixel aspect ratio must be the same for all parts in a file.
    ///
    /// A normal image thus has a pixel aspect ratio of 1.0, while it is 2.0
    /// for an anamorphic image.
    ///
    pub fn set_pixel_aspect_ratio(&mut self, par: f32) {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_pixelAspectRatio(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *ptr = par;
        }
    }

    /// Get a reference to the screen window center
    ///
    /// The screen window represents the bounding rectangle of the image on the
    /// `z=1` plane assuming the image was generated by perspective projection
    /// with a width, `W`, and a center, `C`. The height of the window can be
    /// derived from the center and the pixel aspect ratio.
    ///
    /// Images that were not generated by perspective projection should have
    /// their screen window width set to 1 and their center to (0,0).
    ///
    pub fn screen_window_center<B>(&self) -> &B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_screenWindowCenter_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imath_Box2i_t as *const B)
        }
    }

    /// Get a mutable reference to the screen window center
    ///
    /// The screen window represents the bounding rectangle of the image on the
    /// `z=1` plane assuming the image was generated by perspective projection
    /// with a width, `W`, and a center, `C`. The height of the window can be
    /// derived from the center and the pixel aspect ratio.
    ///
    /// Images that were not generated by perspective projection should have
    /// their screen window width set to 1 and their center to (0,0).
    ///
    pub fn screen_window_center_mut<B>(&mut self) -> &mut B
    where
        B: Box2<i32>,
    {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_screenWindowCenter(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imath_Box2i_t as *mut B)
        }
    }

    /// Get a reference to the screen window width
    ///
    /// The screen window represents the bounding rectangle of the image on the
    /// `z=1` plane assuming the image was generated by perspective projection
    /// with a width, `W`, and a center, `C`. The height of the window can be
    /// derived from the center and the pixel aspect ratio.
    ///
    /// Images that were not generated by perspective projection should have
    /// their screen window width set to 1 and their center to (0,0).
    ///
    pub fn screen_window_width(&self) -> &f32 {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_screenWindowWidth_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*ptr
        }
    }

    /// Get a mutable reference to the screen window width
    ///
    /// The screen window represents the bounding rectangle of the image on the
    /// `z=1` plane assuming the image was generated by perspective projection
    /// with a width, `W`, and a center, `C`. The height of the window can be
    /// derived from the center and the pixel aspect ratio.
    ///
    /// Images that were not generated by perspective projection should have
    /// their screen window width set to 1 and their center to (0,0).
    ///
    pub fn screen_window_width_mut(&mut self) -> &f32 {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_screenWindowWidth(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *ptr
        }
    }

    /// Get a reference to the list of channels in the header
    pub fn channels(&self) -> &ChannelList {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_channels_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &*(ptr as *const sys::Imf_ChannelList_t as *const ChannelList)
        }
    }

    /// Get a mutable reference to the list of channels in the header
    pub fn channels_mut(&mut self) -> &mut ChannelList {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_channels(self.0, &mut ptr)
                .into_result()
                .unwrap();
            &mut *(ptr as *mut sys::Imf_ChannelList_t as *mut ChannelList)
        }
    }

    /// Get the line order from the header
    ///
    /// Specifies the order in which rows of pixels are stored in the file,
    /// either [`LineOrder::IncreasingY`], [`LineOrder::DecreasingY`] or
    /// [`LineOrder::RandomY`] for tiled images.
    ///
    /// This does not affect the pixel space coordinates, only the order in
    /// which the data is stored.
    ///
    pub fn line_order(&self) -> LineOrder {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Header_lineOrder_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            (*ptr).into()
        }
    }

    /// Set the line order in the header
    ///
    /// Specifies the order in which rows of pixels are stored in the file,
    /// either [`LineOrder::IncreasingY`], [`LineOrder::DecreasingY`] or
    /// [`LineOrder::RandomY`] for tiled images.
    ///
    /// This does not affect the pixel space coordinates, only the order in
    /// which the data is stored.
    ///
    pub fn set_line_order(&mut self, lo: LineOrder) {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_Header_lineOrder(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *ptr = lo.into();
        };
    }

    /// Get the compression type from the header
    ///
    /// Defines the compression scheme used to store all pixel data.
    ///
    pub fn compression(&self) -> Compression {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Header_compression_const(self.0, &mut ptr)
                .into_result()
                .unwrap();
            (*ptr).into()
        }
    }

    /// Set the compression type in the header
    ///
    /// Defines the compression scheme used to store all pixel data.
    ///
    pub fn set_compression(&mut self, cmp: Compression) {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_Header_compression(self.0, &mut ptr)
                .into_result()
                .unwrap();
            *ptr = cmp.into();
        }
    }
}

impl Header {
    //! # Required attributes for multi-part files
    //!
    //! These attributes are all mandatory for multi-part files and optional
    //! for single-part files.

    /// Get the name of this part from the header
    ///
    /// Names must be unique, that is no two parts in the same file may share
    /// the same name.
    ///
    pub fn name(&self) -> String {
        unsafe {
            let mut s = std::ptr::null();
            sys::Imf_Header_name_const(self.0, &mut s)
                .into_result()
                .unwrap();

            let mut cptr = std::ptr::null();
            sys::std___cxx11_string_c_str(s, &mut cptr)
                .into_result()
                .unwrap();

            CStr::from_ptr(cptr).to_string_lossy().to_string()
        }
    }

    /// Set the name of this part in the header
    ///
    /// Names must be unique, that is no two parts in the same file may share
    /// the same name.
    ///
    pub fn set_name(&mut self, name: &str) {
        unsafe {
            let cname = CString::new(name).expect("Inner NUL bytes in name");
            // FIXME:
            // this is quite the dance we have to do for std::string
            // the issue is that all the overloads of std::string() that take
            // a const char* also take an implicit allocator, which we don't
            // want to bind.
            // We can get around this by implementing ignored parameters in
            // cppmm
            let mut s = std::ptr::null_mut();
            sys::std___cxx11_string_ctor(&mut s);
            let mut dummy = std::ptr::null_mut();
            sys::std___cxx11_string_assign(
                s,
                &mut dummy,
                cname.as_ptr(),
                cname.as_bytes().len() as u64,
            );
            sys::Imf_Header_setName(self.0, s);
            sys::std___cxx11_string_dtor(s);
        }
    }

    /// Get the image type of this part from the header
    ///
    /// This must be one of:
    /// * `scanlineimage` - Flat, scanline-based.
    /// * `tiledimage` - Flat, tiled.
    /// * `deepscanline` - Deep, scanline-based.
    /// * `deeptile` - Deep, tiled.
    ///
    /// FIXME: Make this return an enum instead of a string
    ///
    pub fn image_type(&self) -> String {
        unsafe {
            let mut s = std::ptr::null();
            sys::Imf_Header_type_const(self.0, &mut s);

            let mut cptr = std::ptr::null();
            sys::std___cxx11_string_c_str(s, &mut cptr);
            CStr::from_ptr(cptr).to_string_lossy().to_string()
        }
    }

    /// Set the image type of this part in the header
    ///
    /// This must be one of:
    /// * `scanlineimage` - Flat, scanline-based.
    /// * `tiledimage` - Flat, tiled.
    /// * `deepscanline` - Deep, scanline-based.
    /// * `deeptile` - Deep, tiled.
    ///
    /// FIXME: Make this take an enum instead of a string
    ///
    pub fn set_image_type(&mut self, image_type: &str) {
        unsafe {
            let cimage_type = CString::new(image_type)
                .expect("Inner NUL bytes in image_type");
            // FIXME:
            // this is quite the dance we have to do for std::string
            // the issue is that all the overloads of std::string() that take
            // a const char* also take an implicit allocator, which we don't
            // want to bind.
            // We can get around this by implementing ignored parameters in
            // cppmm
            let mut s = std::ptr::null_mut();
            sys::std___cxx11_string_ctor(&mut s);
            let mut dummy = std::ptr::null_mut();
            sys::std___cxx11_string_assign(
                s,
                &mut dummy,
                cimage_type.as_ptr(),
                cimage_type.as_bytes().len() as u64,
            );
            sys::Imf_Header_setType(self.0, s);
            sys::std___cxx11_string_dtor(s);
        }
    }

    /// Get the version of the file
    ///
    pub fn version(&self) -> i32 {
        unsafe {
            let mut v = std::ptr::null();
            sys::Imf_Header_version_const(self.0, &mut v);
            *v
        }
    }

    /// Set the version of the file
    ///
    pub fn set_version(&mut self, v: i32) {
        unsafe {
            sys::Imf_Header_setVersion(self.0, v);
        }
    }

    /// Does the file have its version specified?
    pub fn has_version(&self) -> bool {
        unsafe {
            let mut v = false;
            sys::Imf_Header_hasVersion(self.0, &mut v);
            v
        }
    }
}

impl Header {
    //! # Chunk count
    //!
    //! Chunk count is set automatically when writing the file

    /// Does the file have its chunk count specified?
    pub fn has_chunk_count(&self) -> bool {
        unsafe {
            let mut v = false;
            sys::Imf_Header_hasChunkCount(self.0, &mut v);
            v
        }
    }

    /// Get the chunk_count of the file
    ///
    pub fn chunk_count(&self) -> i32 {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_Header_chunkCount_const(self.0, &mut ptr);
            *ptr
        }
    }
}

impl Header {
    //! # Views
    //!
    //! View names must be unique, that is no two parts in the same file may share
    //! the same view. Only supported for multi-part files, deprecated for
    //! single-part files.

    /// Get the view of this part from the header
    ///
    pub fn view(&self) -> String {
        unsafe {
            let mut s = std::ptr::null();
            sys::Imf_Header_view_const(self.0, &mut s);
            let mut cptr = std::ptr::null();
            sys::std___cxx11_string_c_str(s, &mut cptr);
            CStr::from_ptr(cptr).to_string_lossy().to_string()
        }
    }

    /// Set the view of this part in the header
    ///
    pub fn set_view(&mut self, view: &str) {
        unsafe {
            let cview = CString::new(view).expect("Inner NUL bytes in view");
            // FIXME:
            // this is quite the dance we have to do for std::string
            // the issue is that all the overloads of std::string() that take
            // a const char* also take an implicit allocator, which we don't
            // want to bind.
            // We can get around this by implementing ignored parameters in
            // cppmm
            let mut s = std::ptr::null_mut();
            sys::std___cxx11_string_ctor(&mut s);
            let mut dummy = std::ptr::null_mut();
            sys::std___cxx11_string_assign(
                s,
                &mut dummy,
                cview.as_ptr(),
                cview.as_bytes().len() as u64,
            );
            sys::Imf_Header_setView(self.0, s);
            sys::std___cxx11_string_dtor(s);
        }
    }

    /// Does the part have a view specified?
    pub fn has_view(&self) -> bool {
        unsafe {
            let mut v = false;
            sys::Imf_Header_hasView(self.0, &mut v);
            v
        }
    }
}

impl Header {
    //! # Tile Description
    //!
    //! The tile description is a [`TileDescriptionAttribute`] whose name is
    //! `"tiles"`. It is mandatory for tiled files. The [`TiledDescription`]
    //! describes various properties of the tiles that make up the image file.

    /// Get the tile description from the header
    ///
    pub fn tile_description(&self) -> &TileDescription {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Header_tileDescription_const(self.0, &mut ptr);
            &*ptr
        }
    }

    /// Set the tile description in the header
    ///
    pub fn set_tile_description(&mut self, td: &TileDescription) {
        unsafe {
            sys::Imf_Header_setTileDescription(self.0, td);
        }
    }

    /// Does the part have a tile description?
    ///
    pub fn has_tile_description(&self) -> bool {
        unsafe {
            let mut v = false;
            sys::Imf_Header_hasTileDescription(self.0, &mut v);
            v
        }
    }
}

impl Header {
    //! # Preview Image
    //!
    //! The preview image ias a [`PreviewImageAttribute`] whose name is
    //! `"preview"`.
    //! This attribute is special -- while an image file is being written,
    //! the pixels of the preview image can be changed repeatedly by calling
    //! [`OutputFile::updatePreviewImage()`]

    /// Get the preview image from the header
    ///
    pub fn preview_image(&self) -> &PreviewImage {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_Header_previewImage_const(self.0, &mut ptr);
            &*(ptr as *const PreviewImage)
        }
    }

    /// Set the preview image in the header
    ///
    pub fn set_preview_image(&mut self, pi: &PreviewImage) {
        unsafe {
            sys::Imf_Header_setPreviewImage(self.0, pi.0);
        }
    }

    /// Does the part have a preview image?
    ///
    pub fn has_preview_image(&self) -> bool {
        unsafe {
            let mut v = false;
            sys::Imf_Header_hasPreviewImage(self.0, &mut v);
            v
        }
    }
}

impl Header {
    //! # Modifying user attributes

    /// Inserts the given metadata attribute with the given name
    ///
    pub fn insert<A>(&mut self, name: &str, attribute: &A) -> Result<()>
    where
        A: TypedAttribute,
    {
        let c_name = CString::new(name).expect("Invalid UTF-8 in name");
        unsafe {
            sys::Imf_Header_insert(
                self.0,
                c_name.as_ptr(),
                attribute.as_attribute_ptr(),
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Erases the attribute with the given name.
    ///
    /// If no attribute with `name` exists, the [`Header`] is unchanged.
    ///
    pub fn erase(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name).expect("Invalid UTF-8 in name");
        unsafe {
            sys::Imf_Header_erase(self.0, c_name.as_ptr()).into_result()?;
        }
        Ok(())
    }

    /// Get a reference to the Box2iAttribute with the given name
    ///
    /// # Returns
    /// * `Some(&Box2iAttribute)` - If the attribute exists
    /// * `None` - Otherwise
    ///
    pub fn find_typed_attribute_box2i(
        &self,
        name: &str,
    ) -> Option<&Box2iAttribute> {
        let c_name = CString::new(name).expect("Invalid UTF-8 in name");
        let mut attr_ptr = std::ptr::null();
        unsafe {
            sys::Imf_Header_findTypedAttribute_Box2i_const(
                self.0,
                &mut attr_ptr,
                c_name.as_ptr(),
            )
        };

        if !attr_ptr.is_null() {
            Some(unsafe {
                // We can do this as Attribute is a #[repr(transparent)] wrapper
                // over Imf_Attribute_t
                &*(attr_ptr as *const sys::Imf_Box2iAttribute_t
                    as *const Box2iAttribute)
            })
        } else {
            None
        }
    }

    /// Get a mutable reference to the Box2iAttribute with the given name
    ///
    /// # Returns
    /// * `Some(&mut Box2iAttribute)` - If the attribute exists
    /// * `None` - Otherwise
    ///
    pub fn find_typed_attribute_box2i_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut Box2iAttribute> {
        let c_name = CString::new(name).expect("Invalid UTF-8 in name");
        let mut attr_ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_Header_findTypedAttribute_Box2i(
                self.0,
                &mut attr_ptr,
                c_name.as_ptr(),
            )
        };

        if !attr_ptr.is_null() {
            Some(unsafe {
                // We can do this as Attribute is a #[repr(transparent)] wrapper
                // over Imf_Attribute_t
                &mut *(attr_ptr as *mut sys::Imf_Box2iAttribute_t
                    as *mut Box2iAttribute)
            })
        } else {
            None
        }
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_Header_dtor(self.0);
        }
    }
}
