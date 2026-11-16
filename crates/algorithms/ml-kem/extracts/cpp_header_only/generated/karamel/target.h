/* Copyright (c) INRIA and Microsoft Corporation. All rights reserved.
 * Licensed under the Apache 2.0 and MIT Licenses.
 *
 * SPDX-FileCopyrightText: 2024 INRIA and Microsoft Corporation
 * SPDX-FileCopyrightText: 2024 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 */

#ifndef __KRML_TARGET_H
#define __KRML_TARGET_H

#ifndef KRML_HOST_PRINTF
#define KRML_HOST_PRINTF printf
#endif

#if ((defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L) || \
     (defined(__cplusplus) && __cplusplus > 199711L)) &&           \
    (!defined(KRML_HOST_EPRINTF))
#define KRML_HOST_EPRINTF(...) fprintf(stderr, __VA_ARGS__)
#elif !(defined KRML_HOST_EPRINTF) && defined(_MSC_VER)
#define KRML_HOST_EPRINTF(...) fprintf(stderr, __VA_ARGS__)
#endif

#ifndef KRML_HOST_EXIT
#define KRML_HOST_EXIT exit
#endif

// This does not actually force inline.
// Forcing inline increases stack usage beyond acceptable limits
#define KRML_MUSTINLINE inline

#ifndef KRML_NOINLINE
#if defined(_MSC_VER)
#define KRML_NOINLINE __declspec(noinline)
#elif defined(__GNUC__)
#define KRML_NOINLINE __attribute__((noinline, unused))
#else
#define KRML_NOINLINE
#warning "The KRML_NOINLINE macro is not defined for this toolchain!"
#warning "The compiler may defeat side-channel resistance with optimizations."
#warning \
    "Please locate target.h and try to fill it out with a suitable definition for this compiler."
#endif
#endif

#ifndef KRML_ATTRIBUTE_TARGET
#if defined(__GNUC__)
#define KRML_ATTRIBUTE_TARGET(x) __attribute__((target(x)))
#else
#define KRML_ATTRIBUTE_TARGET(x)
#endif
#endif

#endif
