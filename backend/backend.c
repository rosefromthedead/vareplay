#define _GNU_SOURCE
#include <dirent.h>
#include <libdrm/drm_fourcc.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <va/va.h>
#include <va/va_drm.h>
#include <va/va_drmcommon.h>
#include <unistd.h>

int32_t is_cart(const struct dirent *ent);
int32_t init_va_(int drm_conn);
int print_profiles(int drm_fd);
int32_t init_va();
int32_t import_buf(int fd, int width, int height, int size, int n_planes, uint32_t *strides,
                   uint32_t *offsets, uint32_t fourcc);
uint32_t n_planes(uint32_t fourcc);

int32_t is_card(const struct dirent *ent) {
    if (strncmp("card", ent->d_name, 4))
        return 0;
    return 1;
}

int32_t init_va_(int drm_conn) {
    struct dirent **list;
    int n;

    int dri = open("/dev/dri/", O_DIRECTORY);
    n = scandirat(dri, ".", &list, is_card, versionsort);
    for (int i = 0; i < n; i++) {
        int card = openat(dri, list[i]->d_name, 0);
        printf("%s:\n", list[i]->d_name);
        int err = print_profiles(card);
        if (err)
            printf("bad!! %d\n", err);
        close(card);
    }

    

    return 0;
}

int print_profiles(int drm_fd) {
    VAStatus err;
    int ver_maj, ver_min;
    int max_profiles, n_profiles;
    VAProfile *profiles;
    int max_entrypoints, n_entrypoints;
    VAEntrypoint *entrypoints = NULL;

    VADisplay *dpy = vaGetDisplayDRM(drm_fd);
    if (dpy == NULL) {
        printf("\tno display from card - VA unsupported?\n");
        return 0;
    }

    err = vaInitialize(dpy, &ver_maj, &ver_min);
    if (err) {
        printf("\tvaInitialize error: %d\n", err);
        return 0;
    }

    max_profiles = vaMaxNumProfiles(dpy);
    profiles = malloc(max_profiles * sizeof(VAProfile));
    err = vaQueryConfigProfiles(dpy, profiles, &n_profiles);
    if (err)
        return 0;

    for (int i = 0; i < n_profiles; i++) {
        printf("\tva profile: %d\n", profiles[i]);

        max_entrypoints = vaMaxNumEntrypoints(dpy);
        if (entrypoints)
            free(entrypoints);
        entrypoints = malloc(n_entrypoints * sizeof(VAEntrypoint));
        err = vaQueryConfigEntrypoints(dpy, profiles[i], entrypoints, &n_entrypoints);
        if (err) {
            printf("\t\terror getting entrypoints: %s\n", vaErrorStr(err));
            continue;
        }

        for (int j = 0; j < n_entrypoints; j++) {
            printf("\t\tentrypoint: %d\n", entrypoints[j]);
        }
    }
    
    if (entrypoints)
        free(entrypoints);

    free(profiles);
    return 0;
}

int32_t init_va() {
    int drm;
    VADisplay *dpy;

    drm = open("/dev/dri/card0", 0);
    if (!drm) {
        perror("opening card0");
        return 1;
    }

    dpy = vaGetDisplayDRM(drm);
    if (!dpy) {
        printf("can't get display\n");
        return 1;
    }


    return 0;
}

int32_t import_buf(int fd, int width, int height, int size, int n_planes, uint32_t *strides,
                   uint32_t *offsets, uint32_t fourcc) {
    VAStatus err;
    VAImageFormat format = { 0 };
    VASurfaceAttrib attribs[2];
    VASurfaceAttribExternalBuffers extbuf = { 0 };

    extbuf.pixel_format = fourcc;
    extbuf.width = width;
    extbuf.height = height;
    extbuf.data_size = size;
    extbuf.num_planes = n_planes;
    for (int i = 0; i < n_planes; i++) {
        extbuf.pitches[i] = strides[i];
        extbuf.offsets[i] = offsets[i];
    }
    extbuf.buffers = (uintptr_t *)&fd;
    extbuf.num_buffers = 1;
    extbuf.flags = 0;
    extbuf.private_data = NULL;

    attribs[0].type = VASurfaceAttribExternalBufferDescriptor;
    attribs[0].flags = VA_SURFACE_ATTRIB_SETTABLE;
    attribs[0].value.type = VAGenericValueTypePointer;
    attribs[0].value.value.p = &extbuf;

    attribs[1].type = VASurfaceAttribMemoryType;
    attribs[1].flags = VA_SURFACE_ATTRIB_SETTABLE;
    attribs[1].value.type = VAGenericValueTypeInteger;
    attribs[1].value.value.i = VA_SURFACE_ATTRIB_MEM_TYPE_DRM_PRIME;

    return 0;
}

uint32_t n_planes(uint32_t fourcc) {
    switch (fourcc) {
        case DRM_FORMAT_NV12:
            return 2;
        default:
            printf("unsupported fourcc %d\n", fourcc);
            exit(1);
    }
}
