SELECT 
(SELECT a.guid FROM accounts AS a WHERE a.guid=?) AS excluded_account_guid,
(SELECT a.name FROM accounts AS a WHERE a.guid=?) AS excluded_account_name,
(SELECT c.mnemonic FROM commodities AS c WHERE c.guid=
	(SELECT a.commodity_guid FROM accounts AS a WHERE a.guid=?)
)	AS excluded_account_mnemonic,
t.guid,
t.currency_guid,
t.num,
t.post_date,
t.enter_date,
t.description,
s.value_num,
s.value_denom,
(SELECT a.name FROM accounts AS a WHERE a.guid=s.account_guid) AS account_name,
(SELECT a.guid FROM accounts AS a WHERE a.guid=s.account_guid)  AS account_guid
FROM splits AS s
INNER JOIN transactions AS t ON s.tx_guid=t.guid

WHERE s.tx_guid = (
SELECT t.guid 
FROM transactions AS t
WHERE t.description = ? AND
		t.guid in (SELECT splits.tx_guid FROM splits WHERE splits.account_guid = ?)
ORDER BY t.post_date DESC
LIMIT 1
) AND account_guid <> ? 