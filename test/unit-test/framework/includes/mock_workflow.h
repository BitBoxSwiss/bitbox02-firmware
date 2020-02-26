#ifndef _MOCK_WORKFLOW_UNLOCK
#define _MOCK_WORKFLOW_UNLOCK

#include <workflow/workflow.h>

/**
 * Creates a mockup for the unlock workflow, which will pretend to succeed or fail unlocking
 * after N cycles.
 */
workflow_t* mock_workflow_unlock(int n_turns, bool result, void(cb)(bool, void*), void* cb_param);

#endif // _MOCK_WORKFLOW_UNLOCK
