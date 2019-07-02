// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef _PASSWORD_H_
#define _PASSWORD_H_

#include <stdbool.h>

/**
 * Starts the set password workflow.
 * @return true if the the two entered passwords match.  Returns false
 * otherwise.
 */
bool password_set(bool (*callback)(const char* password));

/**
 * Promps the user for the password and returns true if the password is the valid keystore password.
 */
bool password_check(void);

#endif
