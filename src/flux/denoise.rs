use super::Film;

pub struct Denoiser {
    color_filter: oidn2_sys::OIDNFilter,
    color_buf: oidn2_sys::OIDNBuffer,
    output_buf: oidn2_sys::OIDNBuffer,
    pub albedo_denoised: Film,
    pub normal_denoised: Film,
}

impl Denoiser {
    pub fn new(resolution: glam::UVec2, albedo: &Film, normal: &Film) -> Self {
        let area = (resolution.x * resolution.y) as usize;
        let buffer_dim = area * 3 * std::mem::size_of::<f32>();

        unsafe {
            let device = oidn2_sys::oidnNewDevice(0);
            oidn2_sys::oidnCommitDevice(device);

            // create buffers for input/output images

            let color_buf = oidn2_sys::oidnNewBuffer(device, buffer_dim);
            let albedo_buf = oidn2_sys::oidnNewBuffer(device, buffer_dim);
            let normal_buf = oidn2_sys::oidnNewBuffer(device, buffer_dim);
            let output_buf = oidn2_sys::oidnNewBuffer(device, buffer_dim);

            // create filter for denoising the beauty image

            let color_filter = oidn2_sys::oidnNewFilter(device, b"RT\0" as *const _ as _);
            oidn2_sys::oidnSetFilterImage(
                color_filter,
                b"color\0" as *const _ as _,
                color_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );
            oidn2_sys::oidnSetFilterImage(
                color_filter,
                b"albedo\0" as *const _ as _,
                albedo_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );
            oidn2_sys::oidnSetFilterImage(
                color_filter,
                b"normal\0" as *const _ as _,
                normal_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );
            oidn2_sys::oidnSetFilterImage(
                color_filter,
                b"output\0" as *const _ as _,
                output_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );

            // set filter parameters

            oidn2_sys::oidnSetFilterBool(color_filter, b"cleanAux\0" as *const _ as _, true);
            oidn2_sys::oidnSetFilterBool(color_filter, b"hdr\0" as *const _ as _, true);

            oidn2_sys::oidnCommitFilter(color_filter);

            // create separate filter for denoising the albedo channel in-place

            let albedo_filter = oidn2_sys::oidnNewFilter(device, b"RT\0" as *const _ as _);
            oidn2_sys::oidnSetFilterImage(
                albedo_filter,
                b"albedo\0" as *const _ as _,
                albedo_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );
            oidn2_sys::oidnSetFilterImage(
                albedo_filter,
                b"output\0" as *const _ as _,
                albedo_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );

            oidn2_sys::oidnCommitFilter(albedo_filter);

            // create a separate filter for denoising the normal channel in-place

            let normal_filter = oidn2_sys::oidnNewFilter(device, b"RT\0" as *const _ as _);
            oidn2_sys::oidnSetFilterImage(
                normal_filter,
                b"normal\0" as *const _ as _,
                normal_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );
            oidn2_sys::oidnSetFilterImage(
                normal_filter,
                b"output\0" as *const _ as _,
                normal_buf,
                oidn2_sys::OIDNFormat_OIDN_FORMAT_FLOAT3,
                resolution.x as _,
                resolution.y as _,
                0,
                0,
                0,
            );

            oidn2_sys::oidnCommitFilter(normal_filter);

            // fill buffers

            fill_buffer(albedo_buf, albedo);
            fill_buffer(normal_buf, normal);

            // prefilter auxiliary channels

            oidn2_sys::oidnExecuteFilter(albedo_filter);
            let albedo_buf_ptr = oidn2_sys::oidnGetBufferData(albedo_buf);
            let albedo_buf_data =
                std::slice::from_raw_parts_mut(albedo_buf_ptr as *mut f32, 3 * area);
            let albedo_denoised = Film::from_rgb_f32_slice(resolution, albedo_buf_data);

            oidn2_sys::oidnExecuteFilter(normal_filter);
            let normal_buf_ptr = oidn2_sys::oidnGetBufferData(normal_buf);
            let normal_buf_data =
                std::slice::from_raw_parts_mut(normal_buf_ptr as *mut f32, 3 * area);
            let normal_denoised = Film::from_rgb_f32_slice(resolution, normal_buf_data);

            Self {
                color_filter,
                color_buf,
                output_buf,
                albedo_denoised,
                normal_denoised,
            }
        }
    }

    pub fn denoise(&self, color: &Film) -> Film {
        fill_buffer(self.color_buf, color);
        let color_buf_data = unsafe {
            oidn2_sys::oidnExecuteFilter(self.color_filter);

            let output_buf_ptr = oidn2_sys::oidnGetBufferData(self.output_buf);
            let buf_size = (color.resolution.x * color.resolution.y) as usize;
            std::slice::from_raw_parts_mut(output_buf_ptr as *mut f32, 3 * buf_size)
        };
        Film::from_rgb_f32_slice(color.resolution, color_buf_data)
    }
}

fn fill_buffer(color_buf: oidn2_sys::OIDNBuffer, film: &Film) {
    let color_buf_data = unsafe {
        let color_buf_ptr = oidn2_sys::oidnGetBufferData(color_buf);
        let buf_size = (film.resolution.x * film.resolution.y) as usize;
        std::slice::from_raw_parts_mut(color_buf_ptr as *mut f32, 3 * buf_size)
    };
    for y in 0..film.resolution.y {
        for x in 0..film.resolution.x {
            let pixel = film.pixel(x, y);
            let index = (y * film.resolution.x + x) as usize;
            let color = pixel.color();

            color_buf_data[index * 3] = color.x;
            color_buf_data[index * 3 + 1] = color.y;
            color_buf_data[index * 3 + 2] = color.z;
        }
    }
}
