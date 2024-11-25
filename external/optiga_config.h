#ifndef _OPTIGA_LIB_CONFIG_BITBOX02_H_
#define _OPTIGA_LIB_CONFIG_BITBOX02_H_

#ifdef __cplusplus
extern "C" {
#endif

    /** @brief OPTIGA CRYPT random number generation feature enable/disable macro */
    #define OPTIGA_CRYPT_RANDOM_ENABLED
    /** @brief OPTIGA CRYPT hash feature enable/disable macro */
    #define OPTIGA_CRYPT_HASH_ENABLED
    /** @brief OPTIGA CRYPT ECC generate keypair feature enable/disable macro */
    #define OPTIGA_CRYPT_ECC_GENERATE_KEYPAIR_ENABLED
    /** @brief OPTIGA CRYPT ECDSA signature feature enable/disable macro */
    #define OPTIGA_CRYPT_ECDSA_SIGN_ENABLED
    /** @brief OPTIGA CRYPT verify ECDSA signature feature enable/disable macro */
    #define OPTIGA_CRYPT_ECDSA_VERIFY_ENABLED
    /** @brief OPTIGA CRYPT ECDH feature enable/disable macro */
    #define OPTIGA_CRYPT_ECDH_ENABLED
    /** @brief OPTIGA CRYPT ECC 521 feature enable/disable macro */
    #define OPTIGA_CRYPT_ECC_NIST_P_521_ENABLED
    /** @brief OPTIGA CRYPT ECC Brainpool feature enable/disable macro */
    #define OPTIGA_CRYPT_ECC_BRAINPOOL_P_R1_ENABLED
    /** @brief OPTIGA CRYPT TLS PRF sha256 feature enable/disable macro */
    #define OPTIGA_CRYPT_TLS_PRF_SHA256_ENABLED
    /** @brief OPTIGA CRYPT TLS PRF sha384 feature enable/disable macro */
    #define OPTIGA_CRYPT_TLS_PRF_SHA384_ENABLED
    /** @brief OPTIGA CRYPT TLS PRF sha512 feature enable/disable macro */
    #define OPTIGA_CRYPT_TLS_PRF_SHA512_ENABLED
    /** @brief OPTIGA CRYPT RSA generate keypair feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_GENERATE_KEYPAIR_ENABLED
    /** @brief OPTIGA CRYPT RSA sign feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_SIGN_ENABLED
    /** @brief OPTIGA CRYPT RSA verify sign feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_VERIFY_ENABLED
    /** @brief OPTIGA CRYPT RSA Encrypt feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_ENCRYPT_ENABLED
    /** @brief OPTIGA CRYPT RSA Decrypt feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_DECRYPT_ENABLED
    /** @brief OPTIGA CRYPT RSA pre-master feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_PRE_MASTER_SECRET_ENABLED
    /** @brief OPTIGA CRYPT RSA SSA with SHA512 as digest feature enable/disable macro */
    #define OPTIGA_CRYPT_RSA_SSA_SHA512_ENABLED
    /** @brief OPTIGA CRYPT symmetric encrypt feature enable/disable macro */
    #define OPTIGA_CRYPT_SYM_ENCRYPT_ENABLED
    /** @brief OPTIGA CRYPT symmetric decrypt feature enable/disable macro */
    #define OPTIGA_CRYPT_SYM_DECRYPT_ENABLED
    /** @brief OPTIGA CRYPT HMAC feature enable/disable macro */
    #define OPTIGA_CRYPT_HMAC_ENABLED
    /** @brief OPTIGA CRYPT HKDF feature enable/disable macro */
    #define OPTIGA_CRYPT_HKDF_ENABLED
    /** @brief OPTIGA CRYPT symmetric generate key feature enable/disable macro */
    #define OPTIGA_CRYPT_SYM_GENERATE_KEY_ENABLED
    /** @brief OPTIGA CRYPT generate auth code feature enable/disable macro */
    #define OPTIGA_CRYPT_GENERATE_AUTH_CODE_ENABLED
    /** @brief OPTIGA CRYPT HMAC verify feature enable/disable macro */
    #define OPTIGA_CRYPT_HMAC_VERIFY_ENABLED
    /** @brief OPTIGA CRYPT clear AUTO state feature enable/disable macro */
    #define OPTIGA_CRYPT_CLEAR_AUTO_STATE_ENABLED

    /** @brief OPTIGA COMMS shielded connection feature.
     *         To disable the feature, undefine the macro
     */
    #define OPTIGA_COMMS_SHIELDED_CONNECTION

    /** @brief Default reset protection level for OPTIGA CRYPT and UTIL APIs */
    #define OPTIGA_COMMS_DEFAULT_PROTECTION_LEVEL           OPTIGA_COMMS_FULL_PROTECTION
    //#define OPTIGA_COMMS_DEFAULT_PROTECTION_LEVEL           OPTIGA_COMMS_NO_PROTECTION

  /** @brief Default reset type in optiga_comms_open.             \n
     *         Cold Reset - (0) : This is applicable if the host platform has GPIO option for RST and VDD.    \n
     *         Soft Reset - (1) : This is applicable if the host platform doesn't have GPIO options for VDD and RST.  \n
     *         Warm Reset - (2) : This is applicable if the host platform doesn't have GPIO option for VDD. \n
     *         Any other value will lead to error
     */
    #define OPTIGA_COMMS_DEFAULT_RESET_TYPE     (1U)

    /** @brief NULL parameter check.
     *         To disable the check, undefine the macro
     */
    #define OPTIGA_LIB_DEBUG_NULL_CHECK
    /** @brief Maximum number of instance registration */
    #define OPTIGA_CMD_MAX_REGISTRATIONS                (0x02)
    /** @brief Maximum buffer size required to communicate with OPTIGA */
    #define OPTIGA_MAX_COMMS_BUFFER_SIZE                (0x615) //1557 in decimal

    /** @brief Macro to enable logger \n
    * Enable macro OPTIGA_LIB_ENABLE_UTIL_LOGGING for Util Service layer logging     \n
    * Enable macro OPTIGA_LIB_ENABLE_CRYPT_LOGGING for Crypt Service layer logging     \n
    * Enable macro OPTIGA_LIB_ENABLE_CMD_LOGGING for Command layer logging     \n
    * Enable macro OPTIGA_LIB_ENABLE_COMMS_LOGGING for Communication layer logging     */
    // #define OPTIGA_LIB_ENABLE_LOGGING
    /** @brief Enable macro OPTIGA_PAL_INIT_ENABLED for calling pal_init functionality */
    //#define OPTIGA_PAL_INIT_ENABLED
/// @cond
#ifdef OPTIGA_LIB_ENABLE_LOGGING
    /** @brief Macro to enable logger for Util service */
    #define OPTIGA_LIB_ENABLE_UTIL_LOGGING
    /** @brief Macro to enable logger for Crypt service */
    #define OPTIGA_LIB_ENABLE_CRYPT_LOGGING
    /** @brief Macro to enable logger for Command layer */
    #define OPTIGA_LIB_ENABLE_CMD_LOGGING
    /** @brief Macro to enable logger for Communication layer */
    #define OPTIGA_LIB_ENABLE_COMMS_LOGGING
#endif
/// @endcond


#ifdef __cplusplus
}
#endif

#endif /* _OPTIGA_LIB_CONFIG_BITBOX02_*/

/**
* @}
*/
