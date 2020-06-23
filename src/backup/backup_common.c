#include "backup_common.h"

#include <util.h>

void backup_cleanup_backup(Backup* backup)
{
    util_zero(backup, sizeof(Backup));
}

void backup_cleanup_backup_data(BackupData* backup_data)
{
    util_zero(backup_data, sizeof(BackupData));
}
