import requests

from requests.auth import HTTPBasicAuth 

def main(db_url: str="http://localhost:8000", sql_file: str="./init.srql"):

	with open(sql_file, "r") as f:
		query = f.read()

	print(query)

	url = db_url + "/sql"
	headers = {
		'Accept': 'application/json',
		'NS': 'global',
		'DB': 'repository' 
	}
	credentials = HTTPBasicAuth("root", "root")
	print(url)	
	q = requests.post(url, data=query, headers=headers, auth=credentials)
	
	if not q.ok:
		print(q.status_code)
		print(q.content)

	print(q.json())

if __name__ == "__main__":
	main()
