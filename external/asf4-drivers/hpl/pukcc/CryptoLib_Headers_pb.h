/**
 * \file
 *
 * \brief Main header file for PUKCL
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

#ifndef _CryptoLib_HEADERS_PB_INCLUDED
#define _CryptoLib_HEADERS_PB_INCLUDED

/* Include the services headers */
#include "CryptoLib_typedef_pb.h"
#include "CryptoLib_Services_pb.h"
#include "CryptoLib_cf_pb.h"
#include "CryptoLib_Rc_pb.h"
#include "CryptoLib_ClearFlags_pb.h"
#include "CryptoLib_Comp_pb.h"
#include "CryptoLib_CondCopy_pb.h"
#include "CryptoLib_CRT_pb.h"
#include "CryptoLib_Div_pb.h"
#include "CryptoLib_ExpMod_pb.h"
#include "CryptoLib_FastCopy_pb.h"
#include "CryptoLib_Fill_pb.h"
#include "CryptoLib_Fmult_pb.h"
#include "CryptoLib_GCD_pb.h"
#include "CryptoLib_PrimeGen_pb.h"
#include "CryptoLib_RedMod_pb.h"
#include "CryptoLib_Rng_pb.h"
#include "CryptoLib_SelfTest_pb.h"
#include "CryptoLib_Smult_pb.h"
#include "CryptoLib_Square_pb.h"
#include "CryptoLib_Swap_pb.h"
#include "CryptoLib_JumpTable_pb.h"

/* Include headers for ECC */
#include "CryptoLib_ZpEccAdd_pb.h"
#include "CryptoLib_ZpEccAddSub_pb.h"
#include "CryptoLib_ZpEccDbl_pb.h"
#include "CryptoLib_ZpEccMul_pb.h"
#include "CryptoLib_ZpEccQuickDualMul_pb.h"
#include "CryptoLib_ZpEccConv_pb.h"
#include "CryptoLib_ZpEcDsa_pb.h"

#include "CryptoLib_GF2NEccAdd_pb.h"
#include "CryptoLib_GF2NEccDbl_pb.h"
#include "CryptoLib_GF2NEccMul_pb.h"
#include "CryptoLib_GF2NEccConv_pb.h"
#include "CryptoLib_GF2NEcDsa_pb.h"
#include "CryptoLib_Services_pb.h"

typedef struct _PUKCL_status {
	u4 CarryIn : 1;
	u4 CarryOut : 1;
	u4 Zero : 1;
	u4 Gf2n : 1;
	u4 Violation : 1;
	u4 RFU : (32 - 5);
} PUKCL_STATUS, *PPUKCL_STATUS, *PFPUKCL_STATUS;

typedef struct _PUKCL_header {
	u1           u1Service;
	u1           u1SubService;
	u2           u2Option;
	PUKCL_STATUS Specific;
	u2           u2Status;
	u2           __Padding0;
	u4           __Padding1;
} PUKCL_HEADER, *PPUKCL_HEADER, *PFPUKCL_HEADER;

