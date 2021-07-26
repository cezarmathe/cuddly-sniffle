# qrwcell - quick read-write cell

This is a cell with two slots - one for reading and one for writing. Writing
alternates the slot that is currently served to readers, thereby minimising
blocking on a reader-writer lock.

Please be aware that if a cell is not created with a value or updated at
least once attempting to get the inner value will loop forever!
