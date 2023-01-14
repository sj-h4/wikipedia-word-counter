#!/bin/bash

INPUT=$1
OUTPUT=$2

wikiextractor --json --output $OUTPUT --no-templates $INPUT