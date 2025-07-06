# CL-Roman Domination Genetic Algorithm (CLRD-GA)

Este projeto implementa um Algoritmo Genético (AG) para resolver o **Problema de Dominação Romana em Grafos** (PDR). O objetivo do PDR é determinar a **função de dominação romana** de menor peso para um grafo, um problema conhecido por ser **NP-completo**.

## 🧪 Sobre o Problema

Dado um grafo $ G = (V, E) $, uma função $ f : V \to \{0, 1, 2\} $ é chamada de **Função de Dominação Romana (FDR)** se, para todo vértice $ v \in V $ tal que $ f(v) = 0 $, existe ao menos um vizinho $ w \in V $ com $ f(w) = 2 $. O **peso** da função $ f $ é dado por:

\[
\omega(f) = \sum_{v \in V} f(v)
\]

O **Número de Dominação Romana** $ \gamma_R(G) $ corresponde ao menor peso entre todas as FDRs possíveis para $ G $.

Este projeto implementa melhorias no algoritmo genético proposto por **Khandelwal et al. (2021)**, alcançando ganhos significativos em eficiência e qualidade das soluções.

## 📊 Resultados Experimentais

O RD-GA foi avaliado em 120 grafos da coleção **Harwell–Boeing**, superando o algoritmo de Khandelwal et al. em **89,2% dos casos**. Esses resultados demonstram a eficácia das melhorias implementadas. Mais detalhes podem ser encontrados no [artigo do projeto](https://drive.google.com/file/d/1IAkI7aDDK9lKjOObrAe4LW6nN-CalVl_/view).

I'll format the instructions in markdown for your README file:

## **Instruções**

1. **Clone o Repositório**  
   Faça o download do código-fonte usando o comando:  
   ```bash
   git clone https://github.com/hscHeric/roman_domination_ga.git
   cd roman_domination_ga
   ```

2. **Compile o Programa**  
   Para criar um executável otimizado, execute o comando:  
   ```bash
   cargo build --release
   ```
   Isso gerará o executável em `./target/release/roman_domination_ga`.

3. **Rodando o Programa**  
   O programa aceita os seguintes argumentos:  
   ```bash
   ./target/release/roman_domination_ga <arquivo_entrada> <execuções> <max_stagnant> <gerações> <tamanho_torneio> <probabilidade_crossover>
   ```
   * **<arquivo_entrada>**: Caminho para o arquivo que contém o grafo.
     * O arquivo deve conter uma lista de arestas, uma por linha no formato `u v`, onde u e v são os vértices.
   * **<execuções>**: Número de execuções do algoritmo para este grafo.
   * **<max_stagnant>**: Número máximo de gerações sem melhora no valor de fitness.
   * **<gerações>**: Número máximo de gerações.
   * **<tamanho_torneio>**: Tamanho do torneio para seleção.
   * **<probabilidade_crossover>**: Probabilidade de crossover (valor entre 0 e 1).

4. **Exemplo de Execução**  
   Para processar o arquivo `./data/edges/graph1.txt` com 30 execuções, 100 gerações sem melhora, limite de 1000 gerações, tamanho de torneio 2 e probabilidade de crossover 0.9, use:  
   ```bash
   ./target/release/roman_domination_ga ./data/edges/graph1.txt 30 100 1000 2 0.9
   ```

5. **Salvando os Resultados**  
   Para salvar os resultados em um arquivo CSV, redirecione a saída:  
   ```bash
   ./target/release/roman_domination_ga ./data/edges/graph1.txt 30 100 1000 2 0.9 >> ./data/results/graph1.csv
   ```
   Os resultados incluirão as métricas de cada execução.
# kambo-hive-worker
# kambo-hive-worker
