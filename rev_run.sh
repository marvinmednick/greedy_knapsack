#!/bin/bash

for f in $*
do
	outputFile=${f/input/output}
	result=`rev_knap.py $f`
	answer=`cat $outputFile`
	if [[  "$result" == "$answer" ]]
	then 
		echo "$f PASSED $result $answer"
	else 
		echo "$f FAILED $result $answer"
	fi
done
