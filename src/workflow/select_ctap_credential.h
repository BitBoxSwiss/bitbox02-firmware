#ifndef _WORKFLOW_SELECT_CTAP_CREDENTIAL_H
#define _WORKFLOW_SELECT_CTAP_CREDENTIAL_H

#include <fido2/ctap.h>

typedef struct {
    uint32_t creation_time;
    char username[CTAP_STORAGE_USER_NAME_LIMIT];
    char display_name[CTAP_STORAGE_DISPLAY_NAME_LIMIT];
} ctap_credential_display_t;

typedef struct {
    size_t n_elems;
    ctap_credential_display_t creds[CTAP_CREDENTIAL_LIST_MAX_SIZE];
} ctap_credential_display_list_t;

/**
 * Starts the select credential workflow.
 * @param[in] credentials List of credentials to display.
 * @return Index of the selected credential within the list, or -1 if aborted.
 */
int workflow_select_ctap_credential(ctap_credential_display_list_t* credentials);

#endif // _WORKFLOW_SELECT_CTAP_CREDENTIAL_H
