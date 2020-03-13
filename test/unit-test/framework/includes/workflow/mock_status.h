#ifndef _WORKFLOW_MOCK_STATUS_H
#define _WORKFLOW_MOCK_STATUS_H

#include <workflow/workflow.h>
/**
 * Mock a status workflow: just call the callback and exit immediately.
 */
workflow_t* mock_workflow_status(void (*callback)(void*), void* callback_param);

#endif // _WORKFLOW_MOCK_STATUS_H
