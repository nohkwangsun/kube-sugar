#!/bin/bash
# in ks.exe.bash

NS=`ns.get -s`
POD=`ks.get.pod | grep Running | grep $1 | awk '{print $1}' | head -1`

CMD="kubectl -n $NS exec -it $POD /bin/bash ${@:2}"
echo $CMD
exec $CMD
