#include <mock_workflow.h>

#include <stdlib.h>

#include <hardfault.h>
#include <ui/workflow_stack.h>
#include <workflow/workflow.h>

typedef struct {
    size_t count;
    size_t n_turns;
    bool result;
    void (*callback)(bool, void*);
    void* callback_param;
} unlock_data_t;

static void _unlock_init(workflow_t* self)
{
    unlock_data_t* data = (unlock_data_t*)self->data;
    data->count = 0;
}

static void _unlock_clear(workflow_t* self)
{
    free(self->data);
}

static void _unlock_spin(workflow_t* self)
{
    unlock_data_t* data = (unlock_data_t*)self->data;
    if (data->count == data->n_turns) {
        data->callback(data->result, data->callback_param);
        workflow_stack_stop_workflow();
    } else {
        data->count++;
    }
}

workflow_t* mock_workflow_unlock(int n_turns, bool result, void (*cb)(bool, void*), void* cb_param)
{
    workflow_t* wf =
        workflow_allocate(_unlock_init, _unlock_clear, _unlock_spin, sizeof(unlock_data_t));
    unlock_data_t* data = (unlock_data_t*)wf->data;
    data->n_turns = n_turns;
    data->result = result;
    data->callback = cb;
    data->callback_param = cb_param;
    return wf;
}
