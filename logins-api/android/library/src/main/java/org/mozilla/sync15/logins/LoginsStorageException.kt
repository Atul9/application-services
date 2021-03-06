/* Copyright 2018 Mozilla
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use
 * this file except in compliance with the License. You may obtain a copy of the
 * License at http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed
 * under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
 * CONDITIONS OF ANY KIND, either express or implied. See the License for the
 * specific language governing permissions and limitations under the License. */
package org.mozilla.sync15.logins

// TODO: Get more descriptive errors here.
open class LoginsStorageException(msg: String): Exception(msg)

/** Indicates that the sync authentication is invalid, likely due to having
 * expired.
 */
class SyncAuthInvalidException(msg: String): LoginsStorageException(msg)

// This doesn't really belong in this file...
class MismatchedLockException(msg: String): LoginsStorageException(msg)
