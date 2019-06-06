/**
 * \file
 *
 * \brief Structure definition for PUKCL 'ExpMod' service
 *
 * Copyright (c) 2017-2018 Microchip Technology Inc. and its subsidiaries.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Subject to your compliance with these terms, you may use Microchip
 * software and any derivatives exclusively with Microchip products.
 * It is your responsibility to comply with third party license terms applicable
 * to your use of third party software (including open source software) that
 * may accompany Microchip software.
 *
 * THIS SOFTWARE IS SUPPLIED FOR DEMONSTRATION PURPOSES AS AN EXAMPLE.
 *
 * THIS SOFTWARE IS SUPPLIED BY MICROCHIP "AS IS". THERE IS NO
 * SECURITY BUILT INTO THIS SOFTWARE. NO WARRANTIES,WHETHER
 * EXPRESS, IMPLIED OR STATUTORY, APPLY TO THIS SOFTWARE,
 * INCLUDING ANY IMPLIED WARRANTIES OF NON-INFRINGEMENT, MERCHANTABILITY,
 * AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT WILL MICROCHIP BE
 * LIABLE FOR ANY INDIRECT, SPECIAL, PUNITIVE, INCIDENTAL OR CONSEQUENTIAL
 * LOSS, DAMAGE, COST OR EXPENSE OF ANY KIND WHATSOEVER RELATED TO THE
 * SOFTWARE, HOWEVER CAUSED, EVEN IF MICROCHIP HAS BEEN ADVISED OF THE
 * POSSIBILITY OR THE DAMAGES ARE FORESEEABLE.  TO THE FULLEST EXTENT
 * ALLOWED BY LAW, MICROCHIP'S TOTAL LIABILITY ON ALL CLAIMS IN ANY WAY
 * RELATED TO THIS SOFTWARE WILL NOT EXCEED THE AMOUNT OF FEES, IF ANY,
 * THAT YOU HAVE PAID DIRECTLY TO MICROCHIP FOR THIS SOFTWARE.
 *
 * \asf_license_stop
 *
 */

#ifndef _CRYPTOLIB_EXPMOD_PB_INCLUDED
#define _CRYPTOLIB_EXPMOD_PB_INCLUDED

// Structure definition
typedef struct _PUKCL_expmod {
	nu1  nu1XBase; // (3*u2NLength + 6) words LSW is always zero
	nu1  nu1ModBase;
	nu1  nu1CnsBase;
	nu1  nu1PrecompBase; // xxx words LSW is always zero
	pfu1 pfu1ExpBase;    // u2ExpLength words
	u2   u2ModLength;
	u2   u2ExpLength;
	u1   u1Blinding; // Exponent blinding using a 32-bits Xor
	u1   __Padding0;
	u2   __Padding1;
} _PUKCL_EXPMOD, *_PPKCL_EXPMOD;

// Options definition
#define PUKCL_EXPMOD_REGULARRSA 0x01
#define PUKCL_EXPMOD_EXPINPUKCCRAM 0x02
#define PUKCL_EXPMOD_FASTRSA 0x04
#define PUKCL_EXPMOD_OPERATIONMASK 0x07
#define PUKCL_EXPMOD_MODEMASK 0x05 // For faults protection

#define PUKCL_EXPMOD_WINDOWSIZE_MASK 0x18
#define PUKCL_EXPMOD_WINDOWSIZE_1 0x00
#define PUKCL_EXPMOD_WINDOWSIZE_2 0x08
#define PUKCL_EXPMOD_WINDOWSIZE_3 0x10
#define PUKCL_EXPMOD_WINDOWSIZE_4 0x18
#define PUKCL_EXPMOD_WINDOWSIZE_BIT(a) (u2)((a)&PUKCL_EXPMOD_WINDOWSIZE_MASK) >> 3

#endif // _CRYPTOLIB_EXPMOD_PB_INCLUDED
