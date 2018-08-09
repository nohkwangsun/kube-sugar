#!/bin/bash
# in ks.get.rs

NS=`ns.get -s`
CMD="kubectl -n $NS get rs $@"
echo $CMD
exec $CMD
