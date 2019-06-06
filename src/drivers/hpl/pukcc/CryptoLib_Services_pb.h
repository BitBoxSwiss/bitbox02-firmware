/**
 * \file
 *
 * \brief PUKCL service definitions
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

#ifndef _CRYPTOLIB_SERVICES_PB_INCLUDED
#define _CRYPTOLIB_SERVICES_PB_INCLUDED

// Services definition
#define PUKCL_SERVICE_RedMod 0x50
#define PUKCL_SERVICE_CondCopy 0x51
#define PUKCL_SERVICE_Div 0x52
#define PUKCL_SERVICE_ZpEcDsaGenerateFast 0x53
#define PUKCL_SERVICE_GF2NEcRandomiseCoordinate 0x54
#define PUKCL_SERVICE_ZpEcDsaVerifyFast 0x55
#define PUKCL_SERVICE_ZpEcConvProjToAffine 0x56
#define PUKCL_SERVICE_GF2NEcPointIsOnCurve 0x57
#define PUKCL_SERVICE_CRT 0x58
#define PUKCL_SERVICE_GF2NEccAddFast 0x59
#define PUKCL_SERVICE_SelfTest 0x5b
#define PUKCL_SERVICE_FastCopy 0x5c
#define PUKCL_SERVICE_GCD 0x5d
#define PUKCL_SERVICE_ZpEcRandomiseCoordinate 0x5e
#define PUKCL_SERVICE_ClearFlags 0x5f
#define PUKCL_SERVICE_ZpEccDblFast 0x60
#define PUKCL_SERVICE_ZpEcConvAffineToProjective 0x61
#define PUKCL_SERVICE_Rng 0x62
#define PUKCL_SERVICE_Swap 0x63
#define PUKCL_SERVICE_GF2NEccMulFast 0x64
#define PUKCL_SERVICE_ZpEccMulFast 0x65
#define PUKCL_SERVICE_ZpEccAddFast 0x66
#define PUKCL_SERVICE_Smult 0x67
#define PUKCL_SERVICE_ZpEcPointIsOnCurve 0x68
#define PUKCL_SERVICE_GF2NEccDblFast 0x69
#define PUKCL_SERVICE_Comp 0x6b
#define PUKCL_SERVICE_ExpMod 0x6c
#define PUKCL_SERVICE_Square 0x6d
#define PUKCL_SERVICE_PrimeGen 0x6e
#define PUKCL_SERVICE_Fill 0x6f
#define PUKCL_SERVICE_GF2NEcDsaGenerateFast 0x70
#define PUKCL_SERVICE_Fmult 0x71
#define PUKCL_SERVICE_GF2NEcConvProjToAffine 0x72
#define PUKCL_SERVICE_GF2NEcConvAffineToProjective 0x73
#define PUKCL_SERVICE_GF2NEcDsaVerifyFast 0x74
#define PUKCL_SERVICE_ZpEccAddSubFast 0x75
#define PUKCL_SERVICE_ZpEccQuickDualMulFast 0x76
#define PUKCL_SERVICE_ZpEcDsaQuickVerify 0x77

extern ServiceFctType __vPUKCLCsExpMod;
extern ServiceFctType __vPUKCLCsPrimeGen;
extern ServiceFctType __vPUKCLCsSquare;
extern ServiceFctType __vPUKCLCsGF2NEcDsaGenerateFast;
extern ServiceFctType __vPUKCLCsComp;
extern ServiceFctType __vPUKCLCsZpEccMulFast;
extern ServiceFctType __vPUKCLCsFmult;
extern ServiceFctType __vPUKCLCsZpEcDsaGenerateFast;
extern ServiceFctType __vPUKCLCsRng;
extern ServiceFctType __vPUKCLCsGF2NEcConvAffineToProjective;
extern ServiceFctType __vPUKCLCsSelfTest;
extern ServiceFctType __vPUKCLCsZpEcDsaVerifyFast;
extern ServiceFctType __vPUKCLCsGF2NEcConvProjToAffine;
extern ServiceFctType __vPUKCLCsGF2NEccMulFast;
extern ServiceFctType __vPUKCLCsGF2NEccAddFast;
extern ServiceFctType __vPUKCLCsGCD;
extern ServiceFctType __vPUKCLCsDiv;
extern ServiceFctType __vPUKCLCsCondCopy;
extern ServiceFctType __vPUKCLCsClearFlags;
extern ServiceFctType __vPUKCLCsRedMod;
extern ServiceFctType __vPUKCLCsZpEccAddFast;
extern ServiceFctType __vPUKCLCsGF2NEccDblFast;
extern ServiceFctType __vPUKCLCsSmult;
extern ServiceFctType __vPUKCLCsFill;
extern ServiceFctType __vPUKCLCsGF2NEcRandomiseCoordinate;
extern ServiceFctType __vPUKCLCsFastCopy;
extern ServiceFctType __vPUKCLCsZpEcPointIsOnCurve;
extern ServiceFctType __vPUKCLCsZpEcConvProjToAffine;
extern ServiceFctType __vPUKCLCsZpEccDblFast;
extern ServiceFctType __vPUKCLCsGF2NEcDsaVerifyFast;
extern ServiceFctType __vPUKCLCsZpEcConvAffineToProjective;
extern ServiceFctType __vPUKCLCsZpEcRandomiseCoordinate;
extern ServiceFctType __vPUKCLCsSwap;
extern ServiceFctType __vPUKCLCsCRT;
extern ServiceFctType __vPUKCLCsGF2NEcPointIsOnCurve;
extern ServiceFctType __vPUKCLCsZpEccQuickDualMulFast;
extern ServiceFctType __vPUKCLCsZpEcDsaQuickVerify;
#endif //_CRYPTOLIB_SERVICES_PB_INCLUDED