typedef struct _PUKCL_param {
	PUKCL_HEADER PUKCL_Header;
	union {
		_PUKCL_CLEARFLAGS PUKCL_ClearFlags;
		_PUKCL_COMP       PUKCL_Comp;
		_PUKCL_CONDCOPY   PUKCL_CondCopy;
		_PUKCL_CRT        PUKCL_CRT;
		_PUKCL_DIV        PUKCL_Div;
		_PUKCL_EXPMOD     PUKCL_ExpMod;
		_PUKCL_FASTCOPY   PUKCL_FastCopy;
		_PUKCL_FILL       PUKCL_Fill;
		_PUKCL_FMULT      PUKCL_Fmult;
		_PUKCL_GCD        PUKCL_GCD;
		_PUKCL_PRIMEGEN   PUKCL_PrimeGen;
		_PUKCL_REDMOD     PUKCL_RedMod;
		_PUKCL_RNG        PUKCL_Rng;
		_PUKCL_SELFTEST   PUKCL_SelfTest;
		_PUKCL_SMULT      PUKCL_Smult;
		_PUKCL_SQUARE     PUKCL_Square;
		_PUKCL_SWAP       PUKCL_Swap;

		/* ECC on Prime Field */
		_PUKCL_ZPECCADD                   PUKCL_ZpEccAdd;
		_PUKCL_ZPECCDBL                   PUKCL_ZpEccDbl;
		_PUKCL_ZPECCADDSUB                PUKCL_ZpEccAddSub;
		_PUKCL_ZPECCMUL                   PUKCL_ZpEccMul;
		_PUKCL_ZPECDSAGENERATE            PUKCL_ZpEcDsaGenerate;
		_PUKCL_ZPECDSAVERIFY              PUKCL_ZpEcDsaVerify;
		_PUKCL_ZPECDSAQUICKVERIFY         PUKCL_ZpEcDsaQuickVerify;
		_PUKCL_ZPECCQUICKDUALMUL          PUKCL_ZpEccQuickDualMul;
		_PUKCL_ZPECCONVPROJTOAFFINE       PUKCL_ZpEcConvProjToAffine;
		_PUKCL_ZPECCONVAFFINETOPROJECTIVE PUKCL_ZpEcConvAffineToProjective;
		_PUKCL_ZPECRANDOMIZECOORDINATE    PUKCL_ZpEcRandomiseCoordinate;
		_PUKCL_ZPECPOINTISONCURVE         PUKCL_ZpEcPointIsOnCurve;

		/* ECC on Binary Field */
		_PUKCL_GF2NECCADD                   PUKCL_GF2NEccAdd;
		_PUKCL_GF2NECCDBL                   PUKCL_GF2NEccDbl;
		_PUKCL_GF2NECCMUL                   PUKCL_GF2NEccMul;
		_PUKCL_GF2NECDSAGENERATE            PUKCL_GF2NEcDsaGenerate;
		_PUKCL_GF2NECDSAVERIFY              PUKCL_GF2NEcDsaVerify;
		_PUKCL_GF2NECCONVPROJTOAFFINE       PUKCL_GF2NEcConvProjToAffine;
		_PUKCL_GF2NECCONVAFFINETOPROJECTIVE PUKCL_GF2NEcConvAffineToProjective;
		_PUKCL_GF2NECRANDOMIZECOORDINATE    PUKCL_GF2NEcRandomiseCoordinate;
		_PUKCL_GF2NECPOINTISONCURVE         PUKCL_GF2NEcPointIsOnCurve;

	} P;
} PUKCL_PARAM, *PFPUKCL_PARAM;

/* PUKCL helpers */
#define DEF_PARAM pvoid pvPUKCLParam
#define GET_PARAM()
#define USE_PARAM (PPUKCL_PARAM) pvPUKCLParam

#define PUKCL(a) (USE_PARAM)->PUKCL_Header.a

#define PUKCL_ClearFlags(a) (USE_PARAM)->P.PUKCL_ClearFlags.a
#define PUKCL_Comp(a) (USE_PARAM)->P.PUKCL_Comp.a
#define PUKCL_CondCopy(a) (USE_PARAM)->P.PUKCL_CondCopy.a
#define PUKCL_CRT(a) (USE_PARAM)->P.PUKCL_CRT.a
#define PUKCL_Div(a) (USE_PARAM)->P.PUKCL_Div.a
#define PUKCL_ExpMod(a) (USE_PARAM)->P.PUKCL_ExpMod.a
#define PUKCL_FastCopy(a) (USE_PARAM)->P.PUKCL_FastCopy.a
#define PUKCL_Fill(a) (USE_PARAM)->P.PUKCL_Fill.a
#define PUKCL_Fmult(a) (USE_PARAM)->P.PUKCL_Fmult.a
#define PUKCL_GCD(a) (USE_PARAM)->P.PUKCL_GCD.a
#define PUKCL_PrimeGen(a) (USE_PARAM)->P.PUKCL_PrimeGen.a
#define PUKCL_RedMod(a) (USE_PARAM)->P.PUKCL_RedMod.a
#define PUKCL_Rng(a) (USE_PARAM)->P.PUKCL_Rng.a
#define PUKCL_SelfTest(a) (USE_PARAM)->P.PUKCL_SelfTest.a
#define PUKCL_Smult(a) (USE_PARAM)->P.PUKCL_Smult.a
#define PUKCL_Square(a) (USE_PARAM)->P.PUKCL_Square.a
#define PUKCL_Swap(a) (USE_PARAM)->P.PUKCL_Swap.a

