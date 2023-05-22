#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"
DIR="tests/inputs"

wc $DIR/cant-touch-this > $OUTDIR/cant-touch-this.txt
wc $DIR/empty.txt > $OUTDIR/empty.txt
wc $DIR/one_line_and_new_empty_line.txt > $OUTDIR/one_line_and_new_empty_line.txt
wc $DIR/one_line_and_no_new_line_byte.txt > $OUTDIR/one_line_and_no_new_line_byte.txt
wc $DIR/two_empty_lines.txt > $OUTDIR/two_empty_lines.txt
wc $DIR/two_lines.txt > $OUTDIR/two_lines.txt
wc $DIR/three_lines.txt > $OUTDIR/three_lines.txt