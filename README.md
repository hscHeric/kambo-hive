


  <h1>Kambo-Hive</h1>
  <p>
    Kambo-Hive é um sistema distribuído em Rust feita como trabalho final da disciplina de redes de computadores e serve para rodar algoritmos genéticos em paralelo. Ele usa uma arquitetura cliente-servidor e distribui tarefas entre múltiplos workers para acelerar a busca por soluções.
  </p>

  <h2>Estrutura</h2>
  <ul>
    <li><strong>kambo-hive:</strong> biblioteca base e protocolo</li>
    <li><strong>kambo-hive-host:</strong> servidor que distribui tarefas</li>
    <li><strong>kambo-hive-worker:</strong> cliente que executa tarefas</li>
  </ul>

  <h2>Compilação</h2>
  <pre><code>cargo build --release</code></pre>

  <h2>Execução</h2>

  <h3>Host</h3>
  <p>Inicie o host com:</p>
  <pre><code>./target/release/kambo-hive-host 0.0.0.0:12345 ./graphs report.json fifo</code></pre>

  <h3>Worker</h3>
  <p>Conecte o worker com:</p>
  <pre><code>./target/release/kambo-hive-worker 192.168.1.100:12345 ./graphs</code></pre>
  <p>Ou use detecção automática:</p>
  <pre><code>./target/release/kambo-hive-worker --auto ./graphs</code></pre>
