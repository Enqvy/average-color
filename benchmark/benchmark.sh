#!/bin/bash

if [ ! -f "../target/release/average-color" ]; then
    echo "build not found, building..."
    if ! cargo build --release; then
        echo "build failed, cant run app"
        exit 1
    fi
fi

echo "running 20 times..."

times=()

for i in {1..20}; do
    printf "\rrun %d/20" "$i"
    start=$(date +%s%N)
    ../target/release/average-color benchmark.jpg > /dev/null
    end=$(date +%s%N)
    elapsed=$((end - start))
    times+=($elapsed)
    if [ $i -lt 20 ]; then
        sleep 0.3
    fi
done

echo ""

min=${times[0]}
max=${times[0]}
sum=0

for t in "${times[@]}"; do
    sum=$((sum + t))
    if [ $t -lt $min ]; then
        min=$t
    fi
    if [ $t -gt $max ]; then
        max=$t
    fi
done

avg=$((sum / 20))

printf "\nresults:\n"
printf "  min: %d.%04ds\n" $((min / 1000000000)) $(((min % 1000000000) / 100000))
printf "  max: %d.%04ds\n" $((max / 1000000000)) $(((max % 1000000000) / 100000))
printf "  avg: %d.%04ds\n" $((avg / 1000000000)) $(((avg % 1000000000) / 100000))