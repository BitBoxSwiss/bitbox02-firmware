/**
 * \file
 *
 * \brief Structure definition for PUKCL 'GF2NEcDsa' service
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

#ifndef _CRYPTOLIBGF2NECDSA_INCLUDED
#define _CRYPTOLIBGF2NECDSA_INCLUDED
// Structure definition
typedef struct _PUKCL_GF2NEcDsaGenerate {
	nu1 nu1PointABase;
	nu1 nu1OrderPointBase;
	nu1 nu1ModBase;
	nu1 nu1CnsBase;
	nu1 nu1PrivateKey;
	nu1 nu1ScalarNumber;
	nu1 nu1ABase;
	nu1 nu1HashBase;
	nu1 nu1Workspace;
	u2  u2ModLength;
	u2  u2ScalarLength;
	u2  __Padding0;
} _PUKCL_GF2NECDSAGENERATE, *_PPUKCL_GF2NECDSAGENERATE;

typedef struct _PUKCL_GF2NEcDsaVerify {
	nu1 nu1PointABase;
	nu1 nu1OrderPointBase;
	nu1 nu1ModBase;
	nu1 nu1CnsBase;
	nu1 nu1PointPublicKeyGen;
	nu1 nu1PointSignature;
	nu1 nu1ABase;
	nu1 nu1HashBase;
	nu1 nu1Workspace;
	u2  u2ModLength;
	u2  u2ScalarLength;
	u2  __Padding0;
} _PUKCL_GF2NECDSAVERIFY, *_PPUKCL_GF2NECDSAVERIFY;

#endif // _CRYPTOLIBGF2NEcDSA_INCLUDED
