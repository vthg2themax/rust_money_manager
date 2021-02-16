SELECT guid,
    namespace,
    mnemonic,
    fullname,
    cusip,
    fraction,
    quote_flag,
    quote_source,
    quote_tz
FROM commodities
WHERE guid = ?