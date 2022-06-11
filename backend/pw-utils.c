#include <spa/buffer/buffer.h>
#include <spa/buffer/meta.h>
#include <spa/param/format.h>
#include <spa/param/format-utils.h>
#include <spa/param/param.h>
#include <spa/param/video/format.h>
#include <spa/param/video/format-utils.h>
#include <spa/param/video/raw.h>
#include <spa/pod/builder.h>
#include <spa/pod/pod.h>
#include <spa/utils/defs.h>
#include <spa/utils/type.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define CURSOR_META_SIZE(width, height) \
 (sizeof(struct spa_meta_cursor) + \
  sizeof(struct spa_meta_bitmap) + width * height * 4) 

struct spa_pod *build_video_format() {
    uint8_t *buffer = malloc(1024);
    struct spa_pod_builder b = SPA_POD_BUILDER_INIT(buffer, 1024);
    struct spa_pod *ret = spa_pod_builder_add_object(&b,
        SPA_TYPE_OBJECT_Format, SPA_PARAM_EnumFormat,
        SPA_FORMAT_mediaType, SPA_POD_Id(SPA_MEDIA_TYPE_video),
        SPA_FORMAT_mediaSubtype, SPA_POD_Id(SPA_MEDIA_SUBTYPE_raw),
        SPA_FORMAT_VIDEO_format, SPA_POD_Id(SPA_VIDEO_FORMAT_BGRx),
        SPA_FORMAT_VIDEO_framerate, SPA_POD_CHOICE_RANGE_Fraction(
            &SPA_FRACTION(25, 1), &SPA_FRACTION(0, 1), &SPA_FRACTION(360, 1)
        )
    );

    return ret;
}

void on_param_changed(uint32_t id, const struct spa_pod *param, struct spa_pod **out) {
    struct spa_pod_builder b;
    uint8_t *buffer;
    struct spa_video_info format;
    int result;

    if (!param || id != SPA_PARAM_Format)
        return;

    result = spa_format_parse(param, &format.media_type, &format.media_subtype);
    if (result < 0)
        return;

    if (format.media_type != SPA_MEDIA_TYPE_video
        || format.media_subtype != SPA_MEDIA_SUBTYPE_raw)
        return;

    spa_format_video_raw_parse(param, &format.info.raw);

    buffer = malloc(1024);
    b = SPA_POD_BUILDER_INIT(buffer, 1024);

    /* Video crop */
    out[0] = spa_pod_builder_add_object(&b,
        SPA_TYPE_OBJECT_ParamMeta, SPA_PARAM_Meta,
        SPA_PARAM_META_type, SPA_POD_Id(SPA_META_VideoCrop),
        SPA_PARAM_META_size, SPA_POD_Int(sizeof(struct spa_meta_region))
    );

    /* Cursor */
    out[1] = spa_pod_builder_add_object(&b,
        SPA_TYPE_OBJECT_ParamMeta, SPA_PARAM_Meta,
        SPA_PARAM_META_type, SPA_POD_Id(SPA_META_Cursor),
        SPA_PARAM_META_size, SPA_POD_CHOICE_RANGE_Int(
            CURSOR_META_SIZE(64, 64),
            CURSOR_META_SIZE(1, 1),
            CURSOR_META_SIZE(1024, 1024)
        )
    );

    /* Buffer options */
    out[2] = spa_pod_builder_add_object(&b,
        SPA_TYPE_OBJECT_ParamBuffers, SPA_PARAM_Buffers,
        SPA_PARAM_BUFFERS_dataType, SPA_POD_Int(
            (1 << SPA_DATA_MemPtr) |
            (1 << SPA_DATA_DmaBuf)
        )
    );
}