#define PUKCL_ZpEccAdd(a) (USE_PARAM)->P.PUKCL_ZpEccAdd.a
#define PUKCL_ZpEccAddSub(a) (USE_PARAM)->P.PUKCL_ZpEccAddSub.a
#define PUKCL_ZpEccDbl(a) (USE_PARAM)->P.PUKCL_ZpEccDbl.a
#define PUKCL_ZpEccMul(a) (USE_PARAM)->P.PUKCL_ZpEccMul.a
#define PUKCL_ZpEcDsaGenerate(a) (USE_PARAM)->P.PUKCL_ZpEcDsaGenerate.a
#define PUKCL_ZpEcDsaVerify(a) (USE_PARAM)->P.PUKCL_ZpEcDsaVerify.a
#define PUKCL_ZpEccQuickDualMul(a) (USE_PARAM)->P.PUKCL_ZpEccQuickDualMul.a
#define PUKCL_ZpEcDsaQuickVerify(a) (USE_PARAM)->P.PUKCL_ZpEcDsaQuickVerify.a
#define PUKCL_ZpEcConvProjToAffine(a) (USE_PARAM)->P.PUKCL_ZpEcConvProjToAffine.a
#define PUKCL_ZpEcConvAffineToProjective(a) (USE_PARAM)->P.PUKCL_ZpEcConvAffineToProjective.a
#define PUKCL_ZpEcRandomiseCoordinate(a) (USE_PARAM)->P.PUKCL_ZpEcRandomiseCoordinate.a
#define PUKCL_ZpEcPointIsOnCurve(a) (USE_PARAM)->P.PUKCL_ZpEcPointIsOnCurve.a

#define PUKCL_GF2NEccAdd(a) (USE_PARAM)->P.PUKCL_GF2NEccAdd.a
#define PUKCL_GF2NEccDbl(a) (USE_PARAM)->P.PUKCL_GF2NEccDbl.a
#define PUKCL_GF2NEccMul(a) (USE_PARAM)->P.PUKCL_GF2NEccMul.a
#define PUKCL_GF2NEcDsaGenerate(a) (USE_PARAM)->P.PUKCL_GF2NEcDsaGenerate.a
#define PUKCL_GF2NEcDsaVerify(a) (USE_PARAM)->P.PUKCL_GF2NEcDsaVerify.a
#define PUKCL_GF2NEcConvProjToAffine(a) (USE_PARAM)->P.PUKCL_GF2NEcConvProjToAffine.a
#define PUKCL_GF2NEcConvAffineToProjective(a) (USE_PARAM)->P.PUKCL_GF2NEcConvAffineToProjective.a
#define PUKCL_GF2NEcRandomiseCoordinate(a) (USE_PARAM)->P.PUKCL_GF2NEcRandomiseCoordinate.a
#define PUKCL_GF2NEcPointIsOnCurve(a) (USE_PARAM)->P.PUKCL_GF2NEcPointIsOnCurve.a

/* Services options helpers */
#define MULTIPLIEROPTION_MASK 0x0003
#define CARRYOPTION_MASK 0x00fc
#define REDUCTIONOPTION_MASK 0xff00

/* Common carry options to all services supporting arithmetic operations
 * These two definitions are internal only
 */
#define FORCE_CARRYIN 0x10
#define FORCE_NOCARRYIN 0x08

/* These definitions are available for final user use */
#define MISC_COMMAND 0x00
#define ADD_CARRY 0x01
#define SUB_CARRY 0x02
#define ADD_1_PLUS_CARRY 0x03
#define ADD_1_MINUS_CARRY 0x04
#define CARRY_NONE ADD_CARRY | FORCE_NOCARRYIN
#define ADD_1 ADD_CARRY | FORCE_CARRYIN
#define SUB_1 SUB_CARRY | FORCE_CARRYIN
#define ADD_2 ADD_1_PLUS_CARRY | FORCE_CARRYIN

/* Common multiplier options to all services supporting arithmetic operations */
#define MULT_ONLY 0x01
#define MULT_ADD 0x02
#define MULT_SUB 0x03

/* Macro enabling to have access to the Carry Options */
#define CARRYOPTION() ((PUKCL(u2Option) & CARRYOPTION_MASK) >> 2)
#define SET_CARRYOPTION(a) (u2)((a) << 2)

/* Macro enabling to have access to the Multiplier Options */
#define MULTIPLIEROPTION() (PUKCL(u2Option) & MULTIPLIEROPTION_MASK)
#define SET_MULTIPLIEROPTION(a) (u2)(a)

/* Macro enabling to have access to the Multiplier Options */
#define REDUCTIONOPTION() ((PUKCL(u2Option) & REDUCTIONOPTION_MASK) >> 8)
#define SET_REDUCTIONOPTION(a) (u2)((a) << 8)

/* Calling a cryptographic service */
#define vPUKCL_Process(a, b)                                                                                           \
	{                                                                                                                  \
		b->PUKCL_Header.u1Service = PUKCL_SERVICE_##a;                                                                 \
		b->PUKCL_Header.u2Status  = PUKCL_COMPUTATION_NOT_STARTED;                                                     \
		__vPUKCLCs##a(b);                                                                                              \
	}

#endif // _CryptoLib_HEADERS_PB_INCLUDED
