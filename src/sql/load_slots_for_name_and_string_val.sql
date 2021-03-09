SELECT id, obj_guid, name, slot_type, int64_val, string_val, double_val, timespec_val,
       guid_val, numeric_val_num, numeric_val_denom, gdate_val
FROM slots
WHERE name = ? AND string_val = ?