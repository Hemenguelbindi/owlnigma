use rand::Rng;



pub fn print_owl() {
    let owl = r#"
       ___
     .~))>>  
    .~)>>  
  .~))))>>>                 ___
.~~))>>             .       (o,o)
.~~)))>>>)          |      {'   '}  
.~))>>)>>>          |       -"-"-
.~))>>))))>>      _____|__________
~))>>))))>>      '                  `
~)))))>>)>         |   Owlnigma    |
  ~))))>>         \________________/
    "#;

    // ANSI escape codes for colors
    let colors = [
        "\x1b[31m", // red
        "\x1b[32m", // green
        "\x1b[33m", // yellow
        "\x1b[34m", // blue
        "\x1b[35m", // magenta
        "\x1b[36m", // cyan
    ];
    let reset = "\x1b[0m";

    // Генерация случайного индекса для выбора цвета
    let random_color = colors[rand::thread_rng().gen_range(0..colors.len())];

    // Вывод совы с рандомным цветом
    println!("{}{}{}", random_color, owl, reset);
}

