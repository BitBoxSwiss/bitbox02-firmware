/* -------------------------------------------------------------------------------- */
/* -- µGUI - Generic GUI module (C)Achim Döbler, 2015                            -- */
/* -------------------------------------------------------------------------------- */
// µGUI is a generic GUI module for embedded systems.
// This is a free software that is open for education, research and commercial
// developments under license policy of following terms.
//
//  Copyright (C) 2015, Achim Döbler, all rights reserved.
//  URL: http://www.embeddedlightning.com/
//
// * The µGUI module is a free software and there is NO WARRANTY.
// * No restriction on use. You can use, modify and redistribute it for
//   personal, non-profit or commercial products UNDER YOUR RESPONSIBILITY.
// * Redistributions of source code must retain the above copyright notice.
//
/* -------------------------------------------------------------------------------- */
// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef __UGUI_H
#define __UGUI_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

/* -------------------------------------------------------------------------------- */
/* -- TYPEDEFS                                                                   -- */
/* -------------------------------------------------------------------------------- */

typedef int8_t         UG_S8;
typedef int16_t        UG_S16;
typedef int32_t        UG_S32;
typedef uint8_t        UG_U8;
typedef uint16_t       UG_U16;
typedef uint32_t       UG_U32;
typedef UG_U8          UG_COLOR;

/* -------------------------------------------------------------------------------- */
/* -- µGUI FONTS                                                                 -- */
/* -------------------------------------------------------------------------------- */
typedef enum {
    FONT_TYPE_1BPP,
    FONT_TYPE_8BPP
} FONT_TYPE;

typedef struct {
    unsigned char *p;
    FONT_TYPE font_type;
    UG_S16 char_width;
    UG_S16 char_height;
    UG_U16 start_char;
    UG_U16 end_char;
    UG_U8  *widths;
} UG_FONT;

#define UG_FONT_DATA

/* -------------------------------------------------------------------------------- */
/* -- µGUI CORE STRUCTURE                                                        -- */
/* -------------------------------------------------------------------------------- */
typedef struct {
    void (*pset)(UG_S16, UG_S16, UG_COLOR);
    UG_S16 x_dim;
    UG_S16 y_dim;
    UG_FONT font;
    UG_S8 char_h_space;
    UG_S8 char_v_space;
    UG_COLOR fore_color;
    UG_COLOR back_color;
} UG_GUI;

/* -------------------------------------------------------------------------------- */
/* -- COLORS                                                                     -- */
/* -------------------------------------------------------------------------------- */

#define C_BLACK (0x00)
#define C_WHITE (0xFF)

/* -------------------------------------------------------------------------------- */
/* -- µGUI SAFETY LIMITS                                                         -- */
/* -------------------------------------------------------------------------------- */

#define UG_MAX_LINE_COLS 128
#define UG_MAX_LINE_ROWS 16

/* Max allocaiton will be sizeof(char) * UG_MAX_LINE_COLS * UG_MAX_LINE_ROWS */

/* -------------------------------------------------------------------------------- */
/* -- PROTOTYPES                                                                 -- */
/* -------------------------------------------------------------------------------- */

UG_S16 UG_Init( UG_GUI *g, void (*p)(UG_S16, UG_S16, UG_COLOR),
                const UG_FONT *font, UG_S16 x, UG_S16 y );
void UG_FontSelect( const UG_FONT *font );
void UG_FillScreen( UG_COLOR c );
void UG_FillFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c );
void UG_FillRoundFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_S16 r,
                        UG_COLOR c );
void UG_DrawFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c );
void UG_DrawRoundFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_S16 r,
                        UG_COLOR c );
void UG_DrawPixel( UG_S16 x0, UG_S16 y0, UG_COLOR c );
void UG_DrawCircle( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c );
void UG_FillCircle( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c );
void UG_DrawArc( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_U8 s, UG_COLOR c );
void UG_DrawLine( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c );
void UG_WrapTitleString(const char* str, char* str_out, UG_S16 width);
void UG_PutString( UG_S16 x, UG_S16 y, const char *str, bool inverted);
void UG_PutStringNoBreak( UG_S16 x, UG_S16 y, const char *str, bool inverted);
void UG_MeasureString( UG_S16 *xout, UG_S16 *yout, const char *str);
void UG_MeasureStringNoBreak(UG_S16 *xout, UG_S16 *yout, const char *str);
void UG_MeasureStringCentered( UG_S16 *xout, UG_S16 *yout, const char *str);
void UG_PutStringNoBreakCenter( UG_S16 x, UG_S16 y, UG_S16 width, const char *str, bool inverted);
void UG_PutStringCentered( UG_S16 x, UG_S16 y, UG_S16 width, UG_S16 height, const char *str, bool inverted);
void UG_PutChar( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc, bool inverted );
void UG_PutCharTransparent( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, bool inverted );
void UG_SetForecolor( UG_COLOR c );
void UG_SetBackcolor( UG_COLOR c );
UG_S16 UG_GetXDim( void );
UG_S16 UG_GetYDim( void );
void UG_FontSetHSpace( UG_U16 s );
void UG_FontSetVSpace( UG_U16 s );

/* ssd1306.h wrapper */
void UG_SendBuffer(void);
void UG_ClearBuffer(void);

#endif
