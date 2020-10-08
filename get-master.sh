echo -e '\e[92mGetting master alias from shard record'
echo -e '\e[39m'
MASTERALIAS=$(psk peter-demo gcp uswest1 get vts libellis-doggers-puppers-x-x-4bc744fa -o yaml | rg masterAlias | sed 's/masterAlias: //g')
echo -e "\e[92mMaster Alias is:$MASTERALIAS"
echo -e '\e[39m'
