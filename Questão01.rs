a) --importa std_logic da biblioteca IEEE         
biblioteca IEEE;
use IEEE.std_logic_1164.all;

- Declara uma entidade
entidade ANDGATE é
    porta(
        B: em std_logic;
        X: out std_logic);

fim ANDGATE

arquitetura RTL de ANDGATE é


começar

    X <= não A

    fim RTL;


b) --importa std_logic da biblioteca IEEE
biblioteca IEE;
use IEEE.std_logic_1164.all;

- Declara uma entidade
entidade ANDGATE é
    porta(
        B: em std_logic;
        A: em std_logic;
        X: out std_logic);

fim ANDGATE

arquitetura RTL de ANDGATE é

começar

    Y <= não B

    fim RTL;
© 2021 GitHub, Inc.
Termos
a) --importa std_logic da biblioteca IEEE         
biblioteca IEEE;
use IEEE.std_logic_1164.all;

- Declara uma entidade
entidade ANDGATE é
    porta(
        B: em std_logic;
        X: out std_logic);

fim ANDGATE

arquitetura RTL de ANDGATE é


começar

    X <= não A

    fim RTL;


b) --importa std_logic da biblioteca IEEE
biblioteca IEE;
use IEEE.std_logic_1164.all;

- Declara uma entidade
entidade ANDGATE é
    porta(
        B: em std_logic;
        A: em std_logic;
        X: out std_logic);

fim ANDGATE

arquitetura RTL de ANDGATE é

começar

    Y <= não B

    fim RTL;
