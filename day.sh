if [ $# -ne 2 ]; then
  echo 1>&2 "Usage: $0 YEAR DAY"
  exit 3
fi

YEAR=$1
NORMALIZED_DAY=$(printf "%02d" $2)

OUT="$YEAR/day$NORMALIZED_DAY"
mkdir -p $OUT/src


sed 's/dayN/day'"$NORMALIZED_DAY"'/g' template/template/Cargo.toml > $OUT/Cargo.toml
sed "s|DAY: usize = 0|DAY: usize = ${DAY}|" template/template/src/main.rs | sed "s|YEAR: usize = 0|YEAR: usize = ${YEAR}|" > $OUT/src/main.rs
echo created $OUT
code -a $OUT
