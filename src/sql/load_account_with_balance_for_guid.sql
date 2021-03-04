SELECT guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,parent_guid,code,description,hidden,placeholder,
COALESCE((
	SELECT ROUND(SUM(
		splits.value_num / CAST(splits.value_denom AS REAL)
	),8)
	FROM splits
	INNER JOIN transactions AS t ON splits.tx_guid=t.guid WHERE splits.account_guid = accounts.guid
),0) AS balance,
(
	SELECT commodities.mnemonic FROM commodities WHERE commodities.guid=commodity_guid
) AS mnemonic
FROM accounts
WHERE accounts.guid=?