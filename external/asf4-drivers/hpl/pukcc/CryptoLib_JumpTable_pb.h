/**
 * \file
 *
 * \brief Jump table for PUKCL
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

#ifndef _CRYPTOLIB_JUMPTABLE_PB_INCLUDED_
#define _CRYPTOLIB_JUMPTABLE_PB_INCLUDED_

typedef struct _PUKCL_param *PPUKCL_PARAM;
typedef void (*PPUKCL_FUNC)(PPUKCL_PARAM);
/* JumpTable address + 1 as it is thumb code */
#define __vCPKCLCsJumpTableStart 0x02000001
#define __vPUKCLCsGF2NEcRandomiseCoordinate ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x4))
#define __vPUKCLCsRedMod ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x8))
#define __vPUKCLCsCondCopy ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0xc))
#define __vPUKCLCsClearFlags ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x10))
#define __vPUKCLCsGF2NEccDblFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x14))
#define __vPUKCLCsFmult ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x18))
#define __vPUKCLCsGCD ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x1c))
#define __vPUKCLCsGF2NEccMulFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x20))
#define __vPUKCLCsComp ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x24))
#define __vPUKCLCsZpEcDsaGenerateFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x28))
#define __vPUKCLCsZpEcDsaVerifyFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x2c))
#define __vPUKCLCsGF2NEccAddFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x30))
#define __vPUKCLCsZpEccDblFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x34))
#define __vPUKCLCsZpEccAddFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x38))
#define __vPUKCLCsFill ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x3c))
#define __vPUKCLCsZpEccMulFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x40))
#define __vPUKCLCsGF2NEcDsaVerifyFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x44))
#define __vPUKCLCsSmult ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x48))
#define __vPUKCLCsSquare ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x4c))
#define __vPUKCLCsDiv ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x50))
#define __vPUKCLCsSelfTest ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x54))
#define __vPUKCLCsPrimeGen ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x58))
#define __vPUKCLCsCRT ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x5c))
#define __vPUKCLCsFastCopy ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x60))
#define __vPUKCLCsGF2NEcDsaGenerateFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x64))
#define __vPUKCLCsZpEcConvAffineToProjective ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x68))
#define __vPUKCLCsGF2NEcPointIsOnCurve ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x6c))
#define __vPUKCLCsRng ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x70))
#define __vPUKCLCsSwap ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x74))
#define __vPUKCLCsZpEcRandomiseCoordinate ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x78))
#define __vPUKCLCsGF2NEcConvAffineToProjective ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x7c))
#define __vPUKCLCsExpMod ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x80))
#define __vPUKCLCsZpEcConvProjToAffine ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x84))
#define __vPUKCLCsGF2NEcConvProjToAffine ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x88))
#define __vPUKCLCsZpEcPointIsOnCurve ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x8c))
#define __vPUKCLCsZpEccAddSubFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x94))
#define __vPUKCLCsZpEccQuickDualMulFast ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x98))
#define __vPUKCLCsZpEcDsaQuickVerify ((PPUKCL_FUNC)(__vCPKCLCsJumpTableStart + 0x9c))
#endif
