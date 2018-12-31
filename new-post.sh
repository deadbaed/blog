#!/bin/bash
#

TITLE=$1

if [ "$TITLE" == "" ] ; then
	echo "enter title (with dashes for multiple words)";
	exit 1
fi

cat << EOF > "$(date +%Y-%m-%d) $TITLE.md"
# $TITLE

put text here

-phil
EOF

echo "$(date +%Y-%m-%d) $TITLE.md created!"
echo "time to write!"
