#!/bin/bash
# in ks.get.rs

NS=`ks.get.namespace -s`
CMD="kubectl -n $NS get rs $@"
echo $CMD
exec $CMD
