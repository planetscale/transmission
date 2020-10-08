echo -e '\e[92mGetting master alias from shard record'
echo -e '\e[39m'
MASTERALIAS=$(psk peter-demo gcp uswest1 get vts libellis-doggers-puppers-x-x-4bc744fa -o yaml | rg masterAlias | sed 's/masterAlias: //g')
echo -e "\e[92mMaster Alias is:$MASTERALIAS"
echo -e '\e[92mRetrieving master tablet pod'
echo -e '\e[39m'
MASTERPOD=$(psk peter-demo gcp uswest1 get pods | rg $MASTERALIAS | rg -w '^([a-z0-9\-]+).*?$' -r '$1')
echo -e '\e[92mExec into mysqld container of master tablet pod'
echo -e '\e[39m'
psk peter-demo gcp uswest1 exec -ti $MASTERPOD -c mysqld /bin/bash
