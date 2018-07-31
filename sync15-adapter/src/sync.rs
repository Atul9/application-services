/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use changeset::{OutgoingChangeset, IncomingChangeset, CollectionUpdate};
use util::ServerTimestamp;
use error;
use service::Sync15Service;

/// Low-level store functionality. Stores that need custom reconciliation logic should use this.
///
/// Different stores will produce errors of different types.  To accommodate this, we can either
/// have the store's error type encapsulate errors while syncing, or we can have the Sync 1.5
/// adapter's error type encapsulate the underlying error types.  Right now, it's less clear how to
/// encapsulate errors in a generic way, so we expect `Store` implementations to define an
/// associated `Error` type, and we expect to be able to convert our error type into that type.
pub trait Store {
    type Error;

    fn apply_incoming(
        &mut self,
        inbound: IncomingChangeset
    ) -> Result<OutgoingChangeset, Self::Error>;

    fn sync_finished(
        &mut self,
        new_timestamp: ServerTimestamp,
        records_synced: &[String],
    ) -> Result<(), Self::Error>;
}

pub fn synchronize<E>(svc: &Sync15Service,
                   store: &mut Store<Error=E>,
                   collection: String,
                   timestamp: ServerTimestamp,
                   fully_atomic: bool) -> Result<(), E>
where E: From<error::Error>
{

    info!("Syncing collection {}", collection);
    let incoming_changes = IncomingChangeset::fetch(svc, collection.clone(), timestamp)?;
    let last_changed_remote = incoming_changes.timestamp;

    info!("Downloaded {} remote changes", incoming_changes.changes.len());
    let mut outgoing = store.apply_incoming(incoming_changes)?;

    assert_eq!(outgoing.timestamp, timestamp,
        "last sync timestamp should never change unless we change it");

    outgoing.timestamp = last_changed_remote;

    info!("Uploading {} outgoing changes", outgoing.changes.len());
    let upload_info = CollectionUpdate::new_from_changeset(svc, outgoing, fully_atomic)?.upload()?;

    info!("Upload success ({} records success, {} records failed)",
          upload_info.successful_ids.len(),
          upload_info.failed_ids.len());

    store.sync_finished(upload_info.modified_timestamp, &upload_info.successful_ids)?;

    info!("Sync finished!");
    Ok(())
}
