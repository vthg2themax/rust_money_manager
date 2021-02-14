SELECT guid,
    name,
    account_type,
    commodity_guid,
    commodity_scu,
    non_std_scu,
    parent_guid,
    code,
    description,
    hidden,
    placeholder
FROM accounts
WHERE (NOT(name = 'Root Account'))
    AND (NOT(name = 'Template Root'))