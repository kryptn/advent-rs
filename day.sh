if [ $# -ne 2 ]; then
  echo 1>&2 "Usage: $0 YEAR DAY"
  exit 3
fi

YEAR=$1
DAY=$(printf "%02d" $2)

OUT="$YEAR/day$DAY"
mkdir -p $OUT/src


sed 's/dayN/day'"$DAY"'/g' template/Cargo.toml > $OUT/Cargo.toml
sed 's/DAY/'"$DAY"'/g' template/src/main.rs | sed 's/YEAR/'"$YEAR"'/g' > $OUT/src/main.rs

echo created $OUT
code -a $OUT
