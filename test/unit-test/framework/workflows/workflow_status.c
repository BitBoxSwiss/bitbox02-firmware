#include <ui/workflow_stack.h>
#include <workflow/status.h>

typedef struct {
    void (*callback)(void*);
    void* callback_param;
} data_t;

void _mock_status_spin(workflow_t* self)
{
    data_t* data = (data_t*)self->data;
    if (data->callback) {
        data->callback(data->callback_param);
    }
    workflow_stack_stop_workflow();
}

workflow_t* mock_workflow_status(void (*callback)(void*), void* callback_param)
{
    workflow_t* result = workflow_allocate(NULL, NULL, _mock_status_spin, sizeof(data_t));
    data_t* data = (data_t*)result->data;
    data->callback = callback;
    data->callback_param = callback_param;
    return result;
}
