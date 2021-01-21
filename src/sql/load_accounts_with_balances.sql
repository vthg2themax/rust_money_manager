SELECT guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,parent_guid,code,description,hidden,placeholder,
(
	SELECT ROUND(SUM(
		splits.value_num / CAST(splits.value_denom AS REAL)
	),8)
	FROM splits WHERE splits.account_guid = accounts.guid
) AS balance
FROM accounts
WHERE (hidden=0) AND (placeholder=0) AND 
	(NOT(account_type='ROOT')) AND 
	(NOT(account_type='EXPENSE')) AND 
	(NOT(account_type='EQUITY')) AND 
	(NOT(account_type='INCOME')) AND 
	(NOT(name='Expenses'))