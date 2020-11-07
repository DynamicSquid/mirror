tput reset

#echo "$(wc -l src/*.rs)"

rustc --deny warnings src/main.rs
#rustc -A dead_code src/main.rs

if (( $? == 0 )); then
    ./main

    if (( $? == 0 )); then
        g++ -Werror -o out out.cpp;

        if (( $? == 0 )); then
           ./out; rm out
        else
            tput reset
            echo "[error] - invalid syntax"
        fi
    fi

    rm main
fi